mod db;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use rand::distributions::Alphanumeric;
use rand::Rng;

fn main() {
    let opzioni = vec!["FTP", "DATABASE", "ESCI"];

    loop {
        let scelta = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Menu")
            .default(0)
            .items(&opzioni)
            .interact()
            .expect("Errore nella lettura dell'input");

        match scelta {
            0 => gestione_ftp(),
            1 => gestione_db(),
            2 => {
                println!("Uscita...");
                break;
            }
            _ => unreachable!(),
        }
    }
}

fn gestione_ftp() {
    let opzioni = vec!["ESEGUI BCK", "INDIETRO"];

    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    loop {
        let scelta = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("GESTIONE FTP")
            .default(0)
            .items(&opzioni)
            .interact()
            .unwrap();

        match scelta {
            0 => db::create_log(&*s),
            1 => break,
            _ => unreachable!(),
        }
    }
}

fn gestione_db() {
    let opzioni = vec!["CREA DB", "LEGGi LOG", "INDIETRO"];

    loop {
        let scelta = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("GESTIONE DB")
            .default(0)
            .items(&opzioni)
            .interact()
            .unwrap();

        match scelta {
            0 => db::create_db(),
            1 => db::read_db(),
            2 => break,
            _ => unreachable!(),
        }
    }
}
