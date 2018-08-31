extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate tokio;
extern crate tokio_io;
extern crate clap;

use clap::App;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use codecs::log_error;
use codecs::Node;
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

mod codecs;

fn main() {
    let matches = App::new("simple")
        .args_from_usage("-s --server 'Server mode'")
        .get_matches();

    let address = &"127.0.0.1:8000".parse().unwrap();

    if matches.is_present("server") {
        run_server(address);
    } else {
        run_client(address);
    }
}

fn run_server(address: &SocketAddr) {
    let address = address.clone();
    let node = Node::new(address);
    node.listen();
}

fn run_client(address: &SocketAddr) {
    let address = address.clone();
    let (sender_tx, receiver_rx) = mpsc::channel::<String>(1024);

    thread::spawn(move || {
        let node2 = Node::new(address);
        node2.connect(&address, receiver_rx);
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        sender_tx.clone().send(line.unwrap()).wait();
    }
}
