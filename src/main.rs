use std::str;
use std::process::Command;
use chrono::prelude::*;
use openssl::rsa::Rsa;
use openssl::pkey::{PKey};

fn main() {
    let output = Command::new("sh")
      .arg("-c")
      .arg("hostname; whoami; hostname -I; b2sum /boot/* 2>/dev/null; b2sum $(which archaeopteryx); ps auxww; w;")
      .output()
      .expect("Failed to execute command");
    let readi: DateTime<Utc> = Utc::now();
    println!("{} {:?}", readi, output);
    let rsa = Rsa::generate(4096).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();
    let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
    let pri_key: Vec<u8> = pkey.private_key_to_pem_pkcs8().unwrap();
    println!("{}", str::from_utf8(pub_key.as_slice()).unwrap());
    println!("{}", str::from_utf8(pri_key.as_slice()).unwrap());
}
