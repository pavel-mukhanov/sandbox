use std::collections::VecDeque;
use futures::{Async::Ready, Poll};
use std::io;
use tokio_io::AsyncRead;
use tokio_io::AsyncWrite;
use queue::FixedQueue;


#[derive(Debug)]
pub struct Mock {
    pub calls: FixedQueue<io::Result<Op>>,
}

#[derive(Debug)]
pub enum Op {
    Data(Vec<u8>),
    Flush,
}

fn would_block() -> io::Error {
    io::Error::new(io::ErrorKind::WouldBlock, "would block")
}

impl io::Read for Mock {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {

        let mut data: Result<Op, io::Error> = Ok(Op::Flush);
        self.calls.dequeue(&mut data);

        Ok(0)

//        match self.calls.pop_front() {
//            Some(Ok(Op::Data(data))) => {
//                let dst_len = dst.len();
//                let (readed, remained) = data.split_at(dst_len);
//
//                dst[..].copy_from_slice(&readed);
//                self.calls.push_front(Ok(remained.into()));
//                Ok(dst_len)
//            }
//            Some(Ok(_)) => panic!(),
//            Some(Err(e)) => Err(e),
//            None => Ok(0),
//        }
    }
}

impl AsyncRead for Mock {}

impl io::Write for Mock {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
//        self.calls.clear();

//        let len = src.len();
//        self.calls.push_front(Ok(src.into()));
//        Ok(len)
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
//        match self.calls.pop_front() {
//            Some(Ok(Op::Flush)) => Ok(()),
//            Some(Ok(_)) => panic!(),
//            Some(Err(e)) => Err(e),
//            None => Ok(()),
//        }
        Ok(())
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
