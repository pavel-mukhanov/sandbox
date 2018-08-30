extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate tokio;
extern crate tokio_io;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use futures::sync::mpsc;
use futures::{Future, Sink};
use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, RwLock};
use std::thread;
use tokio_io::{
    io::{read_exact, write_all}, AsyncRead, AsyncWrite,
};
use codecs::Node;
use std::io::{Read, BufRead};
use codecs::log_error;

mod codecs;


fn main() {

    let address = "127.0.0.1:8000".parse().unwrap();

    let listen_address = address;
    thread::spawn(move || {
        let node = Node::new(listen_address);
        node.listen();
    });

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

