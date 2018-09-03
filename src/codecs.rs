use std::net::SocketAddr;

use bytes::BytesMut;
use futures::prelude::*;
use futures::stream;
use futures::sync::mpsc;
use std::fmt::Display;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{error::Error as StdError, io};
use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_io::{codec::LinesCodec, AsyncRead, AsyncWrite};
use std::collections::HashMap;

#[test]
fn test_connect() {
    let address = "127.0.0.1:8000".parse().unwrap();

    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    let listen_address = address;
    thread::spawn(move || {
        let node = Node::new(listen_address);
        node.listen(receiver_rx);
    });

    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    thread::spawn(move || {
        let node2 = Node::new(address);
        node2.connect(&address, receiver_rx);
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

pub struct ConnectStream {
    pub stream: dyn Stream<Item = String, Error = io::Error>,
}

pub struct ConnectSink {
    pub sink: dyn Sink<SinkItem = String, SinkError = io::Error>,
}

pub struct ConnectionPool {
    pub peers: HashMap<SocketAddr, (ConnectSink, ConnectStream)>
}

pub struct Node {
    address: SocketAddr,
}

impl Node {
    pub fn new(address: SocketAddr) -> Self {
        Node {
            address,
        }
    }

    pub fn listen(&self, receiver_rx: mpsc::Receiver<String>) {
        let server = TcpListener::bind(&self.address).unwrap().incoming();

        let fut = server
            .for_each(|incoming_connection| {
                println!("connected from {:?}", incoming_connection);
                let (sink, stream) = incoming_connection.framed(LinesCodec::new()).split();

                let sender = sink.send("line for client".to_string())
                    .into_future()
                    .map(drop)
                    .map_err(|e| println!("error!"));

                let fut = stream
                    .for_each(|line| {
                        println!("Received line {}", line);
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

    pub fn connect(&self, address: &SocketAddr, receiver_rx: mpsc::Receiver<String>) {
        let receiver_fut = self.connect_handle(&address, receiver_rx);

        tokio::run(receiver_fut);
    }

    pub fn connect_handle(
        &self,
        address: &SocketAddr,
        receiver_rx: mpsc::Receiver<String>,
    ) -> Box<dyn Future<Item = (), Error = ()> + Send> {
        let address = address.clone();
        let fut = TcpStream::connect(&address)
            .and_then(move |sock| {
                println!("connected to {:?}", sock);

                let (sink, stream) = sock.framed(LinesCodec::new()).split();

                let fut = stream
                    .for_each(|line| {
                        println!("Received line {}", line);
                        Ok(())
                    })
                    .map_err(|e| println!("e {:?}", e))
                    .into_future()
                    .map(drop);

                tokio::spawn(fut);

                Ok((sink))
            })
            .and_then(|sink| {
                println!("receiver_rx to {:?}", sink);
                let sender = receiver_rx
                    .map_err(|e| other_error("error! "))
                    .forward(sink)
                    .map(drop)
                    .map_err(|e| println!("error!"));

                tokio::spawn(sender);
                Ok(())
            })
            .map_err(|e| println!("error happened {:?}", e));

        Box::new(fut)
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