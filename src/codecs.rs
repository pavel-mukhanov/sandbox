use std::net::SocketAddr;

use bytes::BytesMut;
use client_server::Connection;
use client_server::ConnectionPool;
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

use std::sync::Mutex;

lazy_static! {
    static ref POOL: Mutex<HashMap<SocketAddr, mpsc::Sender<String>>> = Mutex::new(HashMap::new());
}

type FramedSink = SplitSink<Framed<TcpStream, LinesCodec>>;

#[test]
fn test_connect() {
    let address = "127.0.0.1:8000".parse().unwrap();
    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    let listen_address = address;

    let pool = ConnectionPool::new();

    let remote_pool = pool.clone();
    thread::spawn(move || {
        let node = Node::new(listen_address, remote_pool);
        node.listen();
    });

    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    let local_pool = pool.clone();
    thread::spawn(move || {
        let node2 = Node::new(address, local_pool);
        node2.connect(&address, rsender_tx, eceiver_rx);
    });

    sender_tx.send("item".to_string()).wait();
    thread::sleep(Duration::from_millis(500));
}

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
    address: SocketAddr,
    connection_pool: ConnectionPool,
}

impl Node {
    pub fn new(address: SocketAddr, connection_pool: ConnectionPool) -> Self {
        Node {
            address,
            connection_pool,
        }
    }

    pub fn listen(&self, receiver_rx: mpsc::Receiver<String>) {
        let server = TcpListener::bind(&self.address).unwrap().incoming();

        let pool = self.connection_pool.clone();

        let mut connection_counter = 0;

        let fut = server
            .for_each(move |incoming_connection| {
                println!("connected from {:?}", incoming_connection);

                connection_counter += 1;

                Self::process_connection(incoming_connection)
            })
            .map_err(|e| println!("error happened {:?}", e));

        Self::process_pool(receiver_rx);

        tokio::run(fut);
    }

    fn process_pool(receiver_rx: mpsc::Receiver<String>) {
        thread::spawn(move || {
            let mut pool_len = 0;
            let sender = receiver_rx.for_each(move |message| {
                let mut write_pool = POOL.lock().unwrap();
                let sender_tx: Vec<mpsc::Sender<String>> = write_pool.values().cloned().collect();

                sender_tx.iter().for_each(move |sen| {
                    let fut = sen.clone()
                        .send(message.clone())
                        .map(drop)
                        .map_err(|e| {

                            log_error(e);
                        });
                    tokio::spawn(fut);
                });

                Ok(())
            });
            tokio::run(sender);
        });
    }

    fn process_connection(connection: TcpStream) -> Result<(), io::Error> {
        let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);
        //
        //        pool.add(Connection::new(
        //            &incoming_connection.peer_addr().unwrap(),
        //            &incoming_connection.local_addr().unwrap(),
        //        ));

        let peer_addr = connection.peer_addr().unwrap();
        let (sink, stream) = connection.framed(LinesCodec::new()).split();

        let sender = receiver_rx
            .filter(|line| !line.is_empty())
            .map_err(|e| other_error("error! "))
            .forward(sink)
            .map(drop)
            .map_err(|e| println!("error!"));

        let mut pool = POOL.lock().unwrap();
        pool.insert(peer_addr, sender_tx);

        let fut = stream
            .for_each(move |line| {
                println!("Received line {}", line);

                Ok(())
            })
            .map_err(|e| println!("e {:?}", e))
            .into_future()
            .map(drop);

        tokio::spawn(fut);
        tokio::spawn(sender);
        Ok(())
    }

    pub fn connect(
        &self,
        address: &SocketAddr,
        remote_sender_tx: mpsc::Sender<String>,
        receiver_rx: mpsc::Receiver<String>,
    ) {
        let address = address.clone();
        let timeout = 1000;
        let max_tries = 5000;
        let strategy = FixedInterval::from_millis(timeout)
            .map(jitter)
            .take(max_tries);

        let action = move || TcpStream::connect(&address);

        let pool = self.connection_pool.clone();
        let sender_tx = remote_sender_tx.clone();

        let future = Retry::spawn(strategy, action)
            .map_err(into_other)
            .and_then(move |outgoing_connection| {
                pool.add(Connection::new(
                    &outgoing_connection.local_addr().unwrap(),
                    &outgoing_connection.peer_addr().unwrap(),
                ));
                Self::process_connection(outgoing_connection)
            })
            .map_err(|e| println!("error happened {:?}", e));

        Self::process_pool(receiver_rx);

        tokio::run(future);
    }
}

fn commands_parser(line: String, pool: ConnectionPool) -> String {
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
    type Error = io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<<Self as Decoder>::Item>, <Self as Decoder>::Error> {
        Ok(Some("str".to_string()))
    }
}

impl Encoder for BadCodecs {
    type Item = String;
    type Error = io::Error;

    fn encode(
        &mut self,
        item: <Self as Encoder>::Item,
        dst: &mut BytesMut,
    ) -> Result<(), <Self as Encoder>::Error> {
        Ok(())
    }
}

pub fn other_error<S: AsRef<str>>(s: S) -> io::Error {
    io::Error::new(io::ErrorKind::Other, s.as_ref())
}

pub fn into_other<E: StdError>(err: E) -> io::Error {
    other_error(&format!("An error occurred, {}", err.description()))
}

pub fn log_error<E: Display>(error: E) {
    println!("An error occurred: {}", error)
}
