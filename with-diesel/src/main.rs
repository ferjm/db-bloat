#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use models::*;
use schema::persons;
use schema::persons::dsl::*;

embed_migrations!("./migrations");

fn main() {
    let connection = SqliteConnection::establish(":memory:")
        .expect("Error connecting to in-memory db");

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    let beatles = vec!["John", "Paul", "George", "Ringo"];
    for beatle in beatles.iter() {
        let person = NewPerson {
            name: beatle
        };
        diesel::insert_into(persons::table)
            .values(&person)
            .execute(&connection)
            .expect("Error saving person");
    }

    let results = persons.load::<Person>(&connection).expect("Error loading persons");

    for person in results {
        println!("Found person {}", person.name);
    }

    let results = persons.filter(name.eq("John"))
                         .limit(1)
                         .load::<Person>(&connection)
                         .expect("Error loading persons");

    for person in results {
        println!("Found person {}", person.name);
    }
}
