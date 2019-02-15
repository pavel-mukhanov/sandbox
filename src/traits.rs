use std::marker::PhantomData;

trait Snapshot {
    fn get(&self);
}

struct Fork {}

impl Fork {
    fn get_fork() {}
}

struct Index<T> {
    _v: PhantomData<T>,
}

impl<T> Index<T>
where
    T: Snapshot,
{
    fn new(snapshot: T) {
        snapshot.get();
    }
}

impl Snapshot for Fork {
    fn get(&self) {
        unimplemented!()
    }
}

impl Index<Fork> {
    fn new2(fork: Fork) {
        fork.get();
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    mate: Option<&'a Person<'a>>,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, mate: Option<&'a Person>) -> Self {
        Self { name, mate }
    }
}

#[test]
fn person_in_and_out() {
    let person = Person {
        name: "person",
        mate: None,
    };

    let tom = Person::new("Tom", Some(&person));

    dbg!(tom);

    println!("{}", (0x0001 << 3) as u64);
}
