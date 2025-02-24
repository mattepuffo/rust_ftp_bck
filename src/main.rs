mod db;
mod ftp;
mod log;
mod sync;
mod compress;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::{self, Write};
use colored::Colorize;

fn main() {
  db::create_db();

  let opzioni = vec!["FTP", "SYNC", "LOG", "BACKUP DB", "PULISCI BCK", "ESCI"];

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
      4 => compress::delete_file(),
      5 => {
        println!("{}", "USCITA...".yellow());
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
      0 => sync::get_all_sync(),
      1 => {
        println!("{}", "SCRIVI DUE VALORI DEL SYNC".blue());
        println!("{}", "NOME, PATH DA COMPRIMERE E SERVER SEPARATI DAL CARATTERE |".blue());
        println!("{}", "NOTA: CONVIENE PRIMA CREARE IL SERVER E ANNOTARSI IL NOME".yellow());
        println!("{}", "AD ESEMPIO: nome1 | /home/fermat, | server1".yellow());

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split("|");

        let k = parts.next().unwrap_or("").trim();
        let v = parts.next().unwrap_or("").trim();
        let s = parts.next().unwrap_or("").trim();

        sync::create_sync(&*k, &*v, &*s);
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
        println!("{}", "SCRIVI SYNC E FTP SERVER SEPARATI DAL CARATTERE |".blue());
        println!("{}", "AD ESEMPIO: nome1 | server1".yellow());

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split("|");

        let sync_name = parts.next().unwrap_or("").trim();
        let ftp_server = parts.next().unwrap_or("").trim();

        let directory_to_zip = match sync::get_sync_by_key(sync_name) {
          Ok(sync) => sync.value,
          Err(err) => {
            println!("Errore: {}", err.red());
            break;
          }
        };

        let upload_server = match ftp::get_server_by_name(ftp_server) {
          Ok(server) => server.host,
          Err(err) => {
            println!("Errore: {}", err.red());
            break;
          }
        };

        let mut _file_zipped = String::new();
        let res_zip = compress::compress_directory(&directory_to_zip);

        match res_zip {
          Ok(path) => _file_zipped = path,
          Err(e) => println!("Errore: {}", e.red()),
        }

        println!("{}", upload_server);
        log::create_log("documenti");
      }
      1 => {
        ftp::get_all_server()
      }
      2 => {
        println!("{}", "CREA UN SERVER FTP".blue());
        println!("{}", "SCRIVI NOME, HOST, USERNAME, PASSWORD SEPARATI DAL CARATTERE |".blue());
        println!("{}", "AD ESEMPIO: nome1 | 127.0.0.1 | user1 | sdkjfdkjs".yellow());

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