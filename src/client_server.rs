use futures::prelude::*;
use log_error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use tokio;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub struct PublicKey([u8]);

pub struct ConnectInfo {
    /// Peer address.
    pub address: SocketAddr,
    /// Peer public key.
    pub public_key: PublicKey,
}

#[derive(Clone, Debug)]
pub struct ConnectionPool {
    connections: Arc<RwLock<Vec<Connection>>>,
}

impl ConnectionPool {
    pub fn new() -> Self {
        ConnectionPool {
            connections: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add(&self, connection: Connection) {
        let mut connections = self.connections.write().expect("ConnectionPool write lock");
        connections.push(connection);
    }
}

#[derive(Debug)]
pub struct Connection {
    local: SocketAddr,
    remote: SocketAddr,
}

impl Connection {
    fn new(local: &SocketAddr, remote: &SocketAddr) -> Self {
        Connection {
            local: *local,
            remote: *remote,
        }
    }
}

#[test]
fn test_connect() {
    println!("test connect!");

    let address = "127.0.0.1:8000".parse().unwrap();

    let connection_pool = ConnectionPool::new();
    let pool1 = connection_pool.clone();

    thread::spawn(move || {
        let server = TcpListener::bind(&address)
            .unwrap()
            .incoming()
            .for_each(move |sock| {
                println!("received connect from {:?}", sock);
                pool1.add(Connection::new(
                    &sock.local_addr().unwrap(),
                    &sock.peer_addr().unwrap(),
                ));
                println!("pool {:?}", pool1);

                Ok(())
            })
            .map_err(log_error);

        tokio::run(server);
    });

    let local_address = address.clone();
    let pool2 = connection_pool.clone();

    thread::sleep(Duration::from_millis(200));

    let connect = TcpStream::connect(&local_address)
        .and_then(move |sock| {
            println!("connected to {:?}", sock);
            pool2.add(Connection::new(
                &sock.peer_addr().unwrap(),
                &sock.local_addr().unwrap(),
            ));
            Ok(())
        })
        .map_err(log_error);

    tokio::run(connect);

    println!("pool {:?}", connection_pool);
}
