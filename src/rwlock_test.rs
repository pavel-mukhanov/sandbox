use std::sync::{Arc, RwLock};
use std::thread;
use std::collections::VecDeque;
use byteorder::LittleEndian;
use byteorder::ByteOrder;
use futures::Future;
use std::io;
use tokio_io::{codec::length_delimited::*,
               io::{read_exact, write_all},
               AsyncRead,
               AsyncWrite};

struct Mock {
    data: Arc<RwLock<VecDeque<u8>>>
}

impl Mock {
    pub fn data(&self) -> Arc<RwLock<VecDeque<u8>>> {
        self.data.clone()
    }
}

fn main() {
    let mut mock = Mock { data: Arc::new(RwLock::new(VecDeque::new()))};

    let arc = Arc::new(mock);
    let local = arc.clone();
    let remote = arc.clone();
    let handle = thread::spawn(move || {
        let data = remote.data();
        read(data)
    });

    write(local.data(), &vec![0u8; 4], 4);
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

#[test]
fn test_rwlock() {
    main()
}