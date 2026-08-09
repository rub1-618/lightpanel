use std::{fs, io, path::PathBuf, process::Command};

use crate::error::RemoveError;

const DEFAULT_DOMAIN: &str      = "localhost";
const LPNL_BACKUP_DIR: &str     = "/etc/lpnl/backups";
const NGINX_CONFIGS_DIR: &str   = "/etc/nginx/sites-available";
const WWW_ROOT_DIR: &str        = "/var/www";

pub fn remove_lpnl(is_forced: bool) -> Result<(), RemoveError> {
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

fn remove_ngnix(domain: String) -> Result<(), RemoveError> {
    let mut nginx_config_file = PathBuf::from(NGINX_CONFIGS_DIR);
    let conf_name = format!("{domain}.conf");
    nginx_config_file.push(conf_name);
    
    match nginx_config_file.exists() {
        true => println!("Config file in '/etc/nginx/sites-available' found. Deleting it..."),
        false => {
            println!("No '{domain}' config file in '/etc/nginx/sites-available' found.");
            return Ok(())
        }
    }

    match fs::remove_file(nginx_config_file) {
        Ok(_) => println!("Ngnix config file in '/etc/nginx/sites-available' successfully deleted."),
        Err(e) => return Err(RemoveError { 
            message: format!("Unable to delete the config file in '/etc/nginx/sites-available': {e}.")
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
                return Err(RemoveError { 
                    message: format!("Config files testing failed with status code: {status_code}.")
                })
            }
        },
        Err(e) => return Err(RemoveError { 
            message: format!("Config files testing processes failed: {e}.")
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
                return Err(RemoveError { 
                    message: format!("Config files reloading failed with status code: {status_code}.")
                })
            }
        },
        Err(e) => return Err(RemoveError { 
            message: format!("Config files reloading processes failed: {e}.")
        })
    }

    Ok(())
}

fn remove_backup_configs(domain: String) -> Result<(), RemoveError> {
    let mut backup_config_file = PathBuf::from(LPNL_BACKUP_DIR);
    let conf_name = format!("{domain}.txt");
    backup_config_file.push(&domain);
    backup_config_file.push(conf_name);
    
    match backup_config_file.exists() {
        true => println!("Config file in '/etc/lpnl/backups/{domain}' found. Deleting it..."),
        false => {
            println!("No '{domain}' config file in '/etc/lpnl/backups/{domain}' found.");
            return Ok(())
        }
    }

    match fs::remove_file(backup_config_file) {
        Ok(_) => println!("Backup config file in '/etc/lpnl/backups/{domain}' successfully deleted."),
        Err(e) => return Err(RemoveError { 
            message: format!("Unable to delete the config file in '/etc/lpnl/backups/{domain}': {e}.")
        })
    }
    Ok(())
}

fn remove_www_files(domain: String) -> Result<(), RemoveError> {
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
        Err(e) => return Err(RemoveError { 
            message: format!("Unable to delete the backup file in '/var/www/{domain}': {e}.")
        })
    }
    Ok(())
}

fn get_domain() -> Result<String, RemoveError> {
    // todo: displaying available domains array for removing
    let mut input = String::new();
    println!("Domain: ");
    match io::stdin().read_line(&mut input) {
        Ok (_) => {},
        Err(_) => return Err(RemoveError { 
            message: "Could not get the domain.".to_string() 
        })
    }

    if input.contains("/") || input.contains("..") {
        return Err(RemoveError{ 
            message: "Invalid domain. '/', '..' and empty strings are not allowed.".to_string()
        })
    }

    if input.trim().is_empty() {
        let domain: String = DEFAULT_DOMAIN.to_string();
        return Ok(domain);
    };

    let domain = input.trim().to_string();
    Ok(domain)
}