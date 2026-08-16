use crate::constants::{LPNL_BACKUP_DIR, NGINX_SITES_ENABLED_DIR, NGINX_SITES_DISABLED_DIR, WWW_ROOT_DIR};
use crate::error::{LpnlError, RemoveErrorKind};
use crate::validation::get_domain;
use crate::commands::proceed_nginx;
use std::{fs, path::PathBuf};

pub fn remove_lpnl(domain: Option<String>, is_forced: bool) -> Result<(), LpnlError> {
    
    let domain = get_domain(domain)?;

    remove_ngnix(domain.clone())?;

    if is_forced {
        remove_backup_configs(domain.clone())?;
        remove_www_files(domain.clone())?;
        return Ok(println!("Forced config removing done!"));
    }

    Ok(println!("Config removing done!"))
}

fn remove_ngnix(domain: String) -> Result<(), LpnlError> {
    let nginx_sites_enabled_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let nginx_sites_disabled_dir_str = format!("{NGINX_SITES_DISABLED_DIR}/{domain}.conf");
    let enabled_file = PathBuf::from(&nginx_sites_enabled_dir_str);
    let disabled_file = PathBuf::from(&nginx_sites_disabled_dir_str);
    
    match enabled_file.exists() {
        true => {
            println!("Config file in '{NGINX_SITES_ENABLED_DIR}' found. Deleting it...");
            match fs::remove_file(nginx_sites_enabled_dir_str) {
                Ok(_) => println!("Ngnix config file in '{NGINX_SITES_ENABLED_DIR}' successfully deleted."),
                Err(e) => return Err(LpnlError::RemoveError { 
                    message: format!("Unable to delete the config file in '{NGINX_SITES_ENABLED_DIR}': {e}."), 
                    kind: RemoveErrorKind::FsFailure
                })
            }
        },
        false => {
            println!("No '{domain}' config file in '{NGINX_SITES_ENABLED_DIR}' found.");
        }
    }
    
    match disabled_file.exists() {
        true => {
            println!("Config file in '{NGINX_SITES_DISABLED_DIR}' found. Deleting it...");
            match fs::remove_file(nginx_sites_disabled_dir_str) {
                Ok(_) => println!("Ngnix config file in '{NGINX_SITES_DISABLED_DIR}' successfully deleted."),
                Err(e) => return Err(LpnlError::RemoveError { 
                    message: format!("Unable to delete the config file in '{NGINX_SITES_DISABLED_DIR}': {e}."), 
                    kind: RemoveErrorKind::FsFailure
                })
            }
        },
        false => {
            println!("No '{domain}' config file in '{NGINX_SITES_DISABLED_DIR}' found.");
        }
    }

    proceed_nginx()?;
    
    Ok(())
}

fn remove_backup_configs(domain: String) -> Result<(), LpnlError> {
    let mut backup_config_file = PathBuf::from(LPNL_BACKUP_DIR);
    let conf_name = format!("{domain}.txt");
    backup_config_file.push(&domain);
    let backup_dir_only = backup_config_file.clone();
    backup_config_file.push(conf_name);
    
    match backup_config_file.exists() {
        true => println!("Config file in '{LPNL_BACKUP_DIR}/{domain}' found. Deleting it..."),
        false => {
            println!("No '{domain}' config file in '{LPNL_BACKUP_DIR}/{domain}' found.");
            match backup_dir_only.exists() {
                true => {
                    match fs::remove_dir(&backup_dir_only) {
                        Ok(_) => println!("Backup folder in '{LPNL_BACKUP_DIR}' successfully deleted."),
                        Err(e) => return Err(LpnlError::RemoveError {
                            message: format!("Unable to delete the '{domain}' folder in '{LPNL_BACKUP_DIR}': {e}."),
                            kind: RemoveErrorKind::FsFailure
                        })
                    }
                    return Ok(());
                }
                false => return Ok(())
            }
        }
    }

    match fs::remove_file(backup_config_file) {
        Ok(_) => println!("Backup config file in '{LPNL_BACKUP_DIR}/{domain}' successfully deleted."),
        Err(e) => return Err(LpnlError::RemoveError {
            message: format!("Unable to delete the config file in '{LPNL_BACKUP_DIR}/{domain}': {e}."),
            kind: RemoveErrorKind::FsFailure
        })
    }

    match fs::remove_dir(backup_dir_only) {
        Ok(_) => println!("Backup folder in '{LPNL_BACKUP_DIR}' successfully deleted."),
        Err(e) => return Err(LpnlError::RemoveError {
            message: format!("Unable to delete the '{domain}' folder in '{LPNL_BACKUP_DIR}': {e}."),
            kind: RemoveErrorKind::FsFailure
        })
    }
    Ok(())
}

fn remove_www_files(domain: String) -> Result<(), LpnlError> {
    let mut default_root_dir = PathBuf::from(WWW_ROOT_DIR);
    default_root_dir.push(&domain);
    
    match default_root_dir.exists() {
        true => println!("Root files in '{WWW_ROOT_DIR}' found. Deleting it..."),
        false => {
            println!("No '{domain}' root files in '{WWW_ROOT_DIR}' found.");
            return Ok(())
        }
    }

    match fs::remove_dir_all(default_root_dir) {
        Ok(_) => println!("Root files in '{WWW_ROOT_DIR}' successfully deleted."),
        Err(e) => return Err(LpnlError::RemoveError { 
            message: format!("Unable to delete the '{domain}' files in '{WWW_ROOT_DIR}': {e}."), 
            kind: RemoveErrorKind::FsFailure
        })
    }
    Ok(())
}