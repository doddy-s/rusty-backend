#[macro_use] extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User{
    name: String,
    age: i32
}

#[post("/post", format = "json", data = "<user>")]
async fn post_user(user: Json<User>) -> Result<Json<User>, Status> {
    let res = User {
        name: "success".to_string(),
        age: user.age
    };

    Ok(Json(res))
}

#[get("/get")]
async fn get_user() -> Result<Json<User>, Status> {
    let res = User {
        name: "success".to_string(),
        age: 16
    };

    Ok(Json(res))
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hey")]
async fn hey() -> &'static str {
    "Hey, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, hey, get_user, post_user])
}