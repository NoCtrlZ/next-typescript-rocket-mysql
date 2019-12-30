extern crate rand;
extern crate crypto;

use rand::seq::SliceRandom;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const STRETCHING: u32 = 1024;

pub fn genelate_salt() -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, 16)
            .cloned()
            .collect()
    ).unwrap()
}

pub fn create_hash_from_password(salt: &str, password: &str) -> String {
    let value = format!("{}{}", salt, password);
    stretch_hash_value(&value)
}

pub fn stretch_hash_value(hash: &str) -> String {
    let mut sha256 = Sha256::new();
    for _ in 0..STRETCHING {
        sha256.input_str(&hash);
    }
    sha256.result_str()
}
