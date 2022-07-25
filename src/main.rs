use rusqlite::{Connection, Result};
use std::io;

#[derive(Debug)]
struct Note {
    id: i32,
    body: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "create table if not exists notes (
      id integer primary key,
      body text not null unique
    )",
        [],
    )?;

    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer)?;

    // INSERT
    // conn.execute("INSERT INTO notes (body) values (?1)", [buffer])?;

    // SELECT
    let mut stmt = conn.prepare("SELECT * FROM notes WHERE id=:id;")?;

    let notes_vec = stmt
        .query_map(&[(":id", "5")], |row| {
            Ok(Note {
                id: row.get(0)?,
                body: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<Note>, rusqlite::Error>>()?;

    for person in notes_vec {
        println!("Found person {:?}", person.body);
    }

    Ok(())
}
