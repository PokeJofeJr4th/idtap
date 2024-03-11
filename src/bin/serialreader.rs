use rusqlite::{Connection, Result};
use std::io::{self, BufRead};

fn lookup_name_by_serial(serial: i32, conn: &Connection) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT name FROM people WHERE serial_number = ?")?;
    let mut rows = stmt.query([serial])?;

    if let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        Ok(Some(name))
    } else {
        Ok(None)
    }
}

fn main() -> Result<()> {
    // Replace "your_database.sqlite" with the actual path to your SQLite database file.
    let conn = Connection::open("your_database.sqlite")?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(serial_str) = line {
            if let Ok(serial) = serial_str.trim().parse::<i32>() {
                match lookup_name_by_serial(serial, &conn)? {
                    Some(name) => println!("Name for serial number {}: {}", serial, name),
                    None => println!("No name found for serial number: {}", serial),
                }
            } else {
                eprintln!("Invalid serial number format: {}", serial_str);
            }
        }
    }

    Ok(())
}
