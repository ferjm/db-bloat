use super::schema::persons;

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[table_name = "persons"]
pub struct NewPerson<'a> {
    pub name: &'a str,
    pub data: Option<Vec<u8>>,
}
