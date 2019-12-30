#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use routes::*;
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions
};

mod models;
mod routes;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http:localhost:3000",
        "http:127.0.0.1:3000",
    ]);

    CorsOptions{
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("somethings wrong with building CORS")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, test, register_temporary_user])
        .attach(make_cors())
}

fn main() {
    rocket().launch();
}