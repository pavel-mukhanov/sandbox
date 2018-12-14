use byteorder::LittleEndian;
use bytes::ByteOrder;
use exonum::crypto::{hash, Hash, HashStream, HASH_SIZE};
use test::Bencher;

const LIST_TAG: u8 = 0x3;
const TEST_HASH_SIZE: usize = 1000;
const HASH_PAD: usize = TEST_HASH_SIZE - HASH_SIZE - 9;
const NUM_ITERATIONS: u32 = 10000;

#[bench]
fn list_hash_stream(b: &mut Bencher) {
    b.iter(|| {
        hasher(list_hash_stream_impl);
    })
}

#[bench]
fn list_hash_array(b: &mut Bencher) {
    b.iter(|| {
        hasher(list_hash_array_impl);
    })
}

pub fn hasher<F>(f: F)
where
    F: Fn(u64, Hash, &[u8]) -> Hash,
{
    let padding = vec![1u8; HASH_PAD];

    for i in 0..NUM_ITERATIONS {
        let mut bytes_to_hash = [0; 4];
        LittleEndian::write_u32(&mut bytes_to_hash, i);
        let hash = hash(&bytes_to_hash);
        f(5, hash, &padding);
    }
}

pub fn list_hash_stream_impl(len: u64, root: Hash, padding: &[u8]) -> Hash {
    let mut len_bytes = [0; 8];
    LittleEndian::write_u64(&mut len_bytes, len);

    HashStream::new()
        .update(&[LIST_TAG])
        .update(&len_bytes)
        .update(root.as_ref())
        .update(padding)
        .hash()
}

pub fn list_hash_array_impl(len: u64, root: Hash, padding: &[u8]) -> Hash {
    let mut hash_bytes = [0u8; TEST_HASH_SIZE];

    hash_bytes[0] = LIST_TAG;
    LittleEndian::write_u64(&mut hash_bytes[1..9], len);
    hash_bytes[9..9 + HASH_SIZE].copy_from_slice(root.as_ref());
    hash_bytes[9 + HASH_SIZE..TEST_HASH_SIZE].copy_from_slice(padding);

    hash(&hash_bytes)
}
