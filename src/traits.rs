use protobuf::Message;

trait BinaryForm {
    fn encode();
}

impl<T> BinaryForm for T
where
    T: Message,
{
    fn encode() {
        unimplemented!()
    }
}
