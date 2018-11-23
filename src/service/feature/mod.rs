use super::auth;
use feature_models::*;
use rocket::request::Form;
use rocket::Rocket;
use rocket::{self, http::Status};
use rocket_contrib::json::Json;
use serde_json::Value;

pub mod feature_models;
pub mod feature_service;

#[get("/")]
pub fn index() -> String {
    format!("Hello world!")
}

#[get("/howdy")]
pub fn howdy_index() -> String {
    format!(
        "Hello 2.0 from the {} !",
        feature_service::get_module_path()
    )
}

#[get("/howdy/sysload")]
pub fn howdy_load() -> Json<SystemInfo> {
    Json(feature_service::sysinf())
}

#[get("/howdy/format")]
pub fn howdy_format() -> String {
    format!(
        "Hello 2.0 there, {}th world!",
        feature_service::plus_one(49)
    )
}

#[get("/howdy/<name>")]
pub fn howdy_name(name: String) -> String {
    format!("Howdy there! You told me your name is: {}.", name)
}

#[post("/howdy", data = "<person>")]
pub fn howdy_person_query(person: Form<Person>) -> String {
    format!(
        "Howdy there! You told me your name is: {}, and you are: {} years old.",
        person.name, person.age
    )
}

#[post("/howdy/json", data = "<person>")]
pub fn howdy_person_json(user: auth::User, person: Json<Person>) -> String {
    println!("HIT!");
    format!(
        "Howdy there! You told me your name is: {}, and you are: {} years old.",
        person.name, person.age
    )
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/", routes![index, howdy_index, howdy_format, howdy_name])
        .mount(
            "/",
            routes![howdy_load, howdy_person_query, howdy_person_json],
        )
        .mount("/user", routes![howdy_person_json])
}
