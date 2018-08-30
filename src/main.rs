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

mod byte_stream;
mod codecs;

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

