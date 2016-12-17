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

fn select<T: RowMapper>(sql: &str, params: &[&ToSql]) -> Vec<T> {
    let conn = connection();
    let mut stmt = conn.prepare(sql).unwrap();
    let results: Vec<T> = stmt.query_map(params, |row| {
        T::map(row)
    }).unwrap().map(|r| r.unwrap()).collect::<Vec<T>>();
    results
}

fn execute(sql: &str, params: &[&ToSql]) {
    let conn = connection();
    conn.execute(sql, params).unwrap();
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
        let users: Vec<User> = select(
            "select id, name from users", &[]
        );
        users
    }

    pub fn find(id: &str) -> User {
        let mut users: Vec<User> = select(
            "select id, name from users where id = $1 limit 1", &[&id]
        );
        users.pop().unwrap()
    }

    pub fn save(&self) {
        execute(
            "update users set name = $1 where id = $2",
            &[&self.name, &self.id]
        )
    }

    pub fn delete(&self) {
        execute(
            "delete from users where id = $1",
            &[&self.id]
        )
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
