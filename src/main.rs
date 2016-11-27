extern crate iron;
extern crate logger;
extern crate env_logger;
extern crate rustc_serialize;
extern crate router;

use std::env;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::Handler;
use logger::Logger;
use rustc_serialize::json;
use router::Router;

#[derive(Debug, RustcEncodable)]
struct Hello {
    message: String
}

struct HelloHandler {}

impl Handler for HelloHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let resource = Hello { message: "Hello!".to_string() };
        let body = json::encode(&resource).unwrap();

        Ok(
            Response::with(
                (ContentType::json().0, status::Ok, body)
            )
        )
    }
}

#[derive(Debug, RustcEncodable)]
struct User {
    id: u16,
    name: String
}

struct CreateUserHandler {}

impl Handler for CreateUserHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let resource = User { id: 1, name: "Alice".to_string() };
        let body = json::encode(&resource).unwrap();

        Ok(
            Response::with(
                (ContentType::json().0, status::Created, body)
            )
        )
    }
}

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/hello", HelloHandler{}, "hello");
    router.post("/users", CreateUserHandler{}, "create_user");

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
