use exonum_proto::ProtobufConvert;
use std::collections::HashMap;

mod proto;

#[derive(Debug, Clone, ProtobufConvert)]
#[protobuf_convert(source = "proto::Point")]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, ProtobufConvert, PartialEq, Eq)]
#[protobuf_convert(source = "proto::BlockHeader")]
struct BlockHeader {
    entries: HashMap<String, Vec<u8>>,
}

#[test]
fn point_pb() {
    let point = Point { x: 1, y: 2 };

    point.to_pb();
}

#[test]
fn test_map_roundtrip() {
    let mut entries = HashMap::new();

    entries.insert("1".to_owned(), vec![1, 2, 3]);
    entries.insert("2".to_owned(), vec![1, 2, 3, 4]);
    entries.insert("3".to_owned(), vec![1, 2, 3, 4, 5]);

    let bh = BlockHeader { entries };

    let proto = bh.to_pb();

    let de_bh = BlockHeader::from_pb(proto).unwrap();

    assert_eq!(bh, de_bh);

    dbg!(bh, de_bh);
}
