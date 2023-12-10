use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct TestData {
    clients: Option<Vec<ClientTestData>>,
    groups: Option<Vec<GroupTestData>>,
}

impl TestData {
    pub fn new(clients: Option<Vec<ClientTestData>>, groups: Option<Vec<GroupTestData>>) -> Self {
        Self { clients, groups }
    }
}

#[derive(Debug)]
pub struct ClientTestData {
    id: i32,
    comment: String,
    group_ids: Option<Vec<i32>>,
}

impl ClientTestData {
    pub fn new(id: i32, comment: String, group_ids: Option<Vec<i32>>) -> Self {
        Self {
            id,
            comment,
            group_ids,
        }
    }
}

#[derive(Debug)]
pub struct GroupTestData {
    id: i32,
    name: String,
}

impl GroupTestData {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

/// Creates an in-memory SQL database given the test_data.
///
/// # Errors
///
/// This function will return an error if any of the SQL statements fail.
pub fn setup(test_data: TestData) -> Result<Connection> {
    let conn = Connection::open_in_memory()?;

    match conn.execute(
        "CREATE TABLE \"group\" (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL
        )",
        (), // empty list of parameters.
    ) {
        Ok(created) => println!(r#"Created table "group" with {created} rows"#),
        Err(err) => println!(r#"Create table "group" failed: {err}"#),
    }

    match conn.execute(
        "CREATE TABLE client (
            id INTEGER PRIMARY KEY,
            comment TEXT
        )",
        (), // empty list of parameters.
    ) {
        Ok(created) => println!("Created table client with {created} rows"),
        Err(err) => println!("Create table client failed: {err}"),
    }

    match conn.execute(
        r#"CREATE TABLE client_by_group (
            client_id INTEGER NOT NULL REFERENCES client (id),
            group_id INTEGER NOT NULL REFERENCES "group" (id),
            PRIMARY KEY (client_id, group_id)
        )"#,
        (), // empty list of parameters.
    ) {
        Ok(created) => println!("Created table client_by_group with {created} rows"),
        Err(err) => println!("Create table client_by_group failed: {err}"),
    }

    if let Some(groups) = test_data.groups {
        for group in groups {
            match conn.execute(
                r#"INSERT INTO "group" (id, name) VALUES (?1, ?2)"#,
                (group.id, group.name),
            ) {
                Ok(inserted) => println!(r#"{inserted} rows were inserted in table "group""#),
                Err(err) => println!("Insert failed: {err}"),
            }
        }
    }

    if let Some(clients) = test_data.clients {
        for client in clients {
            match conn.execute(
                "INSERT INTO client (id, comment) VALUES (?1, ?2)",
                (client.id, client.comment),
            ) {
                Ok(inserted) => println!("{inserted} rows were inserted in table client"),
                Err(err) => println!("Insert failed: {err}"),
            }

            if let Some(groups) = client.group_ids {
                for group_id in groups {
                    match conn.execute(
                        "INSERT INTO client_by_group (client_id, group_id) VALUES (?1, ?2)",
                        (client.id, group_id),
                    ) {
                        Ok(inserted) => {
                            println!("{inserted} rows were inserted in table client_by_group")
                        }
                        Err(err) => println!("Insert failed: {err}"),
                    }
                }
            }
        }
    }

    Ok(conn)
}

/// Dump all tables to stdout.
pub fn dump(conn: &Connection) -> Result<()> {
    dump_client_table(conn)?;
    dump_group_table(conn)?;
    dump_client_by_group_table(conn)
}

fn dump_client_table(conn: &Connection) -> Result<()> {
    println!(".dump TABLE client");
    let mut stmt = conn.prepare("SELECT id, comment FROM client")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0).unwrap();
        let comment: String = row.get(1).unwrap();
        println!("\t(id: {id}, comment: \"{comment}\")");
    }

    Ok(())
}

fn dump_group_table(conn: &Connection) -> Result<()> {
    println!(r#".dump TABLE "group""#);
    let mut stmt = conn.prepare(r#"SELECT id, name FROM "group""#)?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0).unwrap();
        let name: String = row.get(1).unwrap();
        println!("\t(id: {id}, name: \"{name}\")");
    }

    Ok(())
}

fn dump_client_by_group_table(conn: &Connection) -> Result<()> {
    println!(".dump TABLE client_by_group");
    let mut stmt = conn.prepare("SELECT client_id, group_id FROM client_by_group")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let client_id: i32 = row.get(0).unwrap();
        let group_id: i32 = row.get(1).unwrap();
        println!("\t(client_id: {client_id}, group_id: {group_id})");
    }

    Ok(())
}
