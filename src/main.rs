#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod service;

use service::albums::*;

fn main() {

    rocket::ignite()
        .mount("/", routes![howdy, doody, myname, theirs])
        .launch();
}
