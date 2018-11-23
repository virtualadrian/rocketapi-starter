#![feature(proc_macro_hygiene, decl_macro, uniform_paths)]

#[macro_use]
extern crate rocket;
extern crate sys_info;
use rocket::Rocket;

mod service;
use service::feature;

fn main() {
    // ignite routes importaned
    let mut rocket = Rocket::ignite();
    rocket = feature::mount(rocket);
    rocket.launch();
}
