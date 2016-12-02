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
use iron::status;
use iron::headers::ContentType;
use logger::Logger;
use rustc_serialize::json;
use router::Router;
use params::{Params,Value};
use rusblo::{Hello,User,migrate};

struct HelloController {}

fn render_json(status: status::Status, body: String) -> IronResult<Response> {
    Ok(Response::with((ContentType::json().0, status, body)))
}

impl HelloController {
    fn show(_: &mut Request) -> IronResult<Response> {
        let resource = Hello { message: "Hello!".to_string() };
        let body = json::encode(&resource).unwrap();
        render_json(status::Ok, body)
    }
}

struct UserController {}

impl UserController {
    fn create(req: &mut Request) -> IronResult<Response> {
        let params = req.get_ref::<Params>().unwrap();
        match params.get("name") {
            Some(&Value::String(ref name)) => {
                let user = User::create(name);
                let body = json::encode(&user).unwrap();
                render_json(status::Created, body)
            },
            _ => panic!("error")
        }
    }

    fn index(_: &mut Request) -> IronResult<Response> {
        let users = User::all();
        let body = json::encode(&users).unwrap();

        render_json(status::Ok, body)
    }

    fn show(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let user = User::find(id);
        let body = json::encode(&user).unwrap();

        render_json(status::Ok, body)
    }
}

fn start_server() {
    let mut router = Router::new();
    router.get("/hello", HelloController::show, "hello");
    router.post("/users", UserController::create, "create_user");
    router.get("/users", UserController::index, "index_user");
    router.get("/users/:id", UserController::show, "show_user");

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
