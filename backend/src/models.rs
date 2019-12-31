use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemporaryUser {
    pub email: String,
    pub password: String,
}
