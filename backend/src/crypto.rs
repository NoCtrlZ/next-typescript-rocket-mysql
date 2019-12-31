extern crate rand;
extern crate crypto;

use rand::seq::SliceRandom;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const STRETCHING: u32 = 1024;

fn genelate_random_string() -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, 16)
            .cloned()
            .collect()
    ).unwrap()
}

fn genelate_key(email: &str) -> String {
    let param = genelate_random_string();
    let value = format!("{}{}", &param, email);
    let mut sha256 = Sha256::new();
    sha256.input_str(&value);
    sha256.result_str()
}

pub fn create_hash_from_password(password: &str, email: &str) -> (String, String, String) {
    let salt = genelate_random_string();
    let value = format!("{}{}", &salt, password);
    (stretch_hash_value(&value), salt, genelate_key(&email))
}

pub fn stretch_hash_value(value: &str) -> String {
    let mut sha256 = Sha256::new();
    for _ in 0..STRETCHING {
        sha256.input_str(&value);
    }
    sha256.result_str()
}

pub fn hash_salt_and_password(salt: &str, password: &str) -> String {
    let value = format!("{}{}", &salt, password);
    stretch_hash_value(&value)
}
