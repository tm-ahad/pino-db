use std::fs::{OpenOptions, File};
use crate::err::{Error, ErrorType};

pub fn open_file(name: String) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(name.clone())
        .unwrap_or_else(|e| {
            Error::throw(
                ErrorType::OS, 
                &format!("{}: {}", name, e)
            );
            todo!()
        })
}
