use std::fs::OpenOptions;
use std::io::ErrorKind;
use crate::usefull_fun::error_output::error_output;

pub fn check_removing_db(path: &str) {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path) 
    {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
        Err(e) => {
            error_output(&format!("Unknown error. Error text: {}", e));
        }
    }
}