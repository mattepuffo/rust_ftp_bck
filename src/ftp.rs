use std::fmt;
use std::path::{Path, PathBuf};
use duckdb::Connection;
use prettytable::{color, Attr, Cell, Row, Table};

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

static DB_DIR: &'static str = "DATABASE";
static DB_FILE: &'static str = "db.duckdb";

pub fn get_all_server() {
  let db_path: PathBuf = Path::new(DB_DIR).join(DB_FILE);
  let conn = Connection::open(db_path).unwrap();

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

pub fn get_server_by_name(k: &str) -> Result<FtpServer, String> {
  let db_path: PathBuf = Path::new(DB_DIR).join(DB_FILE);
  let conn = Connection::open(db_path)
      .map_err(|e| e.to_string())?;

  let mut stmt = conn
      .prepare("SELECT name, host, username, password FROM ftp WHERE name = ?")
      .map_err(|e| e.to_string())?;

  let mut rows = stmt.query([k])
      .map_err(|e| e.to_string())?;

  if let Some(row) = rows.next().map_err(|e| e.to_string())? {
    Ok(FtpServer {
      name: row.get(0).map_err(|e| e.to_string())?,
      host: row.get(1).map_err(|e| e.to_string())?,
      username: row.get(2).map_err(|e| e.to_string())?,
      password: row.get(3).map_err(|e| e.to_string())?,
    })
  } else {
    Err("Non è stato trovato alcun record".to_string())
  }
}

pub fn add_server(name: &str, host: &str, username: &str, password: &str) {
  let db_path: PathBuf = Path::new(DB_DIR).join(DB_FILE);
  let conn = Connection::open(db_path).unwrap();

  conn.execute(
    "INSERT INTO ftp (name, host, username, password) VALUES (?, ?, ?, ?) ON CONFLICT DO UPDATE SET username = ?, password = ?",
    [name, host, username, password, username, password],
  )
      .expect("ERRORE DI INSERIMENTO NELLA TABELLA ftp");

  println!("OPERAZIONE AVVENUTA CON SUCCESSO!");
  println!("=====");
}