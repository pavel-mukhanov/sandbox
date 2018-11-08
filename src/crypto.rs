
use openssl::symm::Cipher;
use openssl::dh::Dh;
use openssl::dsa::Dsa;
use openssl::ec::EcKey;
use openssl::nid::Nid;
use openssl::rsa::*;
use openssl::pkey::*;
use std::{io, ptr};
use openssl::x509::*;
use base64;
use hex;
use sodiumoxide::crypto::sign::gen_keypair;


use foreign_types_shared::ForeignType;

#[test]
fn test_pem_rsa() {

    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();
    let pem = pkey.private_key_to_pem_pkcs8_passphrase(Cipher::aes_128_cbc(), b"foobar")
        .unwrap();
    PKey::private_key_from_pem_passphrase(&pem, b"foobar").unwrap();

    assert!(PKey::private_key_from_pem_passphrase(&pem, b"fizzbuzz").is_err());



    let string = String::from_utf8(pem);

    println!("pem as string {}", string.unwrap());
}

#[test]
fn test_read_pem() {
    let key = include_bytes!("../key_aes_256.pem");
    let key = PKey::private_key_from_pem_passphrase(key, b"hello").unwrap();
    let key = key.private_key_to_der().unwrap();
    let key_base64 = base64::encode(&key);
    println!("key_base64 {:?}", key_base64);


    let key_hex = hex::decode(key);

    println!("key_hex {:?}", key_hex);
}

#[test]
fn test_write_pem() {
    let _key = include_bytes!("../key_aes_256.pem");

    let sodium_key = gen_keypair();
    let sodium_private_key = &sodium_key.0[..];

    println!("sodium private key {:?}", sodium_private_key);
//    println!("key bytes {:?}", &key[..]);

    let sodium_key = base64::encode(sodium_private_key);
    println!("sodium private key base64 {:?}", sodium_key);
}