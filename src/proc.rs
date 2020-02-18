use failure::Error;

pub trait ProtobufConvert: Sized {
    /// Type of the protobuf clone of Self
    type ProtoStruct;

    /// Struct -> ProtoStruct
    fn to_pb(&self) -> Self::ProtoStruct;

    /// ProtoStruct -> Struct
    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error>;
}

impl<T> ProtobufConvert for Vec<T>
where
    T: ProtobufConvert,
{
    type ProtoStruct = Vec<T::ProtoStruct>;

    fn to_pb(&self) -> Self::ProtoStruct {
        self.iter().map(ProtobufConvert::to_pb).collect()
    }
    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
        pb.into_iter()
            .map(ProtobufConvert::from_pb)
            .collect::<Result<Vec<_>, _>>()
    }
}

impl ProtobufConvert for Vec<u8> {
    type ProtoStruct = Vec<u8>;

    fn to_pb(&self) -> Self::ProtoStruct {
        self.clone()
    }
    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
        Ok(pb)
    }
}

//impl ProtobufConvert for u8 {
//    type ProtoStruct = u32;
//
//    fn to_pb(&self) -> Self::ProtoStruct {
//        unimplemented!()
//    }
//
//    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
//        unimplemented!()
//    }
//}

trait Conflict {
    type Type;
}

impl<T: Conflict> Conflict for Vec<T> {
    type Type = Vec<T::Type>;
}

impl Conflict for u8 {
    type Type = u32;
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use procs::{say_hello, say_hello_attr};

    #[test]
    fn proc_macro() {
        let _ = Hi {};
    }

    #[say_hello_attr(source = "World!")]
    #[derive(Debug)]
    struct Hello {}

    macro_rules! make_a_struct_and_getters {
    ($name:ident { $( ($field:ident, $upper:ident)),* }) => {
        paste::item! {
                $(
                    pub fn [<$name _ $field>]() -> &'static str {
                        $name::<$upper>();
                        "s"
                    }
                )*
        }
    }
}

    fn test_insert<S: KeyTransform>() {}

    trait KeyTransform {}

    enum Hash {}
    enum Raw {}

    impl KeyTransform for Hash {}

    impl KeyTransform for Raw {}

    #[test]
    fn test_paste() {
        make_a_struct_and_getters!(test_insert { (raw, Raw), (hash, Hash) });

        test_insert_raw();
        test_insert_hash();
    }
}
