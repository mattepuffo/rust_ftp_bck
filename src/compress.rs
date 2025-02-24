use std::path::PathBuf;
use std::fs::{read_dir, remove_file};
use zip_archive::Archiver;

static ZIP_DIR: &'static str = "BCK";

pub fn compress_directory(dir: &str) -> Result<String, String> {
  let dir_to_compress = PathBuf::from(dir);
  let dest = PathBuf::from(ZIP_DIR);
  let thread_count = 8;

  let mut archiver = Archiver::new();
  archiver.push(dir_to_compress);
  archiver.set_destination(dest.clone());
  archiver.set_thread_count(thread_count);

  match archiver.archive() {
    Ok(_) => Ok(dest.to_string_lossy().into_owned()),
    Err(e) => Err(format!("Errore durante la compressione: {}", e)),
  }
}

pub fn delete_file() {
  let entries: Vec<_> = read_dir(ZIP_DIR)
      .expect("Errore nella lettura della cartella")
      .collect::<Result<_, _>>()
      .expect("Errore nell'iterazione della cartella");

  if !entries.is_empty() {
    for entry in entries {
      let path = entry.path();

      if path.is_file() {
        remove_file(&path).expect("Errore nell'eliminazione del file");
      }
    }
  }
}