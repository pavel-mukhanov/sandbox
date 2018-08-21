#[macro_use]
extern crate log;

#[derive(Debug)]
pub struct State {
    s: String,
}

#[derive(Debug)]
pub struct StateHolder<K = State> {
    state: K,
}

impl StateHolder {
    pub fn new() -> StateHolder {
        StateHolder {
            state: State {
                s : String::from("State"),
            }
        }
    }
}

fn main() {
    let state = StateHolder::new();

    println!("state: {:#?}", state);


    println!("u16 max value {:?}", <u16>::max_value())
}