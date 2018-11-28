#![feature(test)]

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
extern crate exonum;
extern crate num;
extern crate openssl;
extern crate test;
extern crate tokio;
extern crate tokio_retry;

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
use crate::proof::hasher;

mod client_server;
mod codecs;
mod crypto;
mod future_send;
mod proof;

