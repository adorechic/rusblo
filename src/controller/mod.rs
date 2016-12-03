use params::{Params,Value};
use rustc_serialize::json;
use iron::prelude::*;
use iron::status;
use model::{Hello,User};
use router::Router;
use iron::headers::ContentType;
use logger::Logger;

pub fn start_server() {
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

fn render_json(status: status::Status, body: String) -> IronResult<Response> {
    Ok(Response::with((ContentType::json().0, status, body)))
}

pub struct HelloController {}

impl HelloController {
    pub fn show(_: &mut Request) -> IronResult<Response> {
        let resource = Hello { message: "Hello!".to_string() };
        let body = json::encode(&resource).unwrap();
        render_json(status::Ok, body)
    }
}

pub struct UserController {}

impl UserController {
    pub fn create(req: &mut Request) -> IronResult<Response> {
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

    pub fn index(_: &mut Request) -> IronResult<Response> {
        let users = User::all();
        let body = json::encode(&users).unwrap();

        render_json(status::Ok, body)
    }

    pub fn show(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let user = User::find(id);
        let body = json::encode(&user).unwrap();

        render_json(status::Ok, body)
    }
}
