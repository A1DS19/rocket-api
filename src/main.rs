mod auth;
mod user;

use auth::AuthenticatedUser;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use user::User;

#[macro_use]
extern crate rocket;

#[get("/")]
fn get_name() -> Value {
    json!({"name": "Hello, world"})
}

#[post("/", format = "json", data = "<user>")]
fn post_name(current_user: AuthenticatedUser, user: Json<User>) -> Value {
    println!("Current user: {:?}", current_user);
    json!({"username": user.username, "password": user.password})
}

#[catch(404)]
fn not_found() -> Value {
    json!({"error": "Not found"})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"error": "Unauthorized"})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_name, post_name,])
        .register("/api", catchers![not_found, unauthorized])
}
