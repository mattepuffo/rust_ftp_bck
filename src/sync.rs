use crate::db;
use duckdb::Connection;
use prettytable::{color, Attr, Cell, Row, Table};
use std::path::Path;

#[derive(Debug, Clone)]
struct Sync {
  key: String,
  value: String,
}

static DB_FILE: &'static str = "db.duckdb";

pub fn create_sync(key: &str, value: &str) {
  if !Path::new(&DB_FILE).exists() {
    db::create_db();
  }

  let conn = Connection::open(DB_FILE).unwrap();

  conn.execute(
    "INSERT INTO sync (key, value) VALUES (?, ?) ON CONFLICT DO UPDATE SET value = ?",
    [
      key.to_lowercase(),
      value.to_lowercase(),
      value.to_lowercase(),
    ],
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

    table.add_row(Row::new(vec![Cell::new(&sync.key), Cell::new(&sync.value)]));
  }

  table.printstd();
  println!("=====");
}
