use std::fmt;
use std::path::Path;
use duckdb::Connection;
use prettytable::{color, Attr, Cell, Row, Table};
use crate::db;

#[derive(Debug, Clone)]
pub(crate) struct FtpServer {
  pub(crate) name: String,
  pub(crate) host: String,
  pub(crate) username: String,
  pub(crate) password: String,
}

impl fmt::Display for FtpServer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

static DB_FILE: &'static str = "db.duckdb";

pub fn get_all_server() {
  let conn = Connection::open(DB_FILE).unwrap();

  let mut stmt = conn
    .prepare("SELECT name, host, username, password FROM ftp ORDER BY name DESC")
    .unwrap();

  let rows = stmt
    .query_map([], |row| {
      let name: String = row.get(0)?;
      let host: String = row.get(1)?;
      let username: String = row.get(2)?;
      let password: String = row.get(3)?;

      Ok(FtpServer {
        name,
        host,
        username,
        password,
      })
    })
    .unwrap();

  let mut table = Table::new();
  table.add_row(Row::new(vec![
    Cell::new("NOME")
      .with_style(Attr::Bold)
      .with_style(Attr::ForegroundColor(color::RED)),
    Cell::new("HOST")
      .with_style(Attr::Bold)
      .with_style(Attr::ForegroundColor(color::YELLOW)),
    Cell::new("USERNAME")
      .with_style(Attr::Bold)
      .with_style(Attr::ForegroundColor(color::GREEN)),
    Cell::new("PASSWORD")
      .with_style(Attr::Bold)
      .with_style(Attr::ForegroundColor(color::BLUE)),
  ]));

  for row in rows {
    let item = row.unwrap();

    table.add_row(Row::new(vec![
      Cell::new(&item.name),
      Cell::new(&item.host),
      Cell::new(&item.username),
      Cell::new(&item.password),
    ]));
  }

  table.printstd();
  println!("=====");
}

pub fn get_server_by_name(k: &str) -> FtpServer {
  let conn = Connection::open(DB_FILE).unwrap();

  let mut stmt = conn
    .prepare("SELECT name, host, username, password FROM ftp WHERE name = ?")
    .unwrap();

  let (name, host, username, password): (String, String, String, String) = stmt.query_row([k], |row| {
    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
  }).unwrap();

  FtpServer { name, host, username, password }
}

pub fn add_server(name: &str, host: &str, username: &str, password: &str) {
  if !Path::new(&DB_FILE).exists() {
    db::create_db();
  }

  let conn = Connection::open(DB_FILE).unwrap();

  conn.execute(
    "INSERT INTO ftp (name, host, username, password) VALUES (?, ?, ?, ?) ON CONFLICT UPDATE SET username = ?, password = ?",
    [name, host, username, password, username, password],
  )
    .expect("ERRORE DI INSERIMENTO NELLA TABELLA ftp");

  println!("OPERAZIONE AVVENUTA CON SUCCESSO!");
  println!("=====");
}