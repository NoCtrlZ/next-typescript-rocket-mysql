use crate::crypto;
use crate::database;

pub fn register_temporary_user_data(password: &str, email: &str) {
    let (hashed, salt, cypher) = crypto::create_hash_from_password(&password, &email);
    database::insert_temporary_user(&email, &hashed, &salt, &cypher);
}

pub fn signin_judge(password: &str, email: &str) {
    let (salt, user_password) = database::select_temporary_user_salt(&email);
    let hashed_password = crypto::hash_salt_and_password(&salt, &password);
    if user_password == hashed_password {
        println!("password is correct");
    }
}
