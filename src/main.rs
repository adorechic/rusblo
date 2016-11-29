extern crate iron;
extern crate logger;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate router;
extern crate params;
extern crate rusqlite;

use std::env;
use std::path::Path;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use logger::Logger;
use rustc_serialize::json;
use router::Router;
use params::{Params,Value};
use rusqlite::Connection;

#[derive(Debug, RustcEncodable)]
struct Hello {
    message: String
}

struct HelloController {}

impl HelloController {
    fn show(_: &mut Request) -> IronResult<Response> {
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
    id: i32,
    name: String
}

struct UserController {}

impl UserController {
    fn create(req: &mut Request) -> IronResult<Response> {
        let params = req.get_ref::<Params>().unwrap();
        match params.get("name") {
            Some(&Value::String(ref name)) => {
                let conn = connection();
                conn.execute(
                    "insert into users (name) values ($1)",
                    &[name]
                ).unwrap();

                let mut stmt = conn.prepare(
                    "select id, name from users order by id desc limit 1"
                ).unwrap();
                let mut users = stmt.query_map(&[], |row| {
                    User { id: row.get(0), name: row.get(1) }
                }).unwrap();
                let resource = users.next().unwrap().unwrap();
                let body = json::encode(&resource).unwrap();

                Ok(
                    Response::with(
                        (ContentType::json().0, status::Created, body)
                    )
                )
            },
            _ => panic!("error")
        }
    }
}

fn start_server() {
    let mut router = Router::new();
    router.get("/hello", HelloController::show, "hello");
    router.post("/users", UserController::create, "create_user");

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    info!("start server");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn connection() -> Connection {
    let path = Path::new("tmp/db");
    Connection::open(path).unwrap()
}

fn migrate() {
    info!("Run migration!");
    let conn = connection();
    conn.execute("drop table if exists users", &[]).unwrap();
    conn.execute(
        "create table users (
           id integer primary key autoincrement,
           name varchar not null
        )", &[]).unwrap();
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
