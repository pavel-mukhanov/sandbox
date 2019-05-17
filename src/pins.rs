use std::collections::HashMap;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

// This is a self-referential struct since the slice field points to the data field.
// We cannot inform the compiler about that with a normal reference,
// since this pattern cannot be described with the usual borrowing rules.
// Instead we use a raw pointer, though one which is known to not be null,
// since we know it's pointing at the string.
struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    // To ensure the data doesn't move when the function returns,
    // we place it in the heap where it will stay for the lifetime of the object,
    // and the only way to access it would be through a pointer to it.
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

#[test]
fn unmoved() {
    let unmoved = Unmovable::new("hello".to_string());
    let still_unmoved = unmoved;

    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
}

#[test]
fn nested_map_pin() {
    let mut map = HashMap::new();
    let mut vec = Vec::new();
    vec.push(10);

    let pinned_vec = Box::pin(&vec);

    map.insert(&1, pinned_vec);

    dbg!(map);
}

#[test]
fn nested_map() {
    let mut map = HashMap::new();
    let mut vec = Vec::new();
    vec.push(10);

    map.insert(1, 2);
}
