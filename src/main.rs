mod db;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::BufRead;

fn main() {
    db::create_db();

    let opzioni = vec!["FTP", "DATABASE", "ESCI"];

    loop {
        let scelta = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Menu")
            .default(0)
            .items(&opzioni)
            .interact()
            .expect("ERRORE NELLA LETTURA DELL'INPUT");

        match scelta {
            0 => gestione_ftp(),
            1 => gestione_db(),
            2 => {
                println!("USCITA...");
                break;
            }
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

fn gestione_db() {
    let opzioni = vec!["LEGGI LOG", "AGGIUNGI SYNC", "VISUALIZZA SYNC", "INDIETRO"];

    loop {
        let scelta = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("GESTIONE SYNC")
            .default(0)
            .items(&opzioni)
            .interact()
            .unwrap();

        match scelta {
            0 => db::read_log(),
            1 => {
                // let mut line = String::new();
                // let stdin = io::stdin();
                // stdin.lock().read_line(&mut line).expect("Could not read line");
                // println!("{}", line)
                db::create_sync("VAL1", "VAL34");
            }
            2 => db::read_sync(),
            3 => break,
            _ => unreachable!(),
        }
    }
}
