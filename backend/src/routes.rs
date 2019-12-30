use rocket_contrib::json::Json;
use crate::models::Data;
use crate::models::TemporaryUser;
use crate::database;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/v1/test")]
pub fn test() -> Json<Vec<Data>> {
    Json(vec![Data {
        id: 23,
        name: "sexydynamite".to_string(),
        done: false
    }])
}

#[post("/api/v1/signin", format = "application/json", data="<temporary_user>")]
pub fn register_temporary_user(temporary_user: Json<TemporaryUser>) -> String {
    database::insert_temporary_user(&temporary_user.email, &temporary_user.password);
    format!("Accepted Post Request {:?}", temporary_user.0)
}
