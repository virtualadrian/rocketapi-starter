#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate sys_info;

mod service;
mod static_fmod;

use service::feature_module::*;
use static_fmod::*;

fn main() {
    // ignite routes importaned
    rocket::ignite()
        .mount("/", routes![feature_module::index,
                            feature_module::howdy_index,
                            feature_module::howdy_format,
                            feature_module::howdy_name,
                            feature_module::howdy_load,
                            feature_module::howdy_person_query])
        .mount("/app", routes![static_index,
                               static_files])
        .launch();
}
