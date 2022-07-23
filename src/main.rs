pub mod store;

fn main() {
    let conn = store::open_db().expect("Cannot connect to database");

    store::create_table(&conn)
        .expect("Couldn't create table");


    store::insert_person(&conn, &store::Person::new("Steven", None))
        .expect("Problems with adding person");


    match store::fetch_people(&conn) {
        Ok(people) => people.into_iter().for_each(move |person| {println!("Hi, I'm {}", person.name)}),
        Err(_) => println!("Problem fetching people")
    }
}