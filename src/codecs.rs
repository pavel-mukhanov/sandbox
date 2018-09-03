use std::net::SocketAddr;

use bytes::BytesMut;
use client_server::Connection;
use client_server::ConnectionPool;
use futures::prelude::*;
use futures::stream::{self, Stream};
use futures::sync::mpsc;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{error::Error as StdError, io};
use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_io::{codec::LinesCodec, AsyncRead, AsyncWrite};
use tokio_retry::{
    strategy::{jitter, FixedInterval},
    Retry,
};

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

    pub fn listen(&self) {
        let server = TcpListener::bind(&self.address).unwrap().incoming();

        let pool = self.connection_pool.clone();

        let fut = server
            .for_each(move |incoming_connection| {
                println!("connected from {:?}", incoming_connection);

                pool.add(Connection::new(
                    &incoming_connection.peer_addr().unwrap(),
                    &incoming_connection.local_addr().unwrap(),
                ));

                let (sink, stream) = incoming_connection.framed(LinesCodec::new()).split();
                let sender = sink
                    .send("line one for client".to_string())
                    .into_future()
                    .map(drop)
                    .map_err(|e| println!("error!"));

                let pool = pool.clone();

                let fut = stream
                    .for_each(move |line| {
                        println!("Received line {}", line);

                        if line == "/pool" {
                            println!("pool {:#?}", pool);
                        }

                        Ok(())
                    })
                    .map_err(|e| println!("e {:?}", e))
                    .into_future()
                    .map(drop);

                tokio::spawn(fut);
                tokio::spawn(sender);
                Ok(())
            })
            .map_err(|e| println!("error happened {:?}", e));

        tokio::run(fut);
    }

    pub fn connect(
        &self,
        address: &SocketAddr,
        remote_sender_tx: mpsc::Sender<String>,
        receiver_rx: mpsc::Receiver<String>,
    ) {
        let address = address.clone();
        let timeout = 5000;
        let max_tries = 5000;
        let strategy = FixedInterval::from_millis(timeout)
            .map(jitter)
            .take(max_tries);

        let action = move || TcpStream::connect(&address);

        let pool = self.connection_pool.clone();
        let sender_tx = remote_sender_tx.clone();

        let future = Retry::spawn(strategy, action)
            .and_then(move |sock| {
                pool.add(Connection::new(
                    &sock.local_addr().unwrap(),
                    &sock.peer_addr().unwrap(),
                ));
                let (sink, stream) = sock.framed(LinesCodec::new()).split();
                println!("connection_pool {:?}", pool);

                let fut = stream
                    .forward(sender_tx.sink_map_err(into_other))
                    .map_err(|e| println!("e {:?}", e))
                    .into_future()
                    .map(drop);

                tokio::spawn(fut);

                Ok((sink))
            })
            .and_then(|sink| {
                println!("receiver_rx to {:?}", sink);
                let sender = receiver_rx
                    .filter(|line| !line.is_empty() )
                    .map_err(|e| other_error("error! "))
                    .forward(sink)
                    .map(drop)
                    .map_err(|e| println!("error!"));

                tokio::spawn(sender);
                Ok(())
            })
            .map_err(|e| println!("error happened {:?}", e));

        tokio::run(future);
    }
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
