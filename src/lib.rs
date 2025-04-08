use log::{debug, error, trace};
use rusqlite::{Connection, Result};

pub fn append(conn: &Connection, client_comment: &str, group_name: &str) -> Result<()> {
    trace!("append group: '{group_name}' to client: '{client_comment}'");

    let client_id = get_client_id(conn, client_comment)?;
    let group_id = get_group_id(conn, group_name)?;
    insert(conn, client_id, group_id)
}

pub fn remove(conn: &Connection, client_comment: &str, group_name: &str) -> Result<()> {
    trace!("remove group: '{group_name}' from client: '{client_comment}'");

    let client_id = get_client_id(conn, client_comment)?;
    let group_id = get_group_id(conn, group_name)?;
    delete(conn, client_id, group_id)
}

fn get_group_id(conn: &Connection, name: &str) -> Result<i32> {
    trace!(r#"get_group_id via name: "{name}""#);

    let mut stmt = conn.prepare(r#"SELECT id FROM "group" WHERE name = ?1"#)?;
    stmt.query_row([name], |row| row.get::<_, i32>(0))
}

fn get_client_id(conn: &Connection, comment: &str) -> Result<i32> {
    trace!(r#"get_client_id via comment: "{comment}""#);

    let mut stmt = conn.prepare("SELECT id FROM client WHERE comment = ?1")?;
    stmt.query_row([comment], |row| row.get::<_, i32>(0))
}

fn insert(conn: &Connection, client_id: i32, group_id: i32) -> Result<()> {
    trace!("insert (client_id: {client_id}, group_id: {group_id}) into client_by_group table");

    let mut stmt =
        conn.prepare("INSERT INTO client_by_group (client_id, group_id) VALUES (?1, ?2)")?;
    match stmt.execute((client_id, group_id)) {
        Ok(inserted) => {
            debug!("{} rows were inserted in table client_by_group", inserted);
            Ok(())
        }
        Err(error) => {
            error!("Insert failed: {}", error);
            Err(error)
        }
    }
}

fn delete(conn: &Connection, client_id: i32, group_id: i32) -> Result<()> {
    trace!("delete (client_id: {client_id}, group_id: {group_id}) from client_by_group table");

    let mut stmt =
        conn.prepare("DELETE FROM client_by_group WHERE client_id = ?1 AND group_id = ?2")?;
    match stmt.execute((client_id, group_id)) {
        Ok(inserted) => {
            debug!("{inserted} rows were deleted in table client_by_group");
            Ok(())
        }
        Err(error) => {
            error!("Delete failed: {error}");
            Err(error)
        }
    }
}
