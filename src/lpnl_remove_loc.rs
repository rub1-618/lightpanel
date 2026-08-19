use crate::constants::{NGINX_SITES_ENABLED_DIR, NGINX_SITES_DISABLED_DIR};
use crate::error::{LpnlError, RemoveLocErrorKind};
use crate::validation::{get_domain, get_location};
use crate::commands::{proceed_nginx, proceed_check_nginx_tmp, proceed_check_nginx};
use std::{fs, path::PathBuf};

pub fn remove_loc(domain: Option<String>, location: Option<String>) -> Result<(), LpnlError> {
    proceed_check_nginx()?;

    let domain = get_domain(domain)?;
    let location = get_location(location)?;

    let sites_enabled_dir = PathBuf::from(NGINX_SITES_ENABLED_DIR);
    if !sites_enabled_dir.exists() {
        return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find a config files' folder in '{NGINX_SITES_ENABLED_DIR}'. Consider using the setup command."),
            kind: RemoveLocErrorKind::FsFailure
        })
    }

    let sites_disabled_dir = PathBuf::from(NGINX_SITES_DISABLED_DIR);
    if !sites_disabled_dir.exists() {
        return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find a config files' folder in '{NGINX_SITES_DISABLED_DIR}'. Consider using the setup command."),
            kind: RemoveLocErrorKind::FsFailure
        })
    }

    let sites_enabled_conf_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let sites_disabled_conf_str = format!("{NGINX_SITES_DISABLED_DIR}/{domain}.conf");
    let sites_enabled_conf_dir = PathBuf::from(sites_enabled_conf_str);
    let sites_disabled_conf_dir = PathBuf::from(sites_disabled_conf_str);
    
    if sites_enabled_conf_dir.exists() && sites_disabled_conf_dir.exists() {
        return Err(LpnlError::RemoveLocError { 
            message: format!("'{domain}' is enabled and disabled. Try removing file and initializing it again."),
            kind: RemoveLocErrorKind::AlreadyExists
        })
    }

    if !sites_enabled_conf_dir.exists() && !sites_disabled_conf_dir.exists() {
        return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find '{domain}' config."),
            kind: RemoveLocErrorKind::NotFound
        })
    }

    let target_dir = match sites_enabled_conf_dir.exists() {
        true  => {
            sites_enabled_conf_dir
        }
        false => {
            sites_disabled_conf_dir
        }
    };

    let target_conf_str = target_dir.to_str().unwrap().to_string(); 
    let target_conf_as_str = match fs::read_to_string(&target_dir) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to read the '{target_conf_str}' to get the config: {e}"),
            kind: RemoveLocErrorKind::FsFailure
        })
    };

    if target_conf_as_str.lines().find(|&l| l.contains(&location)).is_none() {
        return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find the 'location {location}' block in '{target_conf_str}'."),
            kind: RemoveLocErrorKind::NotFound
        })
    }

    // * getting the indexes of the part we want to remove
    let fpart = format!("location {location} {{");
    let li = match target_conf_as_str.find(&fpart) {
        Some(u) => u,
        None => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find the 'location {location}' block in '{target_conf_str}'."),
            kind: RemoveLocErrorKind::NotFound
        })
    };
    let before_bracket = match target_conf_as_str[li..].find('}').map(|i| i + li) {
        Some(u) => u,
        None => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find the 'location {location}' block in '{target_conf_str}'."),
            kind: RemoveLocErrorKind::NotFound
        })
    };
    let bi = before_bracket + 2; // '}\n'

    // * cutting out the part we need to delete
    let final_conf = target_conf_as_str.replace(
        &target_conf_as_str[li..bi], ""
    );
    
    // * testing + launching
    let test_conf = format!("events {{  }} http {{ {final_conf} }}");
    
    proceed_check_nginx_tmp(&test_conf)?;
    match fs::write(&target_conf_str, &final_conf) {
        Ok(_) => proceed_nginx()?,
        Err(e) => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to update the config in '{target_conf_str}': {e}"),
            kind: RemoveLocErrorKind::FsFailure
        })
    }

    Ok(println!("Location '{location}' delete from '{domain}' successful."))
}