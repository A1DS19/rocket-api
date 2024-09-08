mod auth;
mod db_connection;
mod dto;
mod models;
mod repositories;
mod schema;
mod user;

use auth::AuthenticatedUser;
use db_connection::DbConn;
use dotenvy::dotenv;
use dto::new_rustacean::NewRustacean;
use repositories::{repository::Repository, rustacean_repository};
use rocket::serde::json::Json;
use rustacean_repository::RustaceanRepository;
use serde_json::{json, Value};

#[macro_use]
extern crate rocket;

#[get("/")]
async fn get_rustaceans(conn: DbConn) -> Value {
    conn.run(|c| {
        let result = RustaceanRepository::find_many(c);

        match result {
            Ok(rustaceans) => {
                json!(rustaceans)
            }
            Err(error) => {
                println!("Error loading rustaceans: {:?}", error);
                json!({"error": "Error loading rustaceans"})
            }
        }
    })
    .await
}

#[post("/", format = "json", data = "<user>")]
async fn post_name(
    current_user: AuthenticatedUser,
    user: Json<NewRustacean>,
    conn: DbConn,
) -> Value {
    println!("Current user: {:?}", current_user);

    conn.run(|c| {
        let result = RustaceanRepository::create(c, user.into_inner());

        match result {
            Ok(_) => {
                json!({"status": "ok"})
            }
            Err(error) => {
                println!("Error inserting new rustacean: {:?}", error);
                json!({"error": "Error inserting new rustacean"})
            }
        }
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!({"error": "Not found"})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"error": "Unauthorized"})
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({"error": "Unprocessable entity"})
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(DbConn::fairing())
        .mount("/api", routes![get_rustaceans, post_name])
        .register(
            "/api",
            catchers![not_found, unauthorized, unprocessable_entity],
        )
}
