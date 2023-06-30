use std::fs;
use std::io::Error;
use std::path::Path;


pub fn path_exists(name: &str) -> bool {
    Path::new(name).exists()
}

pub fn path_is_file(name:&str) -> bool {
    Path::new(name).is_file()
}

pub fn path_is_dir(name:&str) -> bool {
    Path::new(name).is_dir()
}

pub fn path_get_size(name:&str) -> Result<u64, Error> {
    Ok(fs::metadata(Path::new(name))?.len())
}

pub fn rename_file(from:&str, to:&str) -> Result<bool, Error> {
    match fs::rename(from, to) {
        Err(what) => Err(what),
        Ok(_) => Ok(true),
    }
}