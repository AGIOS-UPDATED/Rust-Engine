extern crate iron;

mod routes;
mod controllers;
mod models;

use iron::prelude::*;

pub fn all() -> impl iron::Handler {
    // Returning an actual Handler that Iron can use
    iron::Iron::new(|_req: &mut iron::Request| {
        Ok(iron::Response::with((iron::status::Ok, "Hello, world!")))
    })
}

pub fn run() {
    Iron::new(all()).http("localhost:3000").unwrap();
}