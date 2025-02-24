use chrono::{DateTime, Utc};
use duckdb::Connection;
use std::fs;
use std::path::{Path, PathBuf};

static DB_DIR: &'static str = "DATABASE";
static ZIP_DIR: &'static str = "BCK";
static DB_FILE: &'static str = "db.duckdb";

pub fn create_db() {
  let mut create_table = false;
  let db_path: PathBuf = Path::new(DB_DIR).join(DB_FILE);

  if !Path::new(DB_DIR).exists() {
    fs::create_dir(DB_DIR).expect("Si è verificato un errore in fase di creazione della cartella DB_DIR");
  }

  if !Path::new(ZIP_DIR).exists() {
    fs::create_dir(ZIP_DIR).expect("Si è verificato un errore in fase di creazione della cartella ZIP_DIR");
  }

  if !Path::new(&db_path).exists() {
    create_table = true;
  }

  let conn = Connection::open(db_path).unwrap();

  if create_table {
    println!("=====");
    println!("CREAZIONE TABELLE");

    conn.execute_batch(
      r"CREATE TABLE IF NOT EXISTS operation_log (id INTEGER PRIMARY KEY, operation VARCHAR, date VARCHAR);
            CREATE SEQUENCE seq_log_id START 1;"
    )
        .expect("ERRORE NELLA CREAZIONE DELLA TABELLA operation_log");

    conn.execute_batch(
      r"CREATE TABLE IF NOT EXISTS sync (key VARCHAR, value VARCHAR, server VARCHAR);
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
  let db_path: PathBuf = Path::new(DB_DIR).join(DB_FILE);
  let current_utc: DateTime<Utc> = Utc::now();
  let custom_format = current_utc.format("%Y_%m_%d");
  let mut db_copy = "db_".to_owned();
  db_copy.push_str(custom_format.to_string().as_str());
  db_copy.push_str(".duckdb");

  fs::copy(db_path, Path::new(db_copy.to_string().as_str())).unwrap();

  println!("DATABASE COPIATO");
  println!("=====");
}
