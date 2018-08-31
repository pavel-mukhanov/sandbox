use bytes::BytesMut;

#[test]
fn test_bytes_take() {
    let mut buf = BytesMut::with_capacity(10);

    buf.extend_from_slice(&vec![0; 10]);

    println!("buf {:?}", buf);

    let res_ = buf.take();

    println!("buf {:?}", buf);
}
