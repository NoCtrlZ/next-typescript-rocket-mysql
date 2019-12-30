use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub id: u32,
    pub name: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemporaryUser {
    pub email: String,
    pub password: String,
}