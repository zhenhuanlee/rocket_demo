#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod model;

#[get("/")]
fn index() -> String {
    let ps = &model::CPerson();
    let mut res = String::new();
    for p in ps {
        res.push_str(&format!("<p>{}-{}</p>", p.id, p.name));
    }
    res
}

fn main() {
    
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}