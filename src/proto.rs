use crate::protos::proof::{MapProof, OptionalEntry};
use failure::Error;
use protobuf::reflect::MessageDescriptor;
use protobuf::well_known_types::{Int32Value, StringValue};
use protobuf::{
    parse_from_bytes, CodedInputStream, CodedOutputStream, Message, ProtobufError, RepeatedField,
    UnknownFields,
};
use std::any::Any;

mod db {

    #[derive(Debug, Eq, PartialEq)]
    pub struct OptionalEntry<V> {
        pub value: V,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct MapProof<V> {
        pub entries: Vec<OptionalEntry<V>>,
    }

    impl<V> MapProof<V> {
        pub fn entries(&self) -> &[OptionalEntry<V>] {
            &self.entries
        }
    }
}

trait BinaryValue {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait ProtobufConvert: Sized {
    /// Type of the protobuf clone of Self
    type ProtoStruct: Message;

    /// Struct -> ProtoStruct
    fn to_pb(&self) -> Self::ProtoStruct;

    /// ProtoStruct -> Struct
    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error>;
}

impl<V: ProtobufConvert> ProtobufConvert for db::OptionalEntry<V>
where
    V::ProtoStruct: Message,
{
    type ProtoStruct = OptionalEntry;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut entry = OptionalEntry::new();
        let pb = self.value.to_pb();
        entry.set_value(pb.write_to_bytes().unwrap());
        entry
    }

    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
        let val = parse_from_bytes(pb.get_value());
        let value = V::from_pb(val.unwrap())?;
        Ok(db::OptionalEntry { value })
    }
}

impl<V: ProtobufConvert> ProtobufConvert for db::MapProof<V> {
    type ProtoStruct = MapProof;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut proof = MapProof::new();

        let entries = self
            .entries()
            .iter()
            .map(|entry| entry.to_pb())
            .collect::<Vec<_>>();

        proof.set_entries(RepeatedField::from(entries));
        proof
    }

    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
        let entries = pb
            .get_entries()
            .iter()
            .cloned()
            .map(|entry| ProtobufConvert::from_pb(entry).unwrap())
            .collect::<Vec<db::OptionalEntry<V>>>();

        Ok(db::MapProof { entries })
    }
}

impl ProtobufConvert for i32 {
    type ProtoStruct = Int32Value;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut val = Int32Value::new();
        val.set_value(*self);
        val
    }

    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
        Ok(pb.get_value())
    }
}

impl ProtobufConvert for String {
    type ProtoStruct = StringValue;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut val = StringValue::new();
        val.set_value(self.to_string());
        val
    }

    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
        Ok(pb.get_value().to_string())
    }
}

//impl<T:ProtobufConvert> ProtobufConvert for Vec<T> {
//    type ProtoStruct = Vec<T::ProtoStruct>;
//
//    fn to_pb(&self) -> Self::ProtoStruct {
//
//        self.iter().map(|v| {
//            v.to_pb()
//        }).collect()
//    }
//
//    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
//        Ok(pb.get_value().to_string())
//    }
//}

#[cfg(test)]
mod tests {
    use crate::proto::{db, ProtobufConvert};
    use crate::protos::test::{City, Person};
    use protobuf::well_known_types::{Any, Empty};
    use protobuf::{parse_from_bytes, Message, RepeatedField};

    #[test]
    fn proto() {
        let mut person = Person::new();
        person.set_name("Name".to_string());
        let mut city = City::new();
        city.set_name("Moscow".to_string());
        let mut city17 = City::new();
        city17.set_name("City 17".to_string());
        city17.set_no_value(Empty::new());

        let mut any = Any::new();
        any.set_value(vec![0_u8; 32]);

        city17.set_details(any);
        person.set_city(RepeatedField::from_vec(vec![city, city17]));

        let msg = person.write_to_bytes().unwrap();
        let person: Person = parse_from_bytes(&msg).unwrap();

        dbg!(person);
    }

    #[test]
    fn map_proof() {
        let _entry = db::OptionalEntry { value: "String" };

        let mut entries = vec![];

        for i in 0..1000 {
            entries.push(db::OptionalEntry {
                value: format!("{}", i),
            });
        }

        let proof = db::MapProof { entries };

        let pb_proof = ProtobufConvert::to_pb(&proof);
        let de_proof: db::MapProof<String> = ProtobufConvert::from_pb(pb_proof).unwrap();

        assert_eq!(proof, de_proof);

        u32::from(5_u8);
    }
}
