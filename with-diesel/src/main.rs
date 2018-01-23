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

    let me = NewPerson {
        name: "Yo",
        data: None
    };

    diesel::insert_into(persons::table)
        .values(&me)
        .execute(&connection)
        .expect("Error saving person");

    let results = persons.load::<Person>(&connection).expect("Error loading persons");

    for person in results {
        println!("Found person {:?}", person.name);
    }
}
