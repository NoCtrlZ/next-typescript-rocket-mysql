use rocket_contrib::json::Json;
use crate::models::Data;

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
