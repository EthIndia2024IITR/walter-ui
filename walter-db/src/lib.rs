use rusqlite::{params, Connection, Result};
use rustyline::error::ReadlineError;
use rustyline::Editor;
mod walrus_io;

pub fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <blobID>", args[0]);
        std::process::exit(1);
    }
    std::fs::File::create("/tmp/sqlite.db").expect("Unable to create file");
    let blob_id = &args[1];
    let blob_id = blob_id.to_string();
    let mut blob_id =
        match walrus_io::download_and_extract_id(blob_id.clone(), "/tmp/sqlite.db".to_string()) {
            Some(blob_id) => blob_id,
            None => "".to_string(),
        };
    let conn = Connection::open("/tmp/sqlite.db")?;
    let mut rl = Editor::<(), _>::new()?;

    println!(
        "\x1b[1;34mWalruSQL - SQLite on Walrus with rollbacks\x1b[0m\n\x1b[1;32mType 'help' for available commands\x1b[0m\n\x1b[1;31mType 'exit' or 'quit' to close the shell\x1b[0m"
    );

    loop {
        let readline = rl.readline("\x1b[1;33msqlite>\x1b[0m ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match line.trim() {
                    "exit" | "quit" => break,
                    "help" => print_help(),
                    "tables" => list_tables(&conn)?,
                    cmd if cmd.starts_with("describe ") => {
                        let table_name = cmd.split_whitespace().nth(1).unwrap_or("");
                        describe_table(&conn, table_name)?;
                    }
                    query if !query.is_empty() => {
                        execute_query(&conn, query, &mut blob_id)?;
                    }
                    _ => {}
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("  help     - Show this help message");
    println!("  tables   - List all tables in the database");
    println!("  describe <table> - Show table schema");
    println!("  SQL queries will be executed");
    println!("  exit/quit - Close the shell");
}

fn list_tables(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table';")?;
    let table_names = stmt
        .query_map(params![], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;

    println!("Tables:");
    for table in table_names {
        println!("  - {}", table);
    }
    Ok(())
}

fn describe_table(conn: &Connection, table_name: &str) -> Result<()> {
    if table_name.is_empty() {
        println!("Please provide a table name. Usage: describe <table_name>");
        return Ok(());
    }

    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;

    println!("Schema for table: {}", table_name);
    println!("  Name\t\tType\t\tNullable\tPrimary Key");
    println!("  {}", "-".repeat(50));

    let column_info = stmt.query_map(params![], |row| {
        Ok((
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i32>(3)?,
            row.get::<_, i32>(5)?,
        ))
    })?;

    for info in column_info {
        let (name, type_name, nullable, primary_key) = info?;
        println!(
            "  {}\t\t{}\t\t{}\t\t{}",
            name,
            type_name,
            if nullable == 0 { "Yes" } else { "No" },
            if primary_key == 1 { "Yes" } else { "No" }
        );
    }

    Ok(())
}

fn execute_query(conn: &Connection, query: &str, blob_id_prev: &mut String) -> Result<()> {
    if query.trim().to_uppercase().starts_with("SELECT") {
        let mut stmt = conn.prepare(query)?;

        // Get column names
        let column_names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Print headers
        println!("{}", column_names.join("\t"));

        // Print rows
        let mut rows = stmt.query(params![])?;
        let mut row_count = 0;

        while let Some(row) = rows.next()? {
            let mut row_data = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let value_ref = row.get_ref(i)?;
                let value_str = match value_ref {
                    rusqlite::types::ValueRef::Null => "NULL".to_string(),
                    rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                    rusqlite::types::ValueRef::Real(r) => r.to_string(),
                    rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                    rusqlite::types::ValueRef::Blob(b) => format!("{:?}", b),
                };
                row_data.push(value_str);
            }
            println!("{}", row_data.join("\t"));
            row_count += 1;
        }

        println!("\nRows returned: {}", row_count);
    } else if query.trim().to_uppercase().starts_with("SAVE") {
        let value = walrus_io::append_id_and_upload(
            "/tmp/sqlite.db".to_string(),
            blob_id_prev.clone(),
            Some(1),
        );
        *blob_id_prev = walrus_io::get_blob_id(value.unwrap()).unwrap();
        println!("CURRENT Blob ID: {}", blob_id_prev);
    } else if query.trim().to_uppercase().starts_with("ROLLBACK") {
        let value =
            walrus_io::download_and_extract_id(blob_id_prev.clone(), "/tmp/sqlite.db".to_string());
        // *blob_id_prev = value.unwrap();
        println!("Blob ID on ROLLBACK: {}", value.unwrap());
    } else {
        // For non-SELECT queries (INSERT, UPDATE, DELETE)
        match conn.execute(query, params![]) {
            Ok(rows_affected) => println!("Query executed. {} row(s) affected.", rows_affected),
            Err(err) => println!("Failed to execute query: {}", err),
        }
    }

    Ok(())
}