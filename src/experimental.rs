#[cfg(test)]
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

    fn from_iter_fn() -> impl Iterator<Item = i32> {
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

    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;

    pub fn up_to(limit: u64) -> impl Generator<Yield = u64, Return = u64> {
        move || {
            for x in 0..limit {
                yield x;
            }
            return limit;
        }
    }

    #[test]
    fn generator() {
        let mut generator = || {
            yield 1i32;
            return "foo";
        };

        match Pin::new(&mut generator).resume() {
            GeneratorState::Yielded(val) => {
                dbg!(val);
            }
            _ => panic!("unexpected return from resume"),
        }
        match Pin::new(&mut generator).resume() {
            GeneratorState::Complete("foo") => {}
            _ => panic!("unexpected return from resume"),
        }
    }

    #[test]
    fn ref_generator() {
        let mut generator = up_to(10);

        let val = Pin::new(&mut generator).resume();

        dbg!(val);
    }

    #[test]
    fn match_slice() {
        let slice = [0_u8; 0];

        match slice[..] {
            [] => println!("empty slice"),
            [x] => println!("{:?}", x),
            _ => println!("unknown thing"),
        }
    }

}
