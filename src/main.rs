#![feature(test)]

extern crate base64;
extern crate byteorder;
extern crate bytes;
extern crate clap;
extern crate external;
#[macro_use]
extern crate failure;
extern crate foreign_types_shared;
extern crate futures;
extern crate futures_cpupool;
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate openssl;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_retry;
extern crate exonum;
extern crate test;

use std::io;
use std::io::BufRead;
use std::net::SocketAddr;
use std::thread;

use clap::App;
use clap::Arg;
use futures::{Future, Sink};
use futures::stream::Stream;
use futures::sync::mpsc;

use crate::client_server::ConnectionPool2;
use crate::codecs::{log_error, Node};

mod client_server;
mod codecs;
mod future_send;
mod crypto;
mod proof;

fn main() {
    let matches = App::new("simple")
        .arg(Arg::with_name("LISTEN").short("l").takes_value(true))
        .arg(Arg::with_name("REMOTE").short("r").takes_value(true))
        .get_matches();

    let pool = ConnectionPool2::new();

    let listen = matches.value_of("LISTEN").unwrap().parse().unwrap();
    let remote = matches.value_of("REMOTE").unwrap().parse().unwrap();

    run_node(listen, remote, pool);
}

fn run_node(listen_address: SocketAddr, remote_address: SocketAddr, pool: ConnectionPool2) {
    let listen_address = listen_address.clone();
    let _remote_address = remote_address.clone();
    let (connect_sender_tx, connect_receiver_rx) = mpsc::channel::<String>(1024);
    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    let node = Node::new(listen_address, pool.clone());
    let _connector = node.clone();

    let _remote_sender = sender_tx.clone();

    let listener = node.clone();

    let server = listener.listen(sender_tx.clone());
    let handler = node.request_handler(connect_receiver_rx, sender_tx);
    thread::spawn(|| tokio::run(server.join(handler).map_err(log_error).map(drop)));

    thread::spawn(move || {
        let receiver = receiver_rx.for_each(|line| {
            println!("> {}", line);
            Ok(())
        });
        tokio::run(receiver);
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        connect_sender_tx.clone().send(line.clone()).wait().unwrap();
    }
}
