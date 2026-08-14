use crate::commands::proceed_nginx;
use crate::constants::{NGINX_SITES_ENABLED_DIR, NGINX_SITES_DISABLED_DIR};
use crate::error::{LpnlError, StateErrorKind};
use crate::validation::get_domain;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub enum State {
    Enabled,
    Disabled,
    NotFound,
    EnabledAndDisabled,
    Unknown,
}

pub fn enable_domain(domain: Option<String>) -> Result<(), LpnlError> {
    let domain = get_domain(domain)?;
    let conf_dir_str = format!("{NGINX_SITES_DISABLED_DIR}/{domain}.conf");
    let conf_dir = PathBuf::from(conf_dir_str);

    if !conf_dir.exists() {
        return Err(LpnlError::StateError { 
            message: format!("'{domain}' not found or its config file is already enabled."), 
            kind: StateErrorKind::NotFound
        })
    }

    let sites_enabled_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    match fs::copy(&conf_dir, &sites_enabled_dir_str) {
        Ok(_) => {
            match fs::remove_file(conf_dir) {
                Ok(_) => {},
                Err(e) => eprintln!("Unable to delete the copy of '{domain}.conf' from '{NGINX_SITES_DISABLED_DIR}': {e}")
            }
        },
        Err(e) => eprintln!("Unable to move '{domain}.conf' to '{NGINX_SITES_ENABLED_DIR}': {e}")
    }

    Ok(println!("\nConfiguration file of '{domain}' enabled succesfully."))
}

pub fn disable_domain(domain: Option<String>) -> Result<(), LpnlError> {
    let domain = get_domain(domain)?;
    let conf_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let conf_dir = PathBuf::from(conf_dir_str);

    if !conf_dir.exists() {
        return Err(LpnlError::StateError { 
            message: format!("'{domain}' not found or its config file is already disabled."), 
            kind: StateErrorKind::NotFound
        })
    }

    let sites_disabled_dir_str = format!("{NGINX_SITES_DISABLED_DIR}/{domain}.conf");
    match fs::copy(&conf_dir, &sites_disabled_dir_str) {
        Ok(_) => {
            match fs::remove_file(conf_dir) {
                Ok(_) => {},
                Err(e) => eprintln!("Unable to delete the copy of '{domain}.conf' from '{NGINX_SITES_ENABLED_DIR}': {e}")
            }
        },
        Err(e) => eprintln!("Unable to move '{domain}.conf' to '{NGINX_SITES_DISABLED_DIR}': {e}")
    }

    Ok(println!("\nConfiguration file of '{domain}' disabled succesfully."))
}

pub fn enable_all() -> Result<(), LpnlError> {
    let mut str = String::new();
    let mut count: u32 = 0;
    for entry in fs::read_dir(NGINX_SITES_DISABLED_DIR).map_err(|e| LpnlError::StateError{
        message: format!("Unable to read '{NGINX_SITES_DISABLED_DIR}': {e}"),
        kind: StateErrorKind::FsFailure
    })? {
        match entry {
            Ok(entry) => {
                let file_name = match PathBuf::from(&entry.path()).file_name() {
                    Some(e) => format!("{}", e.to_string_lossy()),
                    None => "unknown".to_string()
                };

                let conf_dir_str = format!("{NGINX_SITES_DISABLED_DIR}/{file_name}");
                let sites_enabled_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{file_name}");
                let conf_dir = PathBuf::from(conf_dir_str);
                match fs::copy(&conf_dir, &sites_enabled_dir_str) {
                    Ok(_) => {
                        match fs::remove_file(conf_dir) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Unable to delete the copy of '{file_name}' from '{NGINX_SITES_DISABLED_DIR}': {e}")
                        }
                    },
                    Err(e) => eprintln!("Unable to move '{file_name}' to '{NGINX_SITES_ENABLED_DIR}': {e}")
                }

                let fname = format!("  - {}\n", file_name);
                str.push_str(&fname);
                count += 1;
            },
            Err(e) => {
                eprintln!("One of the files failed on entry stage: {e}.")
            }
        }
    }

    proceed_nginx()?;

    Ok(println!("\nEnabled {count} configs:\n{str}"))
}

pub fn disable_all() -> Result<(), LpnlError> {
    let mut str = String::new();
    let mut count: u32 = 0;
    for entry in fs::read_dir(NGINX_SITES_ENABLED_DIR).map_err(|e| LpnlError::StateError{
        message: format!("Unable to read '{NGINX_SITES_ENABLED_DIR}': {e}"),
        kind: StateErrorKind::FsFailure
    })? {
        match entry {
            Ok(entry) => {
                let file_name = match PathBuf::from(&entry.path()).file_name() {
                    Some(e) => format!("{}", e.to_string_lossy()),
                    None => "unknown".to_string()
                };

                let conf_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{file_name}");
                let sites_disabled_dir_str = format!("{NGINX_SITES_DISABLED_DIR}/{file_name}");
                let conf_dir = PathBuf::from(conf_dir_str);
                match fs::copy(&conf_dir, &sites_disabled_dir_str) {
                    Ok(_) => {
                        match fs::remove_file(conf_dir) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Unable to delete the copy of '{file_name}' from '{NGINX_SITES_ENABLED_DIR}': {e}")
                        }
                    },
                    Err(e) => eprintln!("Unable to move '{file_name}' to '{NGINX_SITES_DISABLED_DIR}': {e}")
                }

                let fname = format!("  - {}\n", file_name);
                str.push_str(&fname);
                count += 1;
            },
            Err(e) => {
                eprintln!("One of the files failed on entry stage: {e}.")
            }
        }
    }

    proceed_nginx()?;

    Ok(println!("\nDisabled {count} configs:\n{str}"))
}

pub fn get_status(domain: Option<String>) -> Result<State, LpnlError> {
    let domain = get_domain(domain)?;
    let mut nginx_sites_enabled_file = PathBuf::from(NGINX_SITES_ENABLED_DIR);
    let mut nginx_sites_disabled_file = PathBuf::from(NGINX_SITES_DISABLED_DIR);
    let conf_name = format!("{domain}.conf");
    let conf = PathBuf::from(conf_name);
    nginx_sites_enabled_file.push(&conf);
    nginx_sites_disabled_file.push(&conf);

    if nginx_sites_disabled_file.exists() && nginx_sites_enabled_file.exists() {
        Ok(State::EnabledAndDisabled)
    } else if !nginx_sites_disabled_file.exists() && !nginx_sites_enabled_file.exists() {
        Ok(State::NotFound)
    } else if !nginx_sites_disabled_file.exists() && nginx_sites_enabled_file.exists() {
        Ok(State::Enabled)
    } else if nginx_sites_disabled_file.exists() && !nginx_sites_enabled_file.exists() {
        Ok(State::Disabled)
    } else {
        Ok(State::Unknown)
    }
}