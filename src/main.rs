mod db;
mod ftp;
mod log;
mod sync;
mod compress;

use std::fmt::Display;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::{self, Write};
use std::{thread, time};

fn main() {
  // compress::delete_file("C:\\Users\\user\\AppData\\Local\\Temp\\Documenti.zip").unwrap();
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
      0 => sync::get_all_sync(),
      1 => {
        println!("SCRIVI DUE VALORI DEL SYNC");
        println!("NOME, PATH DA COMPRIMERE E SERVER SEPARATI DAL CARATTERE |");
        println!("NOTA: CONVIENE PRIMA CREARE IL SERVER E ANNOTARSI IL NOME");
        println!("AD ESEMPIO: nome1 | /home/fermat, | server1");

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
        println!("SCRIVI SYNC E FTP SERVER SEPARATI DAL CARATTERE |");
        println!("AD ESEMPIO: nome1 | server1");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split("|");

        // let sync_name = parts.next().unwrap_or("").trim();
        // let ftp_server = parts.next().unwrap_or("").trim();

        let sync_name = "nome1";
        let ftp_server = "nome1";

        let mut directory_to_zip = String::new();
        match sync::get_sync_by_key(ftp_server) {
          Ok(sync) => directory_to_zip = sync.value,
          Err(err) => {
            println!("Errore: {}", err);
            break;
          }
        }

        let mut upload_server = String::new();
        match ftp::get_server_by_name(sync_name) {
          Ok(server) => upload_server = server.host,
          Err(err) => {
            println!("Errore: {}", err);
            break;
          }
        }

        let mut file_zipped = String::new();
        let res_zip = compress::compress_directory("C:\\Personal\\Documenti");
        // let res_zip = compress::compress_directory(&directory_to_zip);

        match res_zip {
          Ok(path) => file_zipped = path,
          Err(e) => println!("Errore: {}", e),
        }

        // println!("DIR TO ZIP: {}", directory_to_zip);
        // println!("UPLOAD SERVER: {}", upload_server);

        thread::sleep(time::Duration::from_millis(5));
        compress::delete_file(&*file_zipped).unwrap();
      }
      1 => {
        ftp::get_all_server()
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