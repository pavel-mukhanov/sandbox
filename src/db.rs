use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Foo {
    inner: u32,
}

impl AsRef<u32> for Foo {
    fn as_ref(&self) -> &u32 {
        &self.inner
    }
}

#[allow(dead_code)]
struct DB {
    data: HashMap<u16, Vec<u8>>,
}

impl DB {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn put(&mut self, id: u16, object: Vec<u8>) {
        self.data.insert(id, object);
    }

    #[allow(dead_code)]
    fn get(&self, id: &u16) -> Option<&Vec<u8>> {
        self.data.get(id)
    }

    #[allow(dead_code)]
    fn get_mut(&self, id: &u16) -> RefCell<&Vec<u8>> {
        let data = self.data.get(id);

        RefCell::new(data.unwrap())
    }
}

#[test]
fn db_basic() {
    let mut db = DB::new();

    db.put(1, vec![1]);

    let val = db.get_mut(&1);
    let val = *val.borrow_mut();

    dbg!(val);
}

#[test]
fn simple_reborrow() {
    let mut borrow_tree = (0, (1, (3, 4)), (2, (5, 6)));

    dbg!(borrow_tree.0);
    dbg!((borrow_tree.1).1);
    dbg!(((borrow_tree.2).1).0);

    let t_1_1_0 = &mut ((borrow_tree.1).1).0;

    *t_1_1_0 = 15;

    dbg!(borrow_tree);
}

#[test]
fn hash_map_get_mut() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    dbg!(map.clone());

    let _val = map.get(&1);
    let val_mut = map.get_mut(&1).unwrap();

    *val_mut = 10;

    dbg!(map);
}

#[test]
fn borrow() {
    let foo = Foo { inner: 7u32 };

    println!("foo {:?}", foo);
    println!("foo as ref {:?}", foo.as_ref());
}
