use crate::constants::{LPNL_BACKUP_DIR, NGINX_SITES_ENABLED_DIR, LPNL_TMP_DIR};
use crate::error::{LpnlError, BackupErrorKind};
use crate::commands::{proceed_nginx_with_dir, proceed_check_nginx, proceed_nginx};
use crate::validation::get_domain;
use std::{fs, path::PathBuf};

pub fn set_backup(domain: Option<String>) -> Result<(), LpnlError> {
    let domain = get_domain(domain)?;

    let backup_dir_str = format!("{LPNL_BACKUP_DIR}/{domain}/{domain}.txt");
    let nginx_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");

    let backup_dir = PathBuf::from(&backup_dir_str);
    let nginx_dir = PathBuf::from(&nginx_dir_str);
    if !backup_dir.exists() || !nginx_dir.exists() {
        return Err(LpnlError::BackupError { 
            message: format!("Unable to find '{domain}' files. Suggest using setup or init."),
            kind: BackupErrorKind::NotFound
        }) 
    }

    // * testing + running
    proceed_check_nginx()?;

    let contents = match fs::read_to_string(nginx_dir_str) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::BackupError { 
            message: format!("Unable to read config file: {e}"),
            kind: BackupErrorKind::FsFailure
        }) 
    };

    match fs::write(&backup_dir_str, contents) {
        Ok(_) => Ok(println!("File saved to '{backup_dir_str}' successfully.")),
        Err(e) => return Err(LpnlError::BackupError { 
            message: format!("Unable to write backup file: {e}"),
            kind: BackupErrorKind::FsFailure
        }) 
    }
}

pub fn get_backup(domain: Option<String>) -> Result<(), LpnlError> {
    let domain = get_domain(domain)?;

    let backup_dir_str = format!("{LPNL_BACKUP_DIR}/{domain}/{domain}.txt");
    let nginx_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");

    let old_config_contents = match fs::read_to_string(&nginx_dir_str) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::BackupError { 
            message: format!("Unable to read backup file: {e}"),
            kind: BackupErrorKind::FsFailure
        }) 
    };

    let contents = match fs::read_to_string(&backup_dir_str) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::BackupError { 
            message: format!("Unable to read backup file: {e}"),
            kind: BackupErrorKind::FsFailure
        }) 
    };

    // * testing
    let test_file = format!("{LPNL_TMP_DIR}/run_test.txt");
    let test_contents = format!("events {{  }} http {{ {contents} }}");
    match fs::write(&test_file, test_contents) {
        Ok(_) => {},
        Err(e) => return Err(LpnlError::BackupError { 
            message: format!("Unable to setup a test file: {e}"),
            kind: BackupErrorKind::FsFailure
        }) 
    }

    match proceed_nginx_with_dir(&test_file) {
        Ok(_) => {
            match fs::remove_file(&test_file) {
                Ok(_) => {},
                Err(e) => return Err(LpnlError::BackupError { 
                    message: format!("Unable to remove a config file: {e}"),
                    kind: BackupErrorKind::FsFailure
                })
            }
        },
        Err(e) => {
            match fs::remove_file(&test_file) {
                Ok(_) => {},
                Err(e) => return Err(LpnlError::BackupError { 
                    message: format!("Unable to remove a config file: {e}"),
                    kind: BackupErrorKind::FsFailure
                })
            }
            return Err(e);
        }
    }

    match fs::write(&nginx_dir_str, contents) {
        Ok(_) => println!("File rewrite successful."),
        Err(e) => {
            match fs::write(&nginx_dir_str, old_config_contents) {
                Ok(_) => {},
                Err(e) => return Err(LpnlError::BackupError {
                    message: format!("Unable to get old data for a config file: {e}"),
                    kind: BackupErrorKind::FsFailure
                }) 
            }
            return Err(LpnlError::BackupError {
                message: format!("Unable to rewrite a config file: {e}"),
                kind: BackupErrorKind::FsFailure
            }) 
        }
    }

    // * testing + running
    match proceed_nginx() {
        Ok(_) => Ok(println!("File backed up successfully.")),
        Err(e) => {
            match fs::write(&nginx_dir_str, old_config_contents) {
                Ok(_) => {},
                Err(e) => return Err(LpnlError::BackupError {
                    message: format!("Unable to get old data for a config file: {e}"),
                    kind: BackupErrorKind::FsFailure
                }) 
            }
            return Err(e);
        }
    }   
}