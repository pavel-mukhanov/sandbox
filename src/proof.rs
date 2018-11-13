use exonum::storage::{ProofListIndex, MemoryDB, Database};

#[test]
fn proof_list_index() {

    let db = MemoryDB::new();

    let mut fork = db.fork();

    let mut list: ProofListIndex<_, u32> = ProofListIndex::new("list", &mut fork);

    list.push(1);
    list.push(4398);
    list.push(10);
    list.push(55);
    list.push(123);
    list.push(30);

    let proof =    list.get_proof(5);
    let merkle_root = list.merkle_root();

    assert!(proof.validate(merkle_root, 6).is_ok());

    println!("merkle root {:?}", merkle_root);
    println!("proof {:#?}", proof);
}