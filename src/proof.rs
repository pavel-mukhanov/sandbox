
use exonum::crypto::{HashStream, Hash, HASH_SIZE, hash};
use byteorder::LittleEndian;
use bytes::ByteOrder;
use test::Bencher;

const LIST_TAG: u8 = 0x3;

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

fn hasher<F>(f: F)
    where F:Fn(u64, Hash) -> Hash  {
    for i in 0..10_000 {
        let mut bytes_to_hash = [0; 4];
        LittleEndian::write_u32(&mut bytes_to_hash, i);
        let hash = hash(&bytes_to_hash);
        f(5, hash);
    }
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
