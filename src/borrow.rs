use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{Error, Formatter};
use std::io::Read;
use std::io::Write;
use std::rc::Rc;

struct Fork {
    value: String,
}

trait Snapshot {}

impl Fork {
    fn value(&self) -> &String {
        &self.value
    }
    fn set_value(&mut self, value: String) {
        self.value = value
    }
}

impl Snapshot for Fork {}

impl AsRef<Snapshot> for Fork {
    fn as_ref(&self) -> &(dyn Snapshot + 'static) {
        println!("As REF!!!");
        self
    }
}

struct MigrationSnapshot {
    fork: Rc<RefCell<Fork>>,
}

impl Snapshot for MigrationSnapshot {}

impl MigrationSnapshot {
    fn fork(&self) -> Rc<RefCell<Fork>> {
        self.fork.clone()
    }

    fn value(&self) -> String {
        let fork = self.fork.borrow();
        fork.value().clone()
    }

    fn set_value(&self, value: String) {
        let mut fork = self.fork.borrow_mut();
        fork.set_value(value)
    }
}

impl<'a> BaseIndex<&'a mut Fork> {
    /// Inserts the key-value pair into the index. Both key and value may be of *any* types.
    pub fn put<K, V>(&mut self, key: &K, value: V)
    where
        K: StorageKey,
        V: StorageValue,
    {
        self.set_index_type();
        let key = self.prefixed_key(key);
        self.view.put(&self.name, key, value.into_bytes());
    }
}

impl AsRef<Snapshot> for MigrationSnapshot {
    fn as_ref(&self) -> &(dyn Snapshot + 'static) {
        self
    }
}

#[test]
fn test_impl_as_ref() {
    let fork = Rc::new(RefCell::new(Fork {
        value: "Val!".to_string(),
    }));

    let snapshot1 = MigrationSnapshot { fork: fork.clone() };
    let snapshot2 = MigrationSnapshot { fork: fork.clone() };

    println!("snapshot value {:?}", snapshot1.value());

    snapshot1.set_value("New Val!".to_string());

    println!("snapshot value {:?}", snapshot2.value());
}
