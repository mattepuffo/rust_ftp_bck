use crate::db;
use duckdb::Connection;
use prettytable::{color, Attr, Cell, Row, Table};
use std::path::Path;

#[derive(Debug, Clone)]
pub(crate) struct Sync {
  pub(crate) key: String,
  pub(crate) value: String,
  pub(crate) server: String,
}

static DB_FILE: &'static str = "db.duckdb";

pub fn get_all_sync() {
  let conn = Connection::open(DB_FILE).unwrap();

  let mut stmt = conn
      .prepare("SELECT key, value, server FROM sync ORDER BY key ASC")
      .unwrap();

  let rows = stmt
      .query_map([], |row| {
        let key: String = row.get(0)?;
        let value: String = row.get(1)?;
        let server: String = row.get(2)?;

        Ok(Sync { key, value, server })
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
    Cell::new("SERVER")
        .with_style(Attr::Bold)
        .with_style(Attr::ForegroundColor(color::YELLOW)),
  ]));

  for row in rows {
    let sync = row.unwrap();

    table.add_row(Row::new(vec![
      Cell::new(&sync.key),
      Cell::new(&sync.value),
      Cell::new(&sync.server)
    ]));
  }

  table.printstd();
  println!("=====");
}

pub fn get_sync_by_key(k: &str) -> Result<Sync, String> {
  let conn = Connection::open(DB_FILE)
      .map_err(|e| e.to_string())?;

  let mut stmt = conn
      .prepare("SELECT key, value, server FROM sync WHERE key = ?")
      .map_err(|e| e.to_string())?;

  let mut rows = stmt.query([k])
      .map_err(|e| e.to_string())?;

  if let Some(row) = rows.next().map_err(|e| e.to_string())? {
    Ok(Sync {
      key: row.get(0).map_err(|e| e.to_string())?,
      value: row.get(1).map_err(|e| e.to_string())?,
      server: row.get(2).map_err(|e| e.to_string())?,
    })
  } else {
    Err("Non è stato trovato alcun record".to_string())
  }
}

pub fn create_sync(key: &str, value: &str, server: &str) {
  if !Path::new(&DB_FILE).exists() {
    db::create_db();
  }

  let conn = Connection::open(DB_FILE).unwrap();

  conn.execute(
    "INSERT INTO sync (key, value, server) VALUES (?, ?, ?) ON CONFLICT DO UPDATE SET value = ?, server = ?",
    [
      key.to_lowercase(),
      value.to_lowercase(),
      server.to_lowercase(),
      value.to_lowercase(),
      server.to_lowercase(),
    ],
  )
      .expect("ERRORE DI INSERIMENTO NELLA TABELLA sync");

  println!("OPERAZIONE AVVENUTA CON SUCCESSO!");
  println!("=====");
}