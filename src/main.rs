#![feature(arbitrary_self_types, async_await, await_macro, futures_api, pin)]

extern crate byteorder;
extern crate bytes;
extern crate clap;
extern crate futures;
extern crate num;
extern crate partial_io;
extern crate tokio;
extern crate tokio_codec;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use clap::App;
use client_server::ConnectionPool;
use client_server::Node;
use futures::sync::mpsc;
use futures::{Async::Ready, Poll};
use futures::{Future, Sink};
use std::collections::VecDeque;
use std::error::Error as StdError;
use std::io;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use std::thread;

mod bytes_take;
mod client_server;
mod timer;

fn main() {
    let matches = App::new("simple")
        .args_from_usage("-s --server 'Server mode'")
        .get_matches();

    let address1 = "127.0.0.1:8000".parse().unwrap();
    let address2: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let connection_pool = ConnectionPool::new();

    if matches.is_present("server") {
        println!("server");
        let node1 = Node::new(connection_pool.clone());
        node1.listen(&address1);
    } else {
        println!("client");
        let node2 = Node::new(connection_pool.clone());
        node2.connect(&address1);
    }
}

pub fn other_error<S: AsRef<str>>(s: S) -> io::Error {
    io::Error::new(io::ErrorKind::Other, s.as_ref())
}

pub fn log_error<E: StdError>(err: E) {
    eprintln!("An error occurred: {}", err)
}
