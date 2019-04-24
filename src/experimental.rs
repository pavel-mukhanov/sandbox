
#[cfg(Test)]
mod tests {
    #[allow(unused_imports)]
    use procs::say_hello;
    use std::iter::from_fn;

    #[test]
    fn concat_idents() {
        fn foobar() -> u32 {
            23
        }

        let f = concat_idents!(foo, bar);
        println!("{}", f());
    }

    #[test]
    fn proc_macro() {
        dbg!(say_hello!());
    }

    fn from_iter_fn() -> impl Iterator<Item=i32> {
        let mut count = -1;

        from_fn(move || {
            count += 1;
            if count > 10 {
                None
            } else {
                Some(count)
            }
        })
    }

    #[test]
    fn iter_from_fn() {
        let iter = from_iter_fn();

        for item in iter {
            println!("{:?}", item);
        }
    }
}
