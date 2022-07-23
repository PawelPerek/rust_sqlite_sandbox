use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: Option<String>,
}

impl Person {
    pub fn new(name: &str, data: Option<String>) -> Person {
        Person { id: 0, name: name.to_string(),  data }
    } 
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

pub fn insert_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&person.name, &person.data),
    )?;

    Ok(())
}

pub fn fetch_people(conn: &Connection) -> Result<Vec<Person>> {
    let mut statement = conn.prepare("SELECT id, name, data FROM person")?;
    
    let person_iter: Vec<Person> = statement
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?
        .map(|res| res.expect("Problems with query"))
        .collect();

    Ok(person_iter)
}