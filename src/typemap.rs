use failure::_core::marker::PhantomData;
use nom::lib::std::collections::HashMap;
use std::any::TypeId;
use std::hash::Hash;
use typemap::{Key, TypeMap};

#[derive(PartialEq, Eq, Hash)]
struct SomeDataKey;

#[derive(PartialEq, Eq, Hash)]
struct AnotherDataKey;

struct BinaryTypedMap {
    inner: HashMap<TypeId, Vec<u8>>,
}

trait BinaryMapKey: Eq + Hash {
    type Value;
}

trait BinaryValue: Sized {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SomeData(String);

#[derive(Debug, Clone, PartialEq, Eq)]
struct AnotherData(String);

impl BinaryValue for SomeData {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ()> {
        let data = String::from_utf8(bytes).map_err(|e| ())?;
        Ok(SomeData(data))
    }
}

impl BinaryValue for AnotherData {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ()> {
        let data = String::from_utf8(bytes).map_err(|e| ())?;
        Ok(AnotherData(data))
    }
}

impl BinaryMapKey for SomeDataKey {
    type Value = SomeData;
}

impl BinaryMapKey for AnotherDataKey {
    type Value = AnotherData;
}

impl BinaryTypedMap {
    fn new() -> Self {
        BinaryTypedMap {
            inner: HashMap::default(),
        }
    }

    fn insert<K: BinaryMapKey + 'static>(&mut self, value: K::Value)
    where
        K::Value: BinaryValue,
    {
        self.inner.insert(TypeId::of::<K>(), value.to_bytes());
    }

    fn get<K: BinaryMapKey + 'static>(&self) -> Option<K::Value>
    where
        K::Value: BinaryValue,
    {
        let value = self.inner.get(&TypeId::of::<K>())?;
        BinaryValue::from_bytes(value.clone()).ok()
    }
}

#[test]
fn binary_typed_map() {
    let mut map = BinaryTypedMap::new();

    let some_data = SomeData("data".to_owned());
    map.insert::<SomeDataKey>(some_data.clone());

    let some_data_2 = map.get::<SomeDataKey>();
    assert_eq!(some_data, some_data_2.unwrap());

    let another_data = AnotherData("another_data".to_owned());
    map.insert::<AnotherDataKey>(another_data.clone());

    let another_data_2 = map.get::<AnotherDataKey>();
    assert_eq!(another_data, another_data_2.unwrap());
}
