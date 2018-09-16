#![feature(plugin)]
#![plugin(rocket_codegen)]

#![feature(custom_derive)]

#![feature(custom_attribute)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate rocket;
extern crate serde;

pub mod person;
pub mod routes;


fn main() {
    rocket::ignite().mount("/", routes![
        routes::index,
        routes::hello,
        routes::get_json
    ]).launch();
}