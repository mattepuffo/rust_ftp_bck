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

static DB_FILE: &'static str = "db.duckdb";

pub fn create_db() {
    let mut create_table = false;

    if !Path::new(&DB_FILE).exists() {
        create_table = true;
    }

    let conn = Connection::open(DB_FILE).unwrap();

    if create_table {
        println!("Creating table");
        conn.execute_batch(
        r"CREATE TABLE IF NOT EXISTS operation_log (id INTEGER PRIMARY KEY, operation VARCHAR, date VARCHAR);
            CREATE SEQUENCE seq_id START 1;"
      )
          .unwrap();
    }
}

pub fn create_log(valore: &str) {
    let now: DateTime<Utc> = Utc::now();

    if !Path::new(&DB_FILE).exists() {
        create_db();
    }

    let conn = Connection::open(DB_FILE).unwrap();

    conn.execute(
        "INSERT INTO operation_log (id, operation, date) VALUES (NEXTVAL('seq_id'), ?, ?)",
        [valore, now.timestamp().to_string().as_str()],
    )
    .unwrap();
}

pub fn read_db() {
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
        Cell::new("OPERATION")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::YELLOW)),
        Cell::new("DATE")
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
}
