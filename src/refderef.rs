use std::marker::PhantomData;
use std::ops::Deref;

struct Ref<T> {

    _t: PhantomData<T>,
}

struct RefMut<T> {
    _t: PhantomData<T>,
}

impl <T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

struct Foo<T> {
    a: T,
    s: T,
}

impl <T> Deref for Foo<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}


#[test]
fn deref() {


    let foo = Foo { a: String::new(), s: String::from("string") };


    let val = foo.as_str();

}
