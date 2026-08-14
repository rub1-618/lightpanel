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
            return Ok(())
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
            return Ok(())
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
        true => println!("Config file in '/etc/lpnl/backups/{domain}' found. Deleting it..."),
        false => {
            println!("No '{domain}' config file in '/etc/lpnl/backups/{domain}' found.");
            match backup_dir_only.exists() {
                true => {
                    match fs::remove_dir(&backup_dir_only) {
                        Ok(_) => println!("Backup folder in '/etc/lpnl/backups' successfully deleted."),
                        Err(e) => return Err(LpnlError::RemoveError {
                            message: format!("Unable to delete the '{domain}' folder in '/etc/lpnl/backups': {e}."),
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
        Ok(_) => println!("Backup config file in '/etc/lpnl/backups/{domain}' successfully deleted."),
        Err(e) => return Err(LpnlError::RemoveError {
            message: format!("Unable to delete the config file in '/etc/lpnl/backups/{domain}': {e}."),
            kind: RemoveErrorKind::FsFailure
        })
    }

    match fs::remove_dir(backup_dir_only) {
        Ok(_) => println!("Backup folder in '/etc/lpnl/backups' successfully deleted."),
        Err(e) => return Err(LpnlError::RemoveError {
            message: format!("Unable to delete the '{domain}' folder in '/etc/lpnl/backups': {e}."),
            kind: RemoveErrorKind::FsFailure
        })
    }
    Ok(())
}

fn remove_www_files(domain: String) -> Result<(), LpnlError> {
    let mut default_root_dir = PathBuf::from(WWW_ROOT_DIR);
    default_root_dir.push(&domain);
    
    match default_root_dir.exists() {
        true => println!("Root files in '/var/www/{domain}' found. Deleting it..."),
        false => {
            println!("No root files in '/var/www/{domain}' found.");
            return Ok(())
        }
    }

    match fs::remove_dir_all(default_root_dir) {
        Ok(_) => println!("Backup file in '/var/www/{domain}' successfully deleted."),
        Err(e) => return Err(LpnlError::RemoveError { 
            message: format!("Unable to delete the backup file in '/var/www/{domain}': {e}."), 
            kind: RemoveErrorKind::FsFailure
        })
    }
    Ok(())
}

// ! tests

#[cfg(test)]
mod tests {
    use super::*;

    // * domain logic test copy
    fn check_domain(domain: String) -> Result<String, LpnlError> {
        if domain.contains("/") || domain.contains("..") || domain.trim().is_empty() {
            return Err(LpnlError::RemoveError{
                message: "'/', '..' and empty strings are not allowed.".to_string(),
                kind: RemoveErrorKind::InvalidDomain
            })
        }

        let domain = domain.trim().to_string();
        return Ok(domain)
    }

    // ! domain tests

    #[test]
    fn test_remove_get_domain_ok() {
        let domain = check_domain("bye_world".to_string());
        match domain {
            Err(_) => panic!("Unexpected Error."),
            Ok(d) => assert_eq!(d, "bye_world".to_string())
        }
    }

    #[test]
    fn test_remove_default_domain_empty_err() {
        let domain = check_domain("".to_string());
        match domain {
            Err(e) => {
                match e {
                    LpnlError::RemoveError { kind, .. } => {
                        assert!(matches!(kind, RemoveErrorKind::InvalidDomain))
                    }
                    _ => panic!("Expected RemoveError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_remove_default_domain_spaces_err() {
        let domain = check_domain("    ".to_string());
        match domain {
            Err(e) => {
                match e {
                    LpnlError::RemoveError { kind, .. } => {
                        assert!(matches!(kind, RemoveErrorKind::InvalidDomain))
                    }
                    _ => panic!("Expected RemoveError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_remove_default_invalid_str_err() {
        let domain = check_domain("/..".to_string());
        match domain {
            Err(e) => {
                match e {
                    LpnlError::RemoveError { kind, .. } => {
                        assert!(matches!(kind, RemoveErrorKind::InvalidDomain))
                    }
                    _ => panic!("Expected RemoveError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }
}