use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Mutex;

use failure;
use futures::future;
use futures::prelude::*;
use futures::stream::Stream;
use futures::sync::mpsc;
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};
use tokio_retry::{
    strategy::{jitter, FixedInterval},
    Retry,
};

use crate::client_server::ConnectionPool2;
use tokio::codec::LinesCodec;

lazy_static! {
    static ref POOL: Mutex<HashMap<SocketAddr, mpsc::Sender<String>>> = Mutex::new(HashMap::new());
}

#[test]
fn test_receiver() {
    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    sender_tx.send("String".to_string()).wait().unwrap();

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

    fn send_message(
        pool: ConnectionPool2,
        message: String,
        address: &SocketAddr,
    ) -> impl Future<Item = (), Error = failure::Error> {
        let read_pool = pool.clone();
        let sender_tx = read_pool.peers.read().unwrap();
        let sender = sender_tx.get(&address).unwrap();

        sender
            .clone()
            .send(message.clone())
            .map_err(into_failure)
            .map(drop)
    }

    #[allow(deprecated)]
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

        let sender = sink
            .send(address.to_string())
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
            }).map(drop);

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
                _ => future::Either::B(Self::send_message(
                    pool.clone(),
                    line,
                    &"127.0.0.1:9000".parse().unwrap(),
                )),
            }.map_err(log_error);

            tokio::spawn(fut);
            Ok(())
        });

        handler.map_err(|_e| format_err!(""))
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

        let future = Retry::spawn(strategy, action)
            .map_err(into_failure)
            .and_then(move |outgoing_connection| {
                Self::process_connection(
                    &self_address,
                    outgoing_connection,
                    pool,
                    network_tx,
                    false,
                )
            });

        future
    }
}

pub fn into_failure<E: StdError + Sync + Send + 'static>(error: E) -> failure::Error {
    failure::Error::from_boxed_compat(Box::new(error))
}

pub fn log_error<E: Display>(error: E) {
    println!("An error occurred: {}", error)
}
