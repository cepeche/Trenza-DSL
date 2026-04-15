use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("locks.db")?;
    let mut stmt = conn.prepare("SELECT sender, recipient, subject, body, created_at FROM messages")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?
        ))
    })?;
    for row in rows {
        let (sender, recipient, subject, body, created_at) = row?;
        println!("--- MESSAGE ---");
        println!("FROM: {}, TO: {}, SUBJ: {}, AT: {}", sender, recipient, subject, created_at);
        println!("BODY: {}", body);
    }
    Ok(())
}
