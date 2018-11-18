#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod model;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    model::CPerson();
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}