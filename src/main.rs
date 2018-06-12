#[macro_use]
extern crate log;
extern crate byteorder;
extern crate env_logger;
extern crate futures;
extern crate partial_io;
extern crate tokio_core;
extern crate tokio_io;

use byteorder::{ByteOrder, LittleEndian};
use futures::future::Future;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio_io::{codec::length_delimited::*,
               io::{read_exact, write_all},
               AsyncRead,
               AsyncWrite};

mod byte_stream;
mod lifetimes;

use byte_stream::ByteStream;

use byte_stream::Mock;
use partial_io::{PartialAsyncRead, PartialOp};
use tokio_core::reactor::Core;

fn main() {
    let mut data = VecDeque::new();

    //    data.push_front(Ok(vec![0u8; 4].into()));

    let mock = Mock { calls: data };

    let remote_data = Arc::new(mock);
    //    let local_data = remote_data.clone();

    //    thread::spawn(move || {

    //        let mut core = Core::new().unwrap();
    //        let mock = Arc::try_unwrap(remote_data).unwrap();
    //        let read_fut = read(mock);
    //        let res = core.run(read_fut).unwrap();

    //    });

    let mut core = Core::new().unwrap();

    let mock = Arc::try_unwrap(remote_data).unwrap();

    let write_fut = write(mock, &vec![1u8; 4], 4).and_then(|(sock, msg)| read(sock));

    let res = core.run(write_fut).unwrap();

    println!("res {:?} ", res);
}

fn read<S: AsyncRead + 'static>(sock: S) -> impl Future<Item = (S, Vec<u8>), Error = io::Error> {
    let buf = vec![0u8; 4];
    read_exact(sock, buf).and_then(|(stream, msg)| {
        println!("msg {:?} ", msg);
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
