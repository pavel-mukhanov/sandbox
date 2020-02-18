#![feature(unboxed_closures)]
#![feature(proc_macro_hygiene)]
#![feature(concat_idents)]
#![feature(vec_remove_item)]
#![feature(exclusive_range_pattern)]
#![feature(nll)]
#![feature(generators, generator_trait)]

use failure::_core::marker::PhantomData;
use std::borrow::Borrow;

mod crypto;
mod db;
mod p2p;
mod proc;
mod proto;
mod protos;
mod typemap;

fn main() {
    //    db::list_cf()

    let vec = vec![0, 1, 2, 3, 4, 5];

    let val = vec.iter().skip(20).count();

    dbg!(val);
}

trait BinaryKey {}

struct MapIndex<K: ?Sized> {
    _k: PhantomData<K>,
}

trait BuildProof<K: ?Sized + ToOwned> {
    fn build_proof(&self, key: K::Owned) -> Proof<K::Owned>;
}

impl<T, K> BuildProof<K> for T
where
    K: ToOwned + ?Sized,
    K::Owned: Clone,
    T: MerkleTree<K::Owned>,
{
    fn build_proof(&self, key: K::Owned) -> Proof<K::Owned> {
        Proof {
            key
        }
    }
}

trait MerkleTree<K: ToOwned + ?Sized> {
    fn process_key(&self, key: &K) {

    }
}

impl <K> MerkleTree<K> for MapIndex<K> where K: BinaryKey + ToOwned + ?Sized {

}

struct Proof<K> {
    key: K,
}

impl<K> MapIndex<K>
where
    K: BinaryKey + ToOwned + ?Sized,
{
    fn new() -> Self {
        MapIndex { _k: PhantomData }
    }

    fn proof(&self, key: &K) -> Proof<K::Owned> {
        let proof: Proof<<K as ToOwned>::Owned> = self.build_proof(key);
        proof
//        unimplemented!()
    }
}
