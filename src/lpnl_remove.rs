use crate::constants::{LPNL_BACKUP_DIR, NGINX_SITES_ENABLED_DIR, WWW_ROOT_DIR};
use crate::error::{LpnlError, RemoveErrorKind};
use std::{fs, io, path::PathBuf, process::Command};

pub fn remove_lpnl(is_forced: bool) -> Result<(), LpnlError> {
    let domain = match get_domain() {
        Ok(d) => d,
        Err(e) => return Err(e)
    };

    match remove_ngnix(domain.clone()) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    if is_forced {
        match remove_backup_configs(domain.clone()) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        match remove_www_files(domain.clone()) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        return Ok(println!("Forced config removing done!"));
    }

    Ok(println!("Config removing done!"))
}

fn remove_ngnix(domain: String) -> Result<(), LpnlError> {
    let mut nginx_config_file = PathBuf::from(NGINX_SITES_ENABLED_DIR);
    let conf_name = format!("{domain}.conf");
    nginx_config_file.push(conf_name);
    
    match nginx_config_file.exists() {
        true => println!("Config file in '/etc/nginx/sites-enabled' found. Deleting it..."),
        false => {
            println!("No '{domain}' config file in '/etc/nginx/sites-enabled' found.");
            return Ok(())
        }
    }

    match fs::remove_file(nginx_config_file) {
        Ok(_) => println!("Ngnix config file in '/etc/nginx/sites-enabled' successfully deleted."),
        Err(e) => return Err(LpnlError::RemoveError { 
            message: format!("Unable to delete the config file in '/etc/nginx/sites-enabled': {e}."), 
            kind: RemoveErrorKind::FsFailure
        })
    }

    let mut check_cmd = Command::new("nginx");
    check_cmd.arg("-t");
    match check_cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("Config files successfully tested.")
            } else {
                let status_code = status.code().unwrap_or_default();
                return Err(LpnlError::RemoveError { 
                    message: format!("Config files testing failed with status code: {status_code}."),
                    kind: RemoveErrorKind::InvalidCmdResult
                })
            }
        },
        Err(e) => return Err(LpnlError::RemoveError { 
            message: format!("Config files testing processes failed: {e}."),
            kind: RemoveErrorKind::InvalidCmdResult
        })
    }

    let mut reload_cmd = Command::new("nginx");
    reload_cmd.args(["-s", "reload"]);
    match reload_cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("Reload successful.")
            } else {
                let status_code = status.code().unwrap_or_default();
                return Err(LpnlError::RemoveError { 
                    message: format!("Config files reloading failed with status code: {status_code}."),
                    kind: RemoveErrorKind::InvalidCmdResult
                })
            }
        },
        Err(e) => return Err(LpnlError::RemoveError { 
            message: format!("Config files reloading processes failed: {e}."),
            kind: RemoveErrorKind::InvalidCmdResult
        })
    }

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

fn get_domain() -> Result<String, LpnlError> {
    // todo: displaying enabled domains array for removing
    loop {
        let mut input = String::new();
        println!("Domain: ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(_) => return Err(LpnlError::RemoveError { 
                message: "Could not get the domain.".to_string() ,
                kind: RemoveErrorKind::IoFailure
            })
        }

        if input.contains("/") || input.contains("..") || input.trim().is_empty() {
            eprintln!("'/', '..' and empty strings are not allowed.");
            continue;
        }

        let domain = input.trim().to_string();
        return  Ok(domain)
    }
}

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