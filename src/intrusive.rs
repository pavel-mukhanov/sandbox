use exonum_crypto::{CryptoHash, Hash};

use intrusive_collections::{LinkedList, LinkedListLink};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// A simple struct containing an instrusive link and a value
struct Test {
    link: LinkedListLink,
    value: Cell<i32>,
}

// The adapter describes how an object can be inserted into an intrusive
// collection. This is automatically generated using a macro.
intrusive_adapter!(TestAdapter = Box<Test>: Test { link: LinkedListLink });

#[test]
fn intrusive() {
    // Create a list and some objects
    let mut list = LinkedList::new(TestAdapter::new());
    let a = Box::new(Test {
        link: LinkedListLink::new(),
        value: Cell::new(1),
    });
    let b = Box::new(Test {
        link: LinkedListLink::new(),
        value: Cell::new(2),
    });
    let c = Box::new(Test {
        link: LinkedListLink::new(),
        value: Cell::new(3),
    });

    // Insert the objects at the front of the list
    list.push_front(a);
    list.push_front(b);
    list.push_front(c);
    assert_eq!(
        list.iter().map(|x| x.value.get()).collect::<Vec<_>>(),
        [3, 2, 1]
    );

    // At this point, the objects are owned by the list, and we can modify
    // them through the list.
    list.front().get().unwrap().value.set(4);
    assert_eq!(
        list.iter().map(|x| x.value.get()).collect::<Vec<_>>(),
        [4, 2, 1]
    );

    // Removing an object from an instrusive collection gives us back the
    // Box<Test> that we originally inserted into it.
    let a = list.pop_front().unwrap();
    assert_eq!(a.value.get(), 4);
    assert_eq!(
        list.iter().map(|x| x.value.get()).collect::<Vec<_>>(),
        [2, 1]
    );

    // Dropping the collection will automatically free b and c by
    // transforming them back into Box<Test> and dropping them.
    drop(list);
}

#[derive(Debug)]
struct Object {
    val: u32,
}

#[derive(Debug, Default)]
struct Fork {
    objects: Vec<ObjectRef>,
}

impl Fork {
    fn new() -> Self {
        Self::default()
    }

    fn finalize(&self) -> Vec<Hash> {
        self.objects
            .iter()
            .map(|object| object.borrow().hash())
            .collect()
    }

    fn add(&mut self, object: ObjectRef) {
        self.objects.push(object)
    }
}

type ObjectRef = Rc<RefCell<Object>>;

impl Object {
    fn from(val: u32) -> Self {
        Self { val }
    }

    fn add_and_return(fork: &mut Fork) -> ObjectRef {
        let val = 10;

        let item = Rc::new(RefCell::new(Object::from(val)));

        fork.add(item.clone());

        item.clone()
    }

    fn update(&mut self, val: u32) {
        self.val = val;
    }

    fn hash(&self) -> Hash {
        self.val.hash()
    }
}

#[test]
fn refcell_item() {
    let mut fork = Fork::new();

    let object1 = Object::add_and_return(&mut fork);
    let object2 = Object::add_and_return(&mut fork);
    let object3 = Object::add_and_return(&mut fork);

    object1.borrow_mut().update(10);
    object2.borrow_mut().update(20);
    object3.borrow_mut().update(30);

    dbg!(fork.finalize());
}
