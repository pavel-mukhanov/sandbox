use exonum::storage::{Database, MemoryDB, ProofListIndex};

#[test]
fn proof_list_index() {
    let db = MemoryDB::new();
    let mut fork = db.fork();

    let mut list: ProofListIndex<_, u32> = ProofListIndex::new("list", &mut fork);

    list.push(1);
    list.push(2);
    list.push(3);

    let merkle_root = list.merkle_root();
    println!("merkle root {:?}", merkle_root);

    let proof = list.get_proof(2);

    println!("proof {:#?}", proof);
}
