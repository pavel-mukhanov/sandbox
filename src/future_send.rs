use futures::{self, future, Future};
use failure;
use futures::future::ok;
use tokio;
use crate::codecs::log_error;
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::Arc;
use futures_cpupool::CpuPool;
use external;
use std::fmt::Display;
use num::Num;

#[test]
fn test_future_send() {

    bar::<usize>(1)
}

pub fn bar<P: Display + Num>( // Error won't happen if "bar" is not generic
               baz: P,
) where {

    println!("baz {}", baz);

    foo(baz);
}

fn foo<N:Display + Num>(arg: N) { // Error won't happen if "foo" isn't used in "iterate" or has generics
    println!("arg {}", arg);
}

