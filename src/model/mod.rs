use std::path::Path;
use rusqlite::{Connection,Row};
use rusqlite::types::ToSql;

#[derive(Debug, RustcEncodable)]
pub struct Hello {
    pub message: String
}

#[derive(Debug, RustcEncodable)]
pub struct User {
    pub id: i32,
    pub name: String
}


fn insert(sql: &str, params: &[&ToSql]) -> i32 {
    let conn = connection();
    conn.execute(sql, params).unwrap();
    let mut stmt = conn.prepare(
        "select last_insert_rowid()"
    ).unwrap();
    let id = stmt.query_map(&[], |row| {
        row.get(0)
    }).unwrap().next().unwrap().unwrap();
    id
}

trait RowMapper {
    fn map(row: &Row) -> Self;
}

impl RowMapper for User {
    fn map(row: &Row) -> User {
        User { id: row.get(0), name: row.get(1) }
    }
}

impl User {
    pub fn create(name: &String) -> User {
        let id = insert(
            "insert into users (name) values ($1)",
            &[name]
        );
        User { id: id, name: name.to_string() }
    }

    pub fn all() -> Vec<User> {
        let conn = connection();
        let mut stmt = conn.prepare(
            "select id, name from users"
        ).unwrap();

        let users: Vec<User> = stmt.query_map(&[], |row| {
            User::map(row)
        }).unwrap().map(|r| r.unwrap()).collect::<Vec<User>>();

        users
    }

    pub fn find(id: &str) -> User {
        let conn = connection();
        let mut stmt = conn.prepare(
            "select id, name from users where id = $1 limit 1"
        ).unwrap();
        let user: User = stmt.query_map(&[&id], |row| {
            User::map(row)
        }).unwrap().next().unwrap().unwrap();
        user
    }

    pub fn save(&self) {
        let conn = connection();
        conn.execute(
            "update users set name = $1 where id = $2",
            &[&self.name, &self.id]
        ).unwrap();
    }

    pub fn delete(&self) {
        let conn = connection();
        conn.execute(
            "delete from users where id = $1",
            &[&self.id]
        ).unwrap();
    }
}

fn connection() -> Connection {
    let path = Path::new("tmp/db");
    Connection::open(path).unwrap()
}

pub fn migrate() {
    info!("Run migration!");
    let conn = connection();
    conn.execute("drop table if exists users", &[]).unwrap();
    conn.execute(
        "create table users (
           id integer primary key autoincrement,
           name varchar not null
        )", &[]).unwrap();
}
