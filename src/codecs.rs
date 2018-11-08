use std::net::SocketAddr;

use bytes::BytesMut;
use crate::client_server::{Connection, ConnectionPool2};
use futures::future;
use futures::prelude::*;
use futures::stream::{self, Stream};
use futures::sync::mpsc;
use std::borrow::ToOwned;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std::{error::Error as StdError, io};
use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::stream::SplitSink;
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_io::{codec::LinesCodec, AsyncRead, AsyncWrite};
use tokio_retry::{
    strategy::{jitter, FixedInterval}, Retry,
};

use failure;

use std::sync::Mutex;

lazy_static! {
    static ref POOL: Mutex<HashMap<SocketAddr, mpsc::Sender<String>>> = Mutex::new(HashMap::new());
}

type FramedSink = SplitSink<Framed<TcpStream, LinesCodec>>;

#[test]
fn test_receiver() {
    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    sender_tx.send("String".to_string());

    let fut = receiver_rx.for_each(|item| {
        println!("item {}", item);
        Ok(())
    });

    tokio::run(fut);
}

#[derive(Clone)]
pub struct Node {
    pub address: SocketAddr,
    pool: ConnectionPool2,
}

impl Node {
    pub fn new(address: SocketAddr, connection_pool: ConnectionPool2) -> Self {
        Node {
            address,
            pool: connection_pool,
        }
    }

    pub fn listen(
        &self,
        network_tx: mpsc::Sender<String>,
    ) -> impl Future<Item = (), Error = failure::Error> {
        let server = TcpListener::bind(&self.address).unwrap().incoming();
        let pool = self.pool.clone();
        let mut connection_counter = 0;

        let address = self.address.clone();

        let fut = server
            .map_err(into_failure)
            .for_each(move |incoming_connection| {
            println!("connected from {:?}", incoming_connection);

            connection_counter += 1;
            Self::process_connection(
                &address,
                incoming_connection,
                pool.clone(),
                network_tx.clone(),
                true,
            )
        });

        fut
    }

    fn send_message(pool: ConnectionPool2, message: String, address: &SocketAddr) -> impl Future<Item = (), Error = failure::Error> {
        let read_pool = pool.clone();
        let sender_tx = read_pool.peers.read().unwrap();
        let sender = sender_tx.get(&address).unwrap();

        sender
            .clone()
            .send(message.clone())
            .map_err(into_failure)
            .map(drop)
    }

    fn process_connection(
        address: &SocketAddr,
        connection: TcpStream,
        pool: ConnectionPool2,
        network_tx: mpsc::Sender<String>,
        incoming: bool,
    ) -> Result<(), failure::Error> {
        let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

        let _peer_addr = connection.local_addr().unwrap();
        let (sink, stream) = connection.framed(LinesCodec::new()).split();

        let sender = sink.send(address.to_string())
            .map_err(log_error)
            .and_then(|sink| {
                receiver_rx
                    .filter(|line| !line.is_empty())
                    .map_err(|_e| format_err!("error! "))
                    .forward(sink)
                    .map(drop)
                    .map_err(|_e| println!("error!"))
            });

        let fut = stream
            .into_future()
            .map_err(|e| log_error(e.0))
            .and_then(move |(line, stream)| {
                let remote_address: SocketAddr = line.unwrap().parse().unwrap();
                println!("connected from {}, incoming {}", remote_address, incoming);

                pool.add_peer(&remote_address, sender_tx);

                network_tx
                    .sink_map_err(into_failure)
                    .send_all(stream)
                    .map_err(log_error)
                    .into_future()
                    .map(drop)
            })
            .map(drop);

        tokio::spawn(fut);
        tokio::spawn(sender);
        Ok(())
    }

    pub fn request_handler(
        &self,
        receiver: mpsc::Receiver<String>,
        network_tx: mpsc::Sender<String>,
    ) -> impl Future<Item = (), Error = failure::Error> {
        let address = self.address.clone();
        let pool = self.pool.clone();

        let handler = receiver.for_each(move |line| {
            let fut = match line.as_str() {
                "connect" => future::Either::A(Self::connect(
                    pool.clone(),
                    &address,
                    &"127.0.0.1:9000".parse().unwrap(),
                    network_tx.clone(),
                )),
                _ => future::Either::B(Self::send_message(pool.clone(), line, &"127.0.0.1:9000".parse().unwrap())),
            }.map_err(log_error);

            tokio::spawn(fut);
            Ok(())
        });

        handler.map_err(|_e| format_err!(""))
    }

    pub fn ok() -> impl Future<Item = (), Error = failure::Error> {
        future::ok::<(), failure::Error>(())
    }

    pub fn connect(
        pool: ConnectionPool2,
        self_address: &SocketAddr,
        address: &SocketAddr,
        network_tx: mpsc::Sender<String>,
    ) -> impl Future<Item = (), Error = failure::Error> {
        let address = address.clone();
        let self_address = self_address.clone();
        let timeout = 1000;
        let max_tries = 5000;
        let strategy = FixedInterval::from_millis(timeout)
            .map(jitter)
            .take(max_tries);

        let action = move || TcpStream::connect(&address);
        let pool = pool.clone();

        let future = Retry::spawn(strategy, action).map_err(into_failure).and_then(
            move |outgoing_connection| {
                Self::process_connection(
                    &self_address,
                    outgoing_connection,
                    pool,
                    network_tx,
                    false,
                )
            },
        );

        future
    }
}

fn commands_parser(line: String, pool: ConnectionPool2) -> String {
    if line == "/pool" {
        println!("pool {:#?}", pool);
    }
    line
}

struct BadCodecs {}

impl BadCodecs {
    pub fn new() -> Self {
        BadCodecs {}
    }
}

impl Decoder for BadCodecs {
    type Item = String;
    type Error = failure::Error;

    fn decode(
        &mut self,
        _src: &mut BytesMut,
    ) -> Result<Option<<Self as Decoder>::Item>, <Self as Decoder>::Error> {
        Ok(Some("str".to_string()))
    }
}

impl Encoder for BadCodecs {
    type Item = String;
    type Error = io::Error;

    fn encode(
        &mut self,
        _item: <Self as Encoder>::Item,
        _dst: &mut BytesMut,
    ) -> Result<(), <Self as Encoder>::Error> {
        Ok(())
    }
}

pub fn into_failure<E: StdError + Sync + Send + 'static>(error: E) -> failure::Error {
    failure::Error::from_boxed_compat(Box::new(error))
}

pub fn log_error<E: Display>(error: E) {
    println!("An error occurred: {}", error)
}
