#[cfg(test)]
mod tests {
    use base64;
    use hex;
    use openssl::pkey::*;
    use openssl::rsa::*;
    use openssl::symm::Cipher;

    use secret_tree::{SEED_LEN, SecretTree, Name};
    use rand::thread_rng;
    use sodiumoxide::crypto::sign::{Seed, keypair_from_seed};
    use sodiumoxide::crypto::kx::{Seed as SeedKx, keypair_from_seed as keypair_from_seed_kx, gen_keypair};
    use sodiumoxide::crypto::hash;
    use snow::{Builder, params::NoiseParams};
    use std::collections::BTreeMap;

    #[test]
    fn kdf() {
        let tree = SecretTree::new(&mut thread_rng());
        let mut buffer = [0_u8; 32];
        tree.child(Name::new("validator")).fill(&mut buffer);
        let seed = Seed::from_slice(&buffer).unwrap();
        let (pk, sk) = keypair_from_seed(&seed);

        tree.child(Name::new("identity")).fill(&mut buffer);
        let seed = SeedKx::from_slice(&buffer).unwrap();
        let (pk, sk) = keypair_from_seed_kx(&seed);

        dbg!(pk, sk);
    }

    #[test]
    fn noise_with_sodium_kx() {
        let params: NoiseParams = "Noise_XX_25519_ChaChaPoly_SHA256".parse().unwrap();
        let (pk, sk) = gen_keypair();

        let mut h_i = Builder::new(params.clone())
            .local_private_key(sk.as_ref())
            .build_initiator().unwrap();

        let (pk, sk) = gen_keypair();

        let mut h_r = Builder::new(params)
            .local_private_key(sk.as_ref())
            .build_responder().unwrap();

        let mut buf = [0u8; 1024];
        let mut buf2 = [0u8; 1024];

        // -> e
        let len = h_i.write_message(&[], &mut buf).unwrap();
        let _ = h_r.read_message(&buf[..len], &mut buf2).unwrap();

        // <- e, ee s, es
        let len = h_r.write_message(&[], &mut buf).unwrap();
        let _ = h_i.read_message(&buf[..len], &mut buf2).unwrap();

        // -> s, se
        let len = h_i.write_message(&[], &mut buf).unwrap();
        let _ = h_r.read_message(&buf[..len], &mut buf2).unwrap();

        let mut h_i = h_i.into_transport_mode().unwrap();
        let mut h_r = h_r.into_transport_mode().unwrap();

        let len = h_i.write_message(b"HACK THE PLANET", &mut buf).unwrap();

        let len = h_r.read_message(&buf[..len], &mut buf2).unwrap();
        println!("{:?}", pk.as_ref());
        println!("{:?}", h_i.get_remote_static());

        println!("client said: {}", String::from_utf8_lossy(&buf2[..len]));
    }

    #[test]
    fn hash_order() {
        let mut map = BTreeMap::new();

        for i in 0..10 {
            let hash = hash::hash(&vec![i; 16]);
            map.insert(hash, i);
        }

        let hash = hash::hash(&vec![16; 16]);

        dbg!(&hash);

        map.insert(hash, 1);

        dbg!(map);
    }
}