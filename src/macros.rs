macro_rules! concat_keys {
    (@capacity $key:expr) => ( $key.size() );
    (@capacity $key:expr, $($tail:expr),+) => (
        $key.size() + concat_keys!(@capacity $($tail),+)
    );
    ($type:ident, $($key:expr),+) => ({
        let capacity = concat_keys!(@capacity $($key),+);
        let mut buf = $type::with_capacity(capacity);

        // Unsafe `set_len` here is safe because we never read from `buf`
        // before we write all elements to it.
        #[allow(unsafe_code)]
        unsafe {
            buf.set_len(capacity);
        }

        let mut _pos = 0;
        $(
            _pos += $key.write(&mut buf[_pos.._pos + $key.size()]);
        )*
        buf
    });
    ($($key:expr),+) => ({
        let mut buf = Vec::with_capacity(capacity);
        buf
    });
}


macro_rules! foo {
    ($type:expr, $x:expr) => (
        println!("mode X: {}", $type)
    );
}



#[test]
fn foo_macros() {
    foo!(2 + 3, 3);
}

#[test]
fn concat_keys() {
    use crate::BinaryKey;
    use smallvec::SmallVec;

    let key1 = vec![1,2,3];
    let key2 = vec![3,2,1];

    let res = concat_keys!(Vec, key1, key2);
    let res_sv: SmallVec<[u8; 6]>  = concat_keys!(SmallVec, key1, key2);

    let res_sv = res_sv.as_ref();
    let res = res.as_slice();
    assert!(res_sv == res);
}

#[test]
fn concat_keys_default() {
    use crate::BinaryKey;
    use smallvec::SmallVec;

    let key1 = vec![1,2,3];
    let key2 = vec![3,2,1];

//    let res = concat_keys!(key1, key2);

//    dbg!(res);
}