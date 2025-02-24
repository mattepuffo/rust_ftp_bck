use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use duckdb::Connection;
use prettytable::{color, Attr, Cell, Row, Table};

#[derive(Debug, Clone)]
struct OperationLog {
  id: i32,
  operation: String,
  date: String,
}

static DB_DIR: &'static str = "DATABASE";
static DB_FILE: &'static str = "db.duckdb";

pub fn create_log(value: &str) {
  let now: DateTime<Utc> = Utc::now();

  let db_path: PathBuf = Path::new(DB_DIR).join(DB_FILE);
  let conn = Connection::open(db_path).unwrap();

  conn.execute(
    "INSERT INTO operation_log (id, operation, date) VALUES (NEXTVAL('seq_log_id'), ?, ?)",
    [value, now.timestamp().to_string().as_str()],
  )
      .expect("ERRORE DI INSERIMENTO NELLA TABELLA operation_log");

  println!("OPERAZIONE AVVENUTA CON SUCCESSO!");
  println!("=====");
}

pub fn read_log() {
  let db_path: PathBuf = Path::new(DB_DIR).join(DB_FILE);
  let conn = Connection::open(db_path).unwrap();

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