use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

use futures::prelude::*;
use futures::stream;
use futures::sync::mpsc;
use tokio;
use tokio::io::{self, AsyncRead};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::sync::{atomic::AtomicUsize, Arc, RwLock};
use tokio::codec::LinesCodec;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct ConnectionPool2 {
    id: Arc<AtomicUsize>,
    pub peers: Arc<RwLock<HashMap<SocketAddr, mpsc::Sender<String>>>>,
}

impl ConnectionPool2 {
    pub fn new() -> Self {
        ConnectionPool2 {
            id: Arc::new(AtomicUsize::new(0)),
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[test]
fn test_connect() {
    println!("test connect!");

    let address1 = "127.0.0.1:8000".parse().unwrap();
    let _address2: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let connection_pool = ConnectionPool2::new();

    let node1 = Node::new(connection_pool.clone());
    let node2 = Node::new(connection_pool.clone());

    node1.listen(&address1);
    node2.connect(&address1);

    /*
    node2.listen(&address2);
    thread::sleep(Duration::from_millis(200));

    node2.connect(&address1);
    node2.connect(&address1);
    node1.connect(&address2); */
}

pub struct Node {
    connection_pool: ConnectionPool2,
}

impl Node {
    pub fn new(connection_pool: ConnectionPool2) -> Self {
        Node { connection_pool }
    }

    #[allow(deprecated)]
    pub fn listen(&self, address: &SocketAddr) {
        let address = address.clone();
        let _pool = self.connection_pool.clone();

        let handler = thread::spawn(move || {
            let server = TcpListener::bind(&address)
                .unwrap()
                .incoming()
                .for_each(move |sock| {
                    println!("received connect from {:?}", sock.peer_addr());

                    let (_writer, reader) = sock.framed(LinesCodec::new()).split();

                    let mut index = 0;

                    let fut = reader
                        .for_each(move |line| {
                            index += 1;

                            if index % 1000 == 0 {
                                println!("received {} lines", index);
                                thread::sleep(Duration::from_millis(50));
                            }

                            println!("line {:?}", line);

                            Ok(())
                        })
                        .and_then(|_| {
                            println!("stream has ended");
                            Ok(())
                        })
                        .map_err(log_error);

                    tokio::spawn(fut);

                    Ok(())
                })
                .map_err(log_error);

            tokio::run(server);
        });

        handler.join().unwrap();
    }

    #[allow(deprecated)]
    pub fn connect(&self, address: &SocketAddr) {
        let address = address.clone();
        let _pool = self.connection_pool.clone();

        let connect = TcpStream::connect(&address)
            .and_then(move |sock| {
                println!("connected to {:?}", sock.peer_addr());

                let (writer, _reader) = sock.framed(LinesCodec::new()).split();

                let lines = gen_lines(250_000);

                stream::iter_ok(lines)
                    .map(|line| line.unwrap_or(String::new()))
                    .forward(writer)
                    .map(drop)
            })
            .map_err(log_error);

        tokio::run(connect);
    }
}

fn gen_lines(n: usize) -> Vec<Result<String, io::Error>> {
    let mut res = vec![];

    for i in 0..n {
        res.push(Ok(format!(
            "line line line line line line line line line line line line {}",
            i
        )));
    }

    res
}

pub fn log_error<E: Display>(error: E) {
    println!("An error occurred: {}", error)
}
