use futures::prelude::*;
use futures::stream;
use log_error;
use std::collections::{hash_map::DefaultHasher, BTreeMap};
use std::net::SocketAddr;
use std::sync::{atomic::AtomicUsize, Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::{
    hash::{Hash, Hasher}, sync::atomic::Ordering,
};
use tokio;
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_codec::LinesCodec;

pub struct PublicKey([u8]);

pub struct ConnectInfo {
    /// Peer address.
    pub address: SocketAddr,
    /// Peer public key.
    pub public_key: PublicKey,
}

#[derive(Clone, Debug)]
pub struct ConnectionPool {
    id: Arc<AtomicUsize>,
    connections: Arc<RwLock<BTreeMap<u64, Connection>>>,
}

impl ConnectionPool {
    pub fn new() -> Self {
        ConnectionPool {
            id: Arc::new(AtomicUsize::new(0)),
            connections: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add(&self, connection: Connection) {
        let new_id = self.id.fetch_add(1, Ordering::Relaxed);

        let mut s = DefaultHasher::new();

        connection.hash(&mut s);
        //        let hash = s.finish();
        let hash = new_id as u64;

        println!("hash {:?}", hash);

        let mut connections = self.connections.write().expect("ConnectionPool write lock");
        connections.insert(hash, connection);
    }

    pub fn size(&self) -> usize {
        self.connections.read().unwrap().len()
    }
}

#[derive(Debug, Hash)]
pub struct Connection {
    from: SocketAddr,
    to: SocketAddr,
}

impl Connection {
    pub fn new(local: &SocketAddr, remote: &SocketAddr) -> Self {
        Connection {
            from: *local,
            to: *remote,
        }
    }
}

#[test]
fn test_connect() {
    println!("test connect!");

    let address1 = "127.0.0.1:8000".parse().unwrap();
    let address2: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let connection_pool = ConnectionPool::new();

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
    connection_pool: ConnectionPool,
}

impl Node {
    pub fn new(connection_pool: ConnectionPool) -> Self {
        Node { connection_pool }
    }

    pub fn listen(&self, address: &SocketAddr) {
        let address = address.clone();
        let pool = self.connection_pool.clone();

        let handler = thread::spawn(move || {
            let server = TcpListener::bind(&address)
                .unwrap()
                .incoming()
                .for_each(move |sock| {
                    println!("received connect from {:?}", sock.peer_addr());
                    pool.add(Connection::new(
                        &sock.peer_addr().unwrap(),
                        &sock.local_addr().unwrap(),
                    ));

                    let (writer, reader) = sock.framed(LinesCodec::new()).split();

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

        handler.join();
    }

    pub fn connect(&self, address: &SocketAddr) {
        let address = address.clone();
        let pool = self.connection_pool.clone();

        let connect = TcpStream::connect(&address)
            .and_then(move |sock| {
                println!("connected to {:?}", sock.peer_addr());

                pool.add(Connection::new(
                    &sock.local_addr().unwrap(),
                    &sock.peer_addr().unwrap(),
                ));

                let (writer, reader) = sock.framed(LinesCodec::new()).split();

                let lines = gen_lines(250_000);

                stream::iter(lines).forward(writer).map(drop)
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
