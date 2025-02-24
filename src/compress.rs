use std::{env, fs, io};
use std::path::{Path, PathBuf};
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

pub fn delete_file(file_path: &str) -> io::Result<()> {
  match fs::remove_file(file_path) {
    Ok(_) => Ok(()),
    Err(e) => {
      eprintln!("Errore durante la cancellazione del file: {}", e);
      Err(e)
    }
  }
}