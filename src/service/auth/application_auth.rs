use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{uri::Origin, Method, Status};
use rocket::{Data, Request};

use super::application_user::User;

pub struct ApplicationAuth;
impl Fairing for ApplicationAuth {
    fn info(&self) -> Info {
        Info {
            name: "Request Auth",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, req: &mut Request, _: &Data) {
        match req.guard::<User>() {
            rocket::Outcome::Failure(_) => {
                req.set_method(Method::Get);
                req.set_uri(Origin::parse("/no_auth").unwrap());
            }
            _ => {}
        };
    }
}

#[get("/no_auth")]
pub fn no_auth() -> Status {
    Status::new(401, "Authorization Failed")
}
