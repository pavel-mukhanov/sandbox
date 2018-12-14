use exonum::storage::{Database, MemoryDB, ProofListIndex};

#[test]
fn range_proof() {
    let db = MemoryDB::new();
    let name = "name";
    let mut fork = db.fork();
    let mut index = ProofListIndex::new(name, &mut fork);
    index.extend([10, 11, 12, 13, 14].iter().cloned());
    let range_proof = index.get_range_proof(0, 5);

    println!("range proof {:#?}", range_proof);

    let list_proof = index.get_proof(5);

    println!("list proof {:#?}", list_proof);
}
