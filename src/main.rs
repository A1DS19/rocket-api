use serde_json::{json, Value};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Value {
    json!({"name": "Hello, world"})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
