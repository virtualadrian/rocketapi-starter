use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: String,
    email: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        let key: String =
            String::from("puXoh5queicho8shiengei8teeViev8sho6quahRohng3achahB6ruth8thieLae");

        if keys.len() == 1 && keys[0] == key {
            let user: User = User {
                user_id: String::from("0"),
                email: String::from("adrian@adrian.work"),
            };
            return Outcome::Success(user);
        }

        Outcome::Failure((rocket::http::Status::new(401, "Invalid Auth."), ()))
    }
}
