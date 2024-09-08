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
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use rustacean_repository::RustaceanRepository;
use serde_json::{json, Value};

#[macro_use]
extern crate rocket;

type RequestWithDataResult = Result<Custom<Json<Value>>, Custom<Json<Value>>>;
type RequestWithoutDataResult = Result<Custom<()>, Custom<Json<Value>>>;

#[get("/")]
async fn get_rustaceans(conn: DbConn) -> RequestWithDataResult {
    conn.run(|c| {
        let result = RustaceanRepository::find_many(c);

        match result {
            Ok(rustaceans) => Ok(Custom(Status::Ok, Json(json!(rustaceans)))),

            Err(error) => {
                println!("Error loading rustaceans: {:?}", error);
                Err(Custom(
                    Status::InternalServerError,
                    Json(json!({"error": "Error loading rustaceans"})),
                ))
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
) -> RequestWithoutDataResult {
    conn.run(|c: &mut diesel::SqliteConnection| {
        let result = RustaceanRepository::create(c, user.into_inner());

        match result {
            Ok(_) => {
                println!("New rustacean inserted");
                Ok(Custom(Status::Created, ()))
            }
            Err(error) => {
                println!("Error inserting new rustacean: {:?}", error);
                Err(Custom(
                    Status::InternalServerError,
                    Json(json!({"error": "Error inserting new rustacean"})),
                ))
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
