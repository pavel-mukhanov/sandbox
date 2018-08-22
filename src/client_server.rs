use futures::prelude::*;
use log_error;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock, atomic::AtomicUsize};
use std::thread;
use std::time::Duration;
use tokio;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::{hash::{Hash, Hasher}, sync::atomic::Ordering};
use std::collections::{BTreeMap, hash_map::DefaultHasher};

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
    fn new(local: &SocketAddr, remote: &SocketAddr) -> Self {
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
    let address2 = "127.0.0.1:9000".parse().unwrap();
    let connection_pool = ConnectionPool::new();

    let node1 = Node::new(connection_pool.clone());
    let node2 = Node::new(connection_pool.clone());

    node1.listen(&address1);
    node2.listen(&address2);
    thread::sleep(Duration::from_millis(200));

    node2.connect(&address1);
    node2.connect(&address1);
    node1.connect(&address2);

    println!("pool len {}", connection_pool.size());
    println!("pool {:#?}", connection_pool);
}

struct Node {
    connection_pool: ConnectionPool,
}

impl Node {

    fn new(connection_pool: ConnectionPool) -> Self {
        Node {
            connection_pool
        }
    }

    fn listen(&self, address: &SocketAddr) {
        let address = address.clone();
        let pool = self.connection_pool.clone();

        thread::spawn(move || {
            let server = TcpListener::bind(&address)
                .unwrap()
                .incoming()
                .for_each(move |sock| {
                    println!("received connect from {:?}", sock.peer_addr());
                    pool.add(Connection::new(
                        &sock.peer_addr().unwrap(),
                        &sock.local_addr().unwrap(),
                    ));

                    Ok(())
                })
                .map_err(log_error);

            tokio::run(server);
        });
    }

    fn connect(&self, address: &SocketAddr) {
        let address = address.clone();
        let pool = self.connection_pool.clone();

        let connect = TcpStream::connect(&address)
            .and_then(move |sock| {
                println!("connected to {:?}", sock.peer_addr());

                pool.add(Connection::new(
                    &sock.local_addr().unwrap(),
                    &sock.peer_addr().unwrap(),
                ));
                Ok(())
            })
            .map_err(log_error);

        tokio::run(connect);
    }
}
