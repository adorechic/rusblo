extern crate iron;
extern crate logger;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate router;
extern crate params;
extern crate rusqlite;
extern crate rusblo;

use std::env;
use iron::prelude::*;
use logger::Logger;
use router::Router;
use rusblo::model::migrate;

fn start_server() {
    let mut router = Router::new();
    router.get("/hello", rusblo::controller::HelloController::show, "hello");
    router.post("/users", rusblo::controller::UserController::create, "create_user");
    router.get("/users", rusblo::controller::UserController::index, "index_user");
    router.get("/users/:id", rusblo::controller::UserController::show, "show_user");

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    info!("start server");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "migrate" => migrate(),
        "server"  => start_server(),
        _         => panic!("Unknown command: {}", args[1]),
    }
}
