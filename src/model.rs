extern crate postgres;

use postgres::{Connection, TlsMode};

pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>,
}

pub fn CPerson() -> Vec<Person> {
    let conn = Connection::connect("postgres://jude@localhost:5432/trust", TlsMode::None).unwrap();
    let mut persons: Vec<Person> = Vec::new();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let p = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        persons.push(p);
    }

    persons
}

fn create() {
    let conn = Connection::connect("postgres://jude@localhost:5432/trust", TlsMode::None).unwrap();
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();
}