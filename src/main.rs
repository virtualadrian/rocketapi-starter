#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod service;

use service::feature_module::*;


fn main() {
    // ignite routes importaned
    rocket::ignite()
        .mount("/", routes![feature_module::index,
                            feature_module::howdy_index,
                            feature_module::howdy_format,
                            feature_module::howdy_name,
                            feature_module::howdy_person_query])
        .launch();
}
