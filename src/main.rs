#![feature(unboxed_closures)]
#![feature(proc_macro_hygiene)]
#![feature(concat_idents)]
#![feature(vec_remove_item)]
#![feature(exclusive_range_pattern)]
#![feature(nll)]
#![feature(generators, generator_trait)]

use crate::db::list_cf;

mod crypto;
mod proc;
mod proto;
mod p2p;
mod protos;
mod db;

fn main() {
   list_cf();
}
