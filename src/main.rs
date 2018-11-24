#![feature(proc_macro_hygiene, decl_macro, uniform_paths)]
#![feature(never_type)]

#[macro_use]
extern crate rocket;
extern crate sys_info;
use rocket::fairing::{AdHoc, Fairing, Info, Kind};
use rocket::http::{uri::Origin, Method, Status};
use rocket::{Data, Request, Response, Rocket};

mod service;
use service::auth;
use service::feature;

#[get("/no_auth")]
pub fn no_auth() -> Status {
    Status::new(401, "fuck off")
}

pub struct Auth;

impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Request Auth",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, req: &mut Request, _: &Data) {
        match req.guard::<auth::User>() {
            rocket::Outcome::Failure(_) => {
                req.set_method(Method::Get);
                req.set_uri(Origin::parse("/no_auth").unwrap());
            }
            _ => {}
        };
    }
}

fn main() {
    // ignite routes importaned
    let mut rocket = Rocket::ignite();
    rocket = feature::mount(rocket);
    rocket
        .mount("/", routes![no_auth])
        .attach(Auth)
        // .attach(AdHoc::on_request("Auth Fairing", |req, _| {
        //     match req.guard::<auth::User>() {
        //         rocket::Outcome::Failure(_) => {
        //             req.set_method(Method::Get);
        //             req.set_uri(Origin::parse("/no_auth").unwrap());
        //         }
        //         _ => {}
        //     };
        // }))
        .launch();
}
