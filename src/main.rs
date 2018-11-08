extern crate byteorder;
extern crate bytes;
extern crate clap;
extern crate futures;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_retry;
extern crate futures_cpupool;
extern crate external;
extern crate num;
extern crate openssl;
extern crate foreign_types_shared;
extern crate base64;
extern crate hex;
extern crate sodiumoxide;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

use clap::App;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use clap::Arg;
use crate::codecs::{log_error, Node};
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
    io::{read_exact, write_all}, AsyncRead, AsyncWrite,
};
use crate::client_server::ConnectionPool2;

mod client_server;
mod codecs;
mod future_send;
mod crypto;

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
        connect_sender_tx.clone().send(line.clone()).wait();
    }
}
