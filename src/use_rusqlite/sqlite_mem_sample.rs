use rusqlite::{Connection, Result, Statement};

#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn db_conn() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    println!("get conn ok");
    Ok(conn)
}

pub fn db_create(conn: &Connection) -> Result<usize> {
    let ret = conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    println!("db create ok");
    Ok(ret)
}

pub fn db_insert(conn: &Connection) -> Result<usize> {
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    let ret = conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;
    println!("db insert ok. {:?}", me);
    Ok(ret)
}

pub fn db_select(conn: &Connection) -> Vec<Person> {
    let mut people: Vec<Person> = Vec::new();
    let mut stmt: Statement;
    let mut _stmt = conn.prepare("SELECT id, name, data FROM person");
    match _stmt {
        Ok(s) => {
            stmt = s;
        }
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return people;
        }
    }

    let _person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    });
    match _person_iter {
        Ok(s) => {
            for person in s {
                if let Ok(value) = person {
                    people.push(value);
                } else {
                    println!("persion_iter error.");
                }
            }
        }
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return people;
        }
    }

    people
}



pub fn run_sqlite_mem_sample() {
    let conn: Connection;

    let result = db_conn();
    match result {
        Ok(_conn) => {
            conn = _conn;
        }
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return;
        }
    }

    let _ = db_create(&conn);
    let _ = db_insert(&conn);
    let result = db_select(&conn);
    for person in result {
        println!("db select ok. {:?}", person);
    }
}
