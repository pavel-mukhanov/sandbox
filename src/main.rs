#![feature(arbitrary_self_types, async_await, await_macro, futures_api, pin)]

extern crate byteorder;
extern crate futures;
extern crate num;
extern crate partial_io;
extern crate tokio;
extern crate tokio_io;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use futures::sync::mpsc;
use futures::{Async::Ready, Poll};
use futures::{Future, Sink};
use std::collections::VecDeque;
use std::error::Error as StdError;
use std::io;
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use std::thread;
use tokio_io::{
    codec::length_delimited::*, io::{read_exact, write_all}, AsyncRead, AsyncWrite,
};

mod client_server;
mod rwlock_test;
mod timer;

fn main() {}

pub fn read<S: AsyncRead + 'static>(
    sock: S,
) -> impl Future<Item = (S, Vec<u8>), Error = io::Error> {
    let buf = vec![0u8; 4];
    read_exact(sock, buf).and_then(|(stream, msg)| read_exact(stream, vec![0u8; msg[0] as usize]))
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
