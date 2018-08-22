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
use futures::{Async::Ready, Async::NotReady, Poll};
use log_error;
use tokio;

#[derive(Debug)]
struct MockStream {
    pos: usize,
    pub queue: Arc<RwLock<VecDeque<Vec<u8>>>>,
}

impl io::Read for MockStream {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        println!("READ...");
        let mut queue = self.queue.read().unwrap();
        let data = queue.back();

        match data {
            Some(data) => {
                println!("data {:?}", data);

                let dst_len = dst.len();
                let (readed, remained) = data.split_at(dst_len);

                dst[..].copy_from_slice(&readed);

                self.pos += dst.len();
                Ok(dst.len())
            }
            _ => Ok(1)
        }
    }
}

impl AsyncRead for MockStream {}

impl io::Write for MockStream {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        println!("WRITE...");
        println!("src {:?}", src);
        let mut queue = self.queue.write().unwrap();
        queue.push_back(Vec::from(src));
        Ok(src.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsyncWrite for MockStream {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        Ok(Ready(()))
    }
}

fn main() {
    let queue = Arc::new(RwLock::new(VecDeque::new()));
    let remote = queue.clone();
    let local = queue.clone();

    let handle = thread::spawn(move || {
        let mock = MockStream { pos: 0, queue: remote };
        tokio::run(write(mock, &vec![5u8; 1], 1).and_then(|(mock, msg)| {
            read(mock)
        }).map(drop).map_err(log_error));
    });


//    handle.join().unwrap();


    let mock = MockStream { pos: 0, queue: local };

    let reader = read(mock).and_then(|(mock, msg)| {
        write(mock, &vec![10u8; 1], 1)
    }).map(drop).map_err(log_error);

    tokio::run(reader);

//    println!("res {:?}", res.unwrap().0);
//    write(local.data(), &vec![0u8; 4], 4);
}


pub fn read<S: AsyncRead + 'static>(sock: S) -> impl Future<Item=(S, Vec<u8>), Error=io::Error> {
    let buf = vec![0u8; 4];
    read_exact(sock, buf).and_then(|(stream, msg)| {
        read_exact(stream, vec![0u8; msg[0] as usize])
    })
}

fn write<S: AsyncWrite + 'static>(
    sock: S,
    buf: &[u8],
    len: usize,
) -> impl Future<Item=(S, Vec<u8>), Error=io::Error> {
    let mut message = vec![0u8; 4];

    LittleEndian::write_u16(&mut message, len as u16);
    message.extend_from_slice(&buf[0..len]);
    write_all(sock, message)
}

#[test]
fn rwlock_test() {
    main();
}