#![feature(proc_macro_hygiene, decl_macro, uniform_paths)]
#[macro_use]
extern crate rocket;
extern crate sys_info;

use rocket::Rocket;

mod service;
use service::auth::application_auth::*;
use service::feature;

fn main() {
    // ignite routes importaned
    let mut rocket = Rocket::ignite();
    rocket = feature::mount(rocket);
    rocket
        .mount("/", routes![no_auth])
        .attach(ApplicationAuth)
        .launch();
}
