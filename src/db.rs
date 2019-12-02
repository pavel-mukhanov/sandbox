use rocksdb::{DB, Options};


pub fn list_cf() {
    let path = "/Users/pavelmukhanov/work/rust/exonum/examples/cryptocurrency-advanced/backend/db2";
    {
        let db = DB::open_default(path).unwrap();
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }
    let _ = DB::destroy(&Options::default(), path);
}
