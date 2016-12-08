extern crate dotenv;
extern crate env_logger;
extern crate rusblo;

use dotenv::dotenv;
use std::env;
use rusblo::model::migrate;
use rusblo::controller::start_server;

fn main() {
    dotenv().ok();
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "migrate" => migrate(),
        "server"  => start_server(),
        _         => panic!("Unknown command: {}", args[1]),
    }
}
