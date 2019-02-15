//#![feature(test)]

extern crate base64;
extern crate byteorder;
extern crate bytes;
extern crate clap;
extern crate external;
#[macro_use]
extern crate failure;
extern crate foreign_types_shared;
extern crate futures;
extern crate futures_cpupool;
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate openssl;
//extern crate test;
extern crate tokio;
extern crate tokio_retry;

#[macro_use]
extern crate smallvec;

use std::io;
use std::io::BufRead;
use std::net::SocketAddr;
use std::thread;

use clap::App;
use clap::Arg;
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::{Future, Sink};

use crate::client_server::ConnectionPool2;
use crate::codecs::{log_error, Node};
//use crate::hash_bench::hasher;

mod client_server;
mod codecs;
mod future_send;
//mod hash_bench;
//mod proof;
//mod crypto;
mod traits;
#[macro_use]
mod macros;

pub trait BinaryKey: ToOwned {
    /// Returns the size of the serialized key in bytes.
    fn size(&self) -> usize;

    /// Serializes the key into the specified buffer of bytes.
    ///
    /// The caller must guarantee that the size of the buffer is equal to the precalculated size
    /// of the serialized key. Returns number of written bytes.
    // TODO: Should be unsafe? (ECR-174)
    fn write(&self, buffer: &mut [u8]) -> usize;

    /// Deserializes the key from the specified buffer of bytes.
    // TODO: Should be unsafe? (ECR-174)
    fn read(buffer: &[u8]) -> Self::Owned;
}

impl BinaryKey for Vec<u8> {
    fn size(&self) -> usize {
        self.len()
    }

    fn write(&self, buffer: &mut [u8]) -> usize {
        buffer[..self.size()].copy_from_slice(self);
        self.size()
    }

    fn read(buffer: &[u8]) -> Self {
        buffer.to_vec()
    }
}

fn main() {
    let x = 5;

    let key1 = vec![0; 32];
    let key2 = vec![2; 2];

    let keys = concat_keys!(Vec, key2, key2);

    dbg!(keys);
}

#[test]
fn byteorder() {
    use byteorder::{BigEndian, ByteOrder, LittleEndian};

    let mut buf = vec![0; 4];
    LittleEndian::write_i32(&mut buf, 0x7f);
    dbg!(buf);

    let mut buf = vec![0; 4];
    BigEndian::write_u32(&mut buf, 500);
    dbg!(buf);
}

#[test]
fn overflow() {
    use byteorder::{BigEndian, ByteOrder, LittleEndian};

    let mut buf = vec![0; 4];
    BigEndian::write_u32(&mut buf, 0_i32.wrapping_add(i32::min_value()) as u32);
    dbg!(i32::min_value() as u32);
//    dbg!(buf);

    //BigEndian::$write_method(buffer, self.wrapping_add($itype::min_value()) as $utype);

    let res = BigEndian::read_u32(&buf);
    dbg!(res);
}

enum ObjectType<T> {
    Blob(T)
}

#[test]
fn test_recursion() {
    puzzle(17);
}

fn puzzle(n: u32) -> u32 {
    dbg!(n);

    if (n == 1) {
        return 1
    } else if (n % 2 == 0) {
        return puzzle(n / 2)
    } else {
        return puzzle(3 * n + 1)
    }

}

