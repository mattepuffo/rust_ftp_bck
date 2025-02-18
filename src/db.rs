use chrono::{DateTime, Utc};
use duckdb::Connection;
use prettytable::{color, Attr, Cell, Row, Table};
use std::path::Path;

#[derive(Debug, Clone)]
struct OperationLog {
    id: i32,
    operation: String,
    date: String,
}

#[derive(Debug, Clone)]
struct Sync {
    key: String,
    value: String,
}

static DB_FILE: &'static str = "db.duckdb";

pub fn create_db() {
    let mut create_table = false;

    if !Path::new(&DB_FILE).exists() {
        create_table = true;
    }

    let conn = Connection::open(DB_FILE).unwrap();

    if create_table {
        println!("=====");
        println!("CREAZIONE TABELLE");

        conn.execute_batch(
        r"CREATE TABLE IF NOT EXISTS operation_log (id INTEGER PRIMARY KEY, operation VARCHAR, date VARCHAR);
            CREATE SEQUENCE seq_log_id START 1;"
      )
          .expect("ERRORE NELLA CREAZIONE DELLA TABELLA operation_log");

        conn.execute_batch(
            r"CREATE TABLE IF NOT EXISTS sync (key VARCHAR, value VARCHAR);;
            CREATE UNIQUE INDEX kv_idx ON sync (key);",
        )
        .expect("ERRORE NELLA CREAZIONE DELLA TABELLA sync");

        println!("TABELLE CREATE");
        println!("=====");
    } else {
        println!("=====");
        println!("DATABASE ESISTENTE");
        println!("=====");
    }
}

pub fn create_log(value: &str) {
    let now: DateTime<Utc> = Utc::now();

    if !Path::new(&DB_FILE).exists() {
        create_db();
    }

    let conn = Connection::open(DB_FILE).unwrap();

    conn.execute(
        "INSERT INTO operation_log (id, operation, date) VALUES (NEXTVAL('seq_log_id'), ?, ?)",
        [value, now.timestamp().to_string().as_str()],
    )
    .expect("ERRORE DI INSERIMENTO NELLA TABELLA operation_log");

    println!("OPERAZIONE AVVENUTA CON SUCCESSO!");
    println!("=====");
}

pub fn read_log() {
    let conn = Connection::open(DB_FILE).unwrap();

    let mut stmt = conn
        .prepare("SELECT id, operation, date FROM operation_log ORDER BY id DESC")
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            let operation: String = row.get(1)?;
            let date: String = row.get(2)?;

            let timestamp = DateTime::from_timestamp(date.parse().unwrap(), 0).unwrap();
            let formatted_date = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

            Ok(OperationLog {
                id,
                operation,
                date: formatted_date,
            })
        })
        .unwrap();

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new("OPERAZIONE")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::YELLOW)),
        Cell::new("DATA")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));

    for row in rows {
        let operation_log = row.unwrap();

        table.add_row(Row::new(vec![
            Cell::new(&operation_log.id.to_string()),
            Cell::new(&operation_log.operation),
            Cell::new(&operation_log.date),
        ]));
    }

    table.printstd();
    println!("=====");
}

pub fn create_sync(key: &str, value: &str) {
    if !Path::new(&DB_FILE).exists() {
        create_db();
    }

    let conn = Connection::open(DB_FILE).unwrap();

    conn.execute(
        "INSERT INTO sync (key, value) VALUES (?, ?) ON CONFLICT DO UPDATE SET value = ?",
        [key, value, value],
    )
    .expect("ERRORE DI INSERIMENTO NELLA TABELLA sync");

    println!("OPERAZIONE AVVENUTA CON SUCCESSO!");
    println!("=====");
}

pub fn read_sync() {
    let conn = Connection::open(DB_FILE).unwrap();

    let mut stmt = conn
        .prepare("SELECT key, value FROM sync ORDER BY key ASC")
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;

            Ok(Sync { key, value })
        })
        .unwrap();

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("CHIAVE")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::YELLOW)),
        Cell::new("VALORE")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));

    for row in rows {
        let sync = row.unwrap();

        table.add_row(Row::new(vec![
            Cell::new(&sync.key),
            Cell::new(&sync.value),
        ]));
    }

    table.printstd();
    println!("=====");
}
