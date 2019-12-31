use rocket_contrib::json::Json;
use crate::models::TemporaryUser;
use crate::task;

#[post("/api/v1/signup", format = "application/json", data = "<temporary_user>")]
pub fn register_temporary_user(temporary_user: Json<TemporaryUser>) -> String {
    task::register_temporary_user_data(&temporary_user.password, &temporary_user.email);
    format!("Accepted Post Request {:?}", temporary_user.0)
}

#[post("/api/v1/signin", format = "application/json", data = "<temporary_user>")]
pub fn signin_temporary_user(temporary_user: Json<TemporaryUser>) -> String {
    task::signin_judge(&temporary_user.password, &temporary_user.email);
    format!("Accepted Post Request {:?}", temporary_user.0)
}
