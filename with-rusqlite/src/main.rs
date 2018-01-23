extern crate rusqlite;

use rusqlite::Connection;

struct Person {
    pub id: i32,
    pub name: String,
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Yo".to_string()
    };
    conn.execute("INSERT INTO person (name) VALUES (?1)",
                 &[&me.name]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap().name);
    }
}

