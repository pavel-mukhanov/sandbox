#[test]
fn test_lifetimes() {
    println!("foo {:?}", foo(1));
}

fn foo(x: u32) -> u32 {
    x
}
