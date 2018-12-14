trait Foo {}
auto trait FooAuto {}

impl<T> !FooAuto for T where T: Foo {}
fn test<T>(t: T) where T: FooAuto {}

fn main() {
    test(1i32);
}