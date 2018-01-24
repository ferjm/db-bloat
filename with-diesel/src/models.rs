use super::schema::persons;

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name = "persons"]
pub struct NewPerson<'a> {
    pub name: &'a str
}
