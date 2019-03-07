use std::fmt::Debug;

trait Foo {
    fn check(&self) where Self: Debug;
}

#[derive(Debug)]
struct Bar {}

//impl Foo for Bar {
//    fn check(&self) {
//        println!("I'm regular Foo");
//    }
//}

impl Foo for &Bar {
    fn check(&self) {
        println!("I'm reference Foo");
    }
}

fn ref_bar(bar: &Bar) {
    bar.check();
}

#[test]
fn reference_trait() {
    let bar = Bar {};

    //    bar.check();

    ref_bar(&bar);
}

#[derive(Debug)]
struct FooUnsized<T: ?Sized> {
    foo: T
}

impl <T> FooUnsized<T> {
    fn uniform(&self) {
        println!("fucking magic");
    }
}

impl <T> Foo for FooUnsized<T> {
    fn check(&self) {
        println!("impl foo check");
    }
}

#[test]
fn unsized_fun() {
    let foo_sized = FooUnsized { foo: [1u8,2,3,4] };
    let foo_unsized: &FooUnsized<_> = &foo_sized;
    for &i in foo_unsized.foo.iter() {
        print!("{}", i);
    }
    println!("");

    fn foo<T: Foo + Debug>(obj: &T) {

        obj.check();
    }

    foo(foo_unsized);

//    foo_unsized.uniform();

    FooUnsized::uniform(foo_unsized);

}

trait SizedFoo {

}

impl SizedFoo for Bar {

}

#[test]
fn sized() {

    let b = Bar {};

    let s: &SizedFoo = &b;

}

