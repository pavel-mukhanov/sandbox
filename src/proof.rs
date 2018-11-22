#![feature(test)]

use exonum::storage::{ProofListIndex, MemoryDB, Database};
use exonum::crypto::{HashStream, Hash, HASH_SIZE, hash};
use byteorder::LittleEndian;
use bytes::ByteOrder;
use test::Bencher;

const LIST_TAG: u8 = 0x3;

#[test]
fn proof_list_index() {

    let db = MemoryDB::new();

    let mut fork = db.fork();

    let mut list: ProofListIndex<_, u32> = ProofListIndex::new("list", &mut fork);

    list.push(1);
    list.push(4398);
    list.push(10);
    list.push(55);
    list.push(123);
    list.push(30);

    let proof =    list.get_proof(5);
    let merkle_root = list.merkle_root();

    assert!(proof.validate(merkle_root, 6).is_ok());

    println!("merkle root {:?}", merkle_root);
    println!("proof {:#?}", proof);
}


#[bench]
fn list_hash_stream(b: &mut Bencher) {
    let hash = hash(b"some data for hash");

    b.iter(|| {
        list_hash_stream_impl(5, hash);
    })
}

#[bench]
fn list_hash_array(b: &mut Bencher) {
    let hash = hash(b"some data for hash");

    b.iter(|| {
        list_hash_array_impl(5, hash);
    })
}

fn list_hash_stream_impl(len:u64, root: Hash) -> Hash {
    let mut len_bytes = [0; 8];
    LittleEndian::write_u64(&mut len_bytes, len);

    HashStream::new()
        .update(&[LIST_TAG])
        .update(&len_bytes)
        .update(root.as_ref())
        .hash()
}

pub fn list_hash_array_impl(len:u64, root: Hash) -> Hash {
    let mut hash_bytes = [0u8; 9 + HASH_SIZE];

    hash_bytes[0] = LIST_TAG;
    LittleEndian::write_u64(&mut hash_bytes[1..9], len);
    hash_bytes[9..9 + HASH_SIZE].copy_from_slice(root.as_ref());

    hash(&hash_bytes)
}
