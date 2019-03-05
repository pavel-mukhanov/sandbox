trait Foo {
    fn check(&self);
}

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


#[test]
fn trait_object() {

    fn static_dispatch<T: Foo>(obj: T) {
        let ref_to_t = &obj;

        let cast = ref_to_t as &Foo;

        let coerce: &Foo = ref_to_t;
    }

    fn dynamic_dispatch(obj: &Foo) {

    }

}