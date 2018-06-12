#[test]
fn test_lifetimes() {
    println!("foo {:?}", foo(1));
}

fn foo<'a>(x: u32) -> &'a u32 {
    &x
}
