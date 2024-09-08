use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::user::User;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: User,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthenticatedUser {
    type Error = ();
    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();

        if let Some(token) = headers.get_one("Authorization") {
            if let Some(user) = authorize(token) {
                return Outcome::Success(AuthenticatedUser { user });
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

pub fn authorize(header: &str) -> Option<User> {
    let split = header.split_whitespace().collect::<Vec<_>>();

    if split.len() < 2 {
        return None;
    }

    if split[0] != "Basic" {
        return None;
    }

    decode_base_64(&split[1])
}

pub fn decode_base_64(encoded: &str) -> Option<User> {
    let decoded = base64::decode(encoded).ok()?;
    let decoded = String::from_utf8(decoded).ok()?;
    let split = decoded.split(":").collect::<Vec<_>>();

    if split.len() < 2 {
        return None;
    }

    Some(User {
        username: split[0].to_string(),
        password: split[1].to_string(),
    })
}
