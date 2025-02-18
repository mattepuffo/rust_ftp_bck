mod db;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::{self, Write};

fn main() {
    db::create_db();

    let opzioni = vec!["FTP", "SYNC", "LOG", "ESCI"];

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
            3 => {
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
            0 => db::read_sync(),
            1 => {
                println!(
                    "SCRIVI DUE VALORI DEL SYNC - NOME E PATH DA COMPRIMERE SEPARATI DA UNO SPAZIO"
                );
                println!("AD ESEMPIO: nome1 /home/fermat");

                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let mut parts = input.trim().split_whitespace();

                let k = parts.next().unwrap_or("");
                let v = parts.next().unwrap_or("");

                // println!("Valore 1: {}", k);
                // println!("Valore 2: {}", v);

                db::create_sync(&*k, &*v);
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
            0 => db::read_log(),
            1 => println!("CENCALLLAZIONE LOG"),
            2 => break,
            _ => unreachable!(),
        }
    }
}

fn gestione_ftp() {
    let opzioni = vec!["ESEGUI BCK", "INDIETRO"];

    loop {
        let scelta = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("GESTIONE FTP")
            .default(0)
            .items(&opzioni)
            .interact()
            .unwrap();

        match scelta {
            0 => {
                println!("FTP");
                // db::create_log("LOG");
            }
            1 => break,
            _ => unreachable!(),
        }
    }
}
