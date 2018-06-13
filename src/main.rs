extern crate tokio_io;
extern crate byteorder;
extern crate futures;

use std::sync::{Arc, RwLock};
use std::thread;
use std::collections::VecDeque;
use byteorder::LittleEndian;
use byteorder::ByteOrder;
use std::io;
use tokio_io::{codec::length_delimited::*,
               io::{read_exact, write_all},
               AsyncRead,
               AsyncWrite};
use std::sync::Mutex;
use futures::{Sink, Future};
use futures::sync::mpsc;

mod byte_stream;

#[derive(Debug)]
struct Mock {
    data: Arc<RwLock<VecDeque<u8>>>
}

impl Mock {
    pub fn data(&self) -> Arc<RwLock<VecDeque<u8>>> {
        self.data.clone()
    }
}

fn main() {

    let (rx, tx) = mpsc::channel::<u8>(1);

    let remote_rx = rx.clone();
//    let remote_tx = tx.clone();


    let handle = thread::spawn(move || {

        remote_rx.send(1);
//        let mut data = remote.lock().unwrap();
//        read(*data);
//        println!("data {:?}", *data);
    });


    handle.join().unwrap();
//    write(local.data(), &vec![0u8; 4], 4);
}


pub fn read<S: AsyncRead + 'static>(sock: S) -> impl Future<Item = (S, Vec<u8>), Error = io::Error> {
    let buf = vec![0u8; 4];
    read_exact(sock, buf).and_then(|(stream, msg)| {
        read_exact(stream, vec![0u8; msg[0] as usize])
    })
}

fn write<S: AsyncWrite + 'static>(
    sock: S,
    buf: &[u8],
    len: usize,
) -> impl Future<Item = (S, Vec<u8>), Error = io::Error> {
    let mut message = vec![0u8; 4];

    LittleEndian::write_u16(&mut message, len as u16);
    message.extend_from_slice(&buf[0..len]);
    write_all(sock, message)
}
