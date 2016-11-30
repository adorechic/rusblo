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
use std::path::Path;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use logger::Logger;
use rustc_serialize::json;
use router::Router;
use params::{Params,Value};
use rusqlite::Connection;
use rusblo::{Hello,User};

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

struct UserController {}

impl UserController {
    fn create(req: &mut Request) -> IronResult<Response> {
        let params = req.get_ref::<Params>().unwrap();
        match params.get("name") {
            Some(&Value::String(ref name)) => {
                let user = User::create(name);
                let body = json::encode(&user).unwrap();

                Ok(
                    Response::with(
                        (ContentType::json().0, status::Created, body)
                    )
                )
            },
            _ => panic!("error")
        }
    }

    fn index(_: &mut Request) -> IronResult<Response> {
        let conn = connection();
        let mut stmt = conn.prepare(
            "select id, name from users"
        ).unwrap();
        let users: Vec<User> = stmt.query_map(&[], |row| {
            User { id: row.get(0), name: row.get(1) }
        }).unwrap().map(|r| r.unwrap()).collect::<Vec<User>>();
        let body = json::encode(&users).unwrap();

        Ok(
            Response::with(
                (ContentType::json().0, status::Ok, body)
            )
        )
    }

    fn show(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let conn = connection();
        let mut stmt = conn.prepare(
            "select id, name from users where id = $1 limit 1"
        ).unwrap();
        let user: User = stmt.query_map(&[id], |row| {
            User { id: row.get(0), name: row.get(1) }
        }).unwrap().next().unwrap().unwrap();
        let body = json::encode(&user).unwrap();

        Ok(
            Response::with(
                (ContentType::json().0, status::Ok, body)
            )
        )

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
