#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod person;
use person::{Person};

#[post("/", data = "<person>")]
fn create(person: Json<Person>) -> Json<Person> {
    person
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "hero 1",
        "hero 2"
    ]))
}

#[put("/<id>", data = "<person>")]
fn update(id: i32, person: Json<Person>) -> Json<Person> {
    person
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite()
        .mount("/people", routes![create, update, delete, read])
        .launch();
}