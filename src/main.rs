#![feature(arbitrary_self_types, async_await, await_macro, futures_api, pin)]

extern crate tokio_io;
extern crate byteorder;
extern crate futures;
extern crate partial_io;
extern crate tokio;
extern crate num;

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
use queue::FixedQueue;
use futures::{Async::Ready, Poll};
use std::{error::Error as StdError};

mod byte_stream;
mod queue;
mod rwlock_test;
mod timer;
mod client_server;

#[derive(Debug)]
struct Mock {
    pub queue: Arc<FixedQueue<Vec<u8>>>
}

impl io::Read for Mock {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {

        let mut data = Vec::new();
        self.queue.dequeue(&mut data);

        println!("dst {:?}", dst);

        println!("data {:?}", data);

        let dst_len = dst.len();
        let (readed, remained) = data.split_at(dst_len);

        dst[..].copy_from_slice(&readed);
        self.queue.enqueue(remained.into());

        Ok(data.len())

    }
}

impl AsyncRead for Mock {}

impl io::Write for Mock {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        println!("src {:?}", src);
        self.queue.enqueue(Vec::from(src));
        Ok(src.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsyncWrite for Mock {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        Ok(Ready(()))
    }
}

fn main() {

    let queue = Arc::new(FixedQueue::new(100));
    let remote = queue.clone();
    let local = queue.clone();


    ///     let reader = Cursor::new(vec![1, 2, 3, 4]);
    ///     let iter = vec![PartialOp::Err(io::ErrorKind::WouldBlock), PartialOp::Limited(2)];
    ///     let partial_reader = PartialAsyncRead::new(reader, iter);
    ///     let out = vec![0; 256];

    let handle = thread::spawn(move || {
        let mock = Mock { queue: remote };


        tokio::run(write(mock, &vec![5u8; 1], 1).map(drop).map_err(log_error));
    });


    handle.join().unwrap();

    let mock = Mock {queue:local};

    tokio::run(read(mock).map(drop).map_err(log_error));

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

pub fn into_other<E: StdError>(err: E) -> io::Error {
    other_error(&format!("An error occurred, {}", err.description()))
}



pub fn other_error<S: AsRef<str>>(s: S) -> io::Error {
    io::Error::new(io::ErrorKind::Other, s.as_ref())
}

pub fn log_error<E: StdError>(err: E) {
    eprintln!("An error occurred: {}", err)
}
