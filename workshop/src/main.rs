use std::collections::HashMap;
use std::path::PathBuf;
use std::result::Result;
use std::vec::Vec;
use walkdir::Error;
use walkdir::WalkDir;

type FileHash = HashMap<md5::Digest, Vec<PathBuf>>;

fn main() -> Result<(), Error> {
    let files = get_files()?;
    let hash = find_dupl(files);
    show_dupl(&hash);

    return Ok(());
}

fn get_files() -> Result<Vec<PathBuf>, Error> {
    let mut files: Vec<PathBuf> = Vec::new();
    for f in WalkDir::new(".") {
        files.push(f?.path().to_path_buf());
    }
    return Result::Ok(files);
}

fn find_dupl(files: Vec<PathBuf>) -> FileHash {
    let mut hash = FileHash::new();
    for f in files {
        match std::fs::read(&f) {
            Ok(data) => {
                let digest = md5::compute(data);
                hash.entry(digest).or_default().push(f);
            },
            Err(_) => {},
        }
    }
    return hash;
}

fn show_dupl(hash: &FileHash) {
    for (_digest, eq_files) in hash.iter() {
        if eq_files.len() >= 2 {
            println!("{}", eq_files[0].display());
            for i in 1..(eq_files.len()) {
                println!("    = {}", eq_files[i].display());
            }
        }
    }
}
