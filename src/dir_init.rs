use std::{fs, path::PathBuf};

use crate::error::DirInitializationError;

// todo: constants

pub fn ensure_lpnl_directories() -> Result<(), DirInitializationError> {
    // ! /etc/lpnl
    let lpnl_dir = PathBuf::from("/etc/lpnl");
    match &lpnl_dir.exists() {
        true  => {},
        false => {
            match fs::create_dir_all(&lpnl_dir) {
                Ok(_)  => {},
                Err(_) => return Err(DirInitializationError { 
                    message: "Unable to create lpnl directory.".to_string() 
                })
            }
        }
    }

    // ! /etc/lpnl/backups
    let mut backup_dir = PathBuf::from("/etc/lpnl");
    backup_dir.push("backups");
    if !&backup_dir.exists() {
        match fs::create_dir_all(&backup_dir) {
            Ok(_) => {},
            Err(e) => return Err(DirInitializationError { 
                message: format!("Unable to create a backup directory: {e}")
            })
        }
    }

    // ! /var/www
    let root_dir = PathBuf::from("/var/www");
    if !&root_dir.exists() {
        match fs::create_dir_all(&root_dir) {
            Ok(_) => {},
            Err(e) => return Err(DirInitializationError { 
                message: format!("Unable to create a default-root directory: {e}")
            })
        }
    }

    Ok(println!("All lpnl directories initialized."))
}