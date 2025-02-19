mod db;
mod ftp;
mod log;
mod sync;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::{self, Write};

fn main() {
  db::create_db();

  let opzioni = vec!["FTP", "SYNC", "LOG", "BACKUP DB", "ESCI"];

  loop {
    let scelta = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("Menu")
      .default(0)
      .items(&opzioni)
      .interact()
      .expect("ERRORE NELLA LETTURA DELL'INPUT");

    match scelta {
      0 => gestione_ftp(),
      1 => gestione_sync(),
      2 => gestione_log(),
      3 => db::copy_db(),
      4 => {
        println!("USCITA...");
        break;
      }
      _ => unreachable!(),
    }
  }
}

fn gestione_sync() {
  let opzioni = vec!["VISUALIZZA SYNC", "AGGIUNGI SYNC", "INDIETRO"];

  loop {
    let scelta = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("GESTIONE SYNC")
      .default(0)
      .items(&opzioni)
      .interact()
      .unwrap();

    match scelta {
      0 => sync::read_sync(),
      1 => {
        println!("SCRIVI DUE VALORI DEL SYNC");
        println!("NOME E PATH DA COMPRIMERE SEPARATI DAL CARATTERE |");
        println!("AD ESEMPIO: nome1 | /home/fermat");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // let mut parts = input.trim().split_whitespace();
        let mut parts = input.trim().split("|");

        let k = parts.next().unwrap_or("").trim();
        let v = parts.next().unwrap_or("").trim();

        sync::create_sync(&*k, &*v);
      }
      2 => break,
      _ => unreachable!(),
    }
  }
}

fn gestione_log() {
  let opzioni = vec!["LEGGI LOG", "CANCELLA LOG", "INDIETRO"];

  loop {
    let scelta = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("GESTIONE LOG")
      .default(0)
      .items(&opzioni)
      .interact()
      .unwrap();

    match scelta {
      0 => log::read_log(),
      1 => println!("CENCELLLAZIONE LOG"),
      2 => break,
      _ => unreachable!(),
    }
  }
}

fn gestione_ftp() {
  let opzioni = vec!["ESEGUI BCK", "VISUALIZZA SERVER", "AGGIUNGI SERVER", "INDIETRO"];

  loop {
    let scelta = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("GESTIONE FTP")
      .default(0)
      .items(&opzioni)
      .interact()
      .unwrap();

    match scelta {
      0 => {
        exec_bck()
      }
      1 => {
        ftp::read_server()
      }
      2 => {
        println!("CREA UN SERVER FTP");
        println!("SCRIVI NOME, HOST, USERNAME, PASSWORD SEPARATI DAL CARATTERE |");
        println!("AD ESEMPIO: nome1 | 127.0.0.1 | user1 | sdkjfdkjs");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split("|");

        let name = parts.next().unwrap_or("").trim();
        let host = parts.next().unwrap_or("").trim();
        let username = parts.next().unwrap_or("").trim();
        let password = parts.next().unwrap_or("").trim();

        ftp::add_server(&*name, &*host, &*username, &*password);
      }
      3 => break,
      _ => unreachable!(),
    }
  }
}

fn exec_bck() {
  let mut opzioni = vec![];
  let mut list: Vec<ftp::FtpServer> = ftp::list_server();

  for item in list.iter_mut() {
    opzioni.push(item.name.clone());
  }

  loop {
    let scelta = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("SCEGLI SERVER")
      .default(0)
      .items(&opzioni)
      .interact()
      .unwrap();
  }
}