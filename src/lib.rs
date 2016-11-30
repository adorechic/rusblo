extern crate rustc_serialize;
extern crate rusqlite;

use std::path::Path;
use rusqlite::Connection;

#[derive(Debug, RustcEncodable)]
pub struct Hello {
    pub message: String
}

#[derive(Debug, RustcEncodable)]
pub struct User {
    pub id: i32,
    pub name: String
}

impl User {
    pub fn create(name: &String) -> User {
        let conn = connection();
        conn.execute(
            "insert into users (name) values ($1)",
            &[name]
        ).unwrap();

        let mut stmt = conn.prepare(
            "select last_insert_rowid()"
        ).unwrap();
        let id = stmt.query_map(&[], |row| {
            row.get(0)
        }).unwrap().next().unwrap().unwrap();

        User { id: id, name: name.to_string() }
    }
}

fn connection() -> Connection {
    let path = Path::new("tmp/db");
    Connection::open(path).unwrap()
}
