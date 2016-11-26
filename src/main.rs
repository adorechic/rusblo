extern crate iron;
extern crate logger;
extern crate env_logger;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use logger::Logger;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(
        Response::with(
            (ContentType::json().0, status::Ok, "{\"message\": \"hello\"}")
        )
    )
}

fn main() {
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(handler);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    Iron::new(chain).http("localhost:3000").unwrap();
}
