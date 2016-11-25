extern crate iron;

use iron::prelude::*;
use iron::status;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello")))
}

fn main() {
    Iron::new(handler).http("localhost:3000").unwrap();
}
