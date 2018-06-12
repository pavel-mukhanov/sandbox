use std::collections::VecDeque;

#[derive(Debug)]
pub struct ByteStream {
    pub buffer: VecDeque<u8>,
}

impl ByteStream {}

#[derive(Debug)]
pub struct Mock {
    pub calls: VecDeque<io::Result<Op>>,
}

#[derive(Debug)]
pub enum Op {
    Data(Vec<u8>),
    Flush,
}

use self::Op::*;
use futures::{Async::Ready, Poll};
use std::io;
use tokio_io::AsyncRead;
use tokio_io::AsyncWrite;

fn would_block() -> io::Error {
    io::Error::new(io::ErrorKind::WouldBlock, "would block")
}

impl io::Read for Mock {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        match self.calls.pop_front() {
            Some(Ok(Op::Data(data))) => {
                println!("data {:?}", data);
                println!("dst.len() {:?}", dst.len());
                let dst_len = dst.len();
                //                debug_assert!(dst.len() >= data.len());
                //                dst[..].copy_from_slice(&data[..dst_len]);

                let (readed, remained) = data.split_at(dst_len);

                dst[..].copy_from_slice(&readed);

                self.calls.push_front(Ok(remained.into()));

                Ok(dst_len)
            }
            Some(Ok(_)) => panic!(),
            Some(Err(e)) => Err(e),
            None => Ok(0),
        }
    }
}

impl AsyncRead for Mock {}

impl io::Write for Mock {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        self.calls.clear();

        let len = src.len();
        self.calls.push_front(Ok(src.into()));
        Ok(len)

        //        match self.calls.pop_front() {
        //            Some(Ok(Op::Data(data))) => {
        //                let len = data.len();
        //                println!("src {:?}", src);
        //                assert!(src.len() >= len, "expect={:?}; actual={:?}", data, src);
        //                assert_eq!(&data[..], &src[..len]);
        //                Ok(len)
        //            }
        //            Some(Ok(_)) => panic!(),
        //            Some(Err(e)) => Err(e),
        //            None => Ok(0),
        //        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.calls.pop_front() {
            Some(Ok(Op::Flush)) => Ok(()),
            Some(Ok(_)) => panic!(),
            Some(Err(e)) => Err(e),
            None => Ok(()),
        }
    }
}

impl AsyncWrite for Mock {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        Ok(Ready(()))
    }
}

impl<'a> From<&'a [u8]> for Op {
    fn from(src: &'a [u8]) -> Op {
        Op::Data(src.into())
    }
}

impl From<Vec<u8>> for Op {
    fn from(src: Vec<u8>) -> Op {
        Op::Data(src)
    }
}
