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
use rusblo::model::migrate;
use rusblo::controller::start_server;

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
