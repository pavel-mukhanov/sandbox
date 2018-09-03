extern crate byteorder;
extern crate bytes;
extern crate clap;
extern crate futures;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_retry;

use clap::App;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use clap::Arg;
use client_server::ConnectionPool;
use codecs::log_error;
use codecs::Node;
use futures::stream::{self, Stream};
use futures::sync::mpsc;
use futures::{Future, Sink};
use std::collections::VecDeque;
use std::io;
use std::io::{BufRead, Read};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::thread;
use tokio_io::{
    io::{read_exact, write_all},
    AsyncRead, AsyncWrite,
};

mod client_server;
mod codecs;

fn main() {
    let matches = App::new("simple")
        .arg(Arg::with_name("LISTEN").short("l").takes_value(true))
        .arg(Arg::with_name("REMOTE").short("r").takes_value(true))
        .get_matches();

    let pool = ConnectionPool::new();

    let listen = matches.value_of("LISTEN").unwrap().parse().unwrap();
    let remote = matches.value_of("REMOTE").unwrap().parse().unwrap();

    run_node(listen, remote, pool);
}

fn run_node(listen_address: SocketAddr, remote_address: SocketAddr, pool: ConnectionPool) {
    let listen_address = listen_address.clone();
    let remote_address = remote_address.clone();
    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);
    let (remote_sender_tx, remote_receiver_rx) = mpsc::channel::<String>(1024);

    let node = Node::new(listen_address, pool);
    let connector = node.clone();

    thread::spawn(move || {
        connector.connect(&remote_address, remote_sender_tx, receiver_rx);
    });

    let listener = node.clone();
    thread::spawn(move || {
        listener.listen();
    });

    thread::spawn(move || {
        let fut = remote_receiver_rx.for_each(|line| {
            println!("received line {}", line);
            Ok(())
        });

        tokio::run(fut);
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        sender_tx.clone().send(line.unwrap()).wait();
    }
}
