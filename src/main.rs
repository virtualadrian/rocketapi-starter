#![feature(const_atomic_bool_new)]
#![feature(const_atomic_bool_new)]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate sys_info;

#[macro_use] extern crate serde_derive;



mod service;
mod static_fmod;

use service::feature::feature_controller;
use static_fmod::*;

fn main() {
    // ignite routes importaned
    rocket::ignite()
        .mount("/", routes![feature_controller::index,
                            feature_controller::howdy_index,
                            feature_controller::howdy_format,
                            feature_controller::howdy_name,
                            feature_controller::howdy_load,
                            feature_controller::howdy_person_query])
        .mount("/app", routes![static_index,
                               static_files])
        .launch();
}
