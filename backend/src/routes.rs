use rocket_contrib::json::Json;
use crate::models::TemporaryUser;
use crate::database;
use crate::crypto;


#[post("/api/v1/signup", format = "application/json", data = "<temporary_user>")]
pub fn register_temporary_user(temporary_user: Json<TemporaryUser>) -> String {
    let (hashed, salt, cypher) = crypto::create_hash_from_password(&temporary_user.password, &temporary_user.email);
    database::insert_temporary_user(&temporary_user.email, &hashed, &salt, &cypher);
    format!("Accepted Post Request {:?}", temporary_user.0)
}

#[post("/api/v1/signin", format = "application/json", data = "<temporary_user>")]
pub fn signin_temporary_user(temporary_user: Json<TemporaryUser>) -> String {
    let (salt, password) = database::select_temporary_user_salt(&temporary_user.email);
    let hashed_password = crypto::hash_salt_and_password(&salt, &temporary_user.password);
    if password == hashed_password {
        println!("password is correct");
    }
    println!("Accested Login Request {:?}", hashed_password);
    format!("Accepted Post Request {:?}", temporary_user.0)
}
