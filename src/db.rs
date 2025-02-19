use chrono::{DateTime, Utc};
use duckdb::Connection;
use std::fs;
use std::path::Path;

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
      r"CREATE TABLE IF NOT EXISTS sync (key VARCHAR, value VARCHAR);
            CREATE UNIQUE INDEX kv_idx ON sync (key);",
    )
      .expect("ERRORE NELLA CREAZIONE DELLA TABELLA sync");

    conn.execute_batch(
      r"CREATE TABLE IF NOT EXISTS ftp (name VARCHAR, host VARCHAR, username VARCHAR, password VARCHAR);
            CREATE UNIQUE INDEX ftp_idx ON ftp (name);",
    )
      .expect("ERRORE NELLA CREAZIONE DELLA TABELLA ftp");

    println!("TABELLE CREATE");
    println!("=====");
  } else {
    println!("=====");
    println!("DATABASE ESISTENTE");
    println!("=====");
  }
}

pub fn copy_db() {
  let current_utc: DateTime<Utc> = Utc::now();
  let custom_format = current_utc.format("%Y_%m_%d");
  let mut db_copy = "db_".to_owned();
  db_copy.push_str(custom_format.to_string().as_str());
  db_copy.push_str(".duckdb");

  fs::copy(DB_FILE, Path::new(db_copy.to_string().as_str())).unwrap();

  println!("DATABASE COPIATO");
  println!("=====");
}
