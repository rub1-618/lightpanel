use std::{fs, path::PathBuf};

use crate::error::{LpnlError, DirInitErrorKind};

// todo: constants

pub fn ensure_lpnl_directories() -> Result<(), LpnlError> {
    // ! /etc/lpnl
    let lpnl_dir = PathBuf::from("/etc/lpnl");
    match &lpnl_dir.exists() {
        true  => {},
        false => {
            match fs::create_dir_all(&lpnl_dir) {
                Ok(_)  => {},
                Err(_) => return Err(LpnlError::DirInitError { 
                    message: "Unable to create lpnl directory.".to_string(),
                    kind: DirInitErrorKind::LpnlCreationFailure 
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
            Err(e) => return Err(LpnlError::DirInitError { 
                message: format!("Unable to create a backup directory: {e}"), 
                kind: DirInitErrorKind::BackupCreationFailure  
            })
        }
    }

    // ! /var/www
    let root_dir = PathBuf::from("/var/www");
    if !&root_dir.exists() {
        match fs::create_dir_all(&root_dir) {
            Ok(_) => {},
            Err(e) => return Err(LpnlError::DirInitError { 
                message: format!("Unable to create a default-root directory: {e}"),
                kind: DirInitErrorKind::RootCreationFailure  
            })
        }
    }

    Ok(println!("All lpnl directories initialized."))
}