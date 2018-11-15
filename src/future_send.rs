use std::fmt::Display;

use num::Num;

#[test]
fn test_future_send() {
    bar::<usize>(1)
}

pub fn bar<P: Display + Num>(
    // Error won't happen if "bar" is not generic
    baz: P
) {
    println!("baz {}", baz);

    foo(baz);
}

fn foo<N: Display + Num>(arg: N) {
    // Error won't happen if "foo" isn't used in "iterate" or has generics
    println!("arg {}", arg);
}
