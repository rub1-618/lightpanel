use crate::constants::{NGINX_SITES_ENABLED_DIR, LPNL_TMP_DIR};
use crate::error::{LpnlError, RemoveLocErrorKind};
use crate::validation::{get_domain, get_location};
use crate::commands::{proceed_nginx, proceed_check_nginx_with_dir, proceed_check_nginx};
use std::{fs, path::PathBuf};

pub fn remove_loc(domain: Option<String>, location: Option<String>) -> Result<(), LpnlError> {
    proceed_check_nginx()?;

    let test_file = format!("{LPNL_TMP_DIR}/run_test.txt");

    let domain = get_domain(domain)?;
    let location = get_location(location)?;

    let sites_enabled_conf_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let sites_enabled_conf_dir = PathBuf::from(sites_enabled_conf_str.clone());
    let sites_enabled_conf_as_str = match fs::read_to_string(&sites_enabled_conf_dir) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to read the '{sites_enabled_conf_str}' to get the config: {e}"),
            kind: RemoveLocErrorKind::FsFailure
        })
    };

    if sites_enabled_conf_as_str.lines().find(|&l| l.contains(&location)).is_none() {
        return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find the 'location {location}' block in '{sites_enabled_conf_str}'."),
            kind: RemoveLocErrorKind::NotFound
        })
    }

    // * getting the indexes of the part we want to remove
    let fpart = format!("location {location} {{");
    let li = match sites_enabled_conf_as_str.find(&fpart) {
        Some(u) => u,
        None => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find the 'location {location}' block in '{sites_enabled_conf_str}'."),
            kind: RemoveLocErrorKind::NotFound
        })
    };
    let before_bracket = match sites_enabled_conf_as_str[li..].find('}').map(|i| i + li) {
        Some(u) => u,
        None => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to find the 'location {location}' block in '{sites_enabled_conf_str}'."),
            kind: RemoveLocErrorKind::NotFound
        })
    };
    let bi = before_bracket + 2; // '}\n'

    // * cutting out the part we need to delete
    let final_conf = sites_enabled_conf_as_str.replace(
        &sites_enabled_conf_as_str[li..bi], ""
    );
    
    // * testing + launching
    let test_conf = format!("events {{  }} http {{ {final_conf} }}");
    match fs::write(&test_file, &test_conf) {
        Ok(_) => {
            match proceed_check_nginx_with_dir(&test_file) {
                Ok(_) => {
                    match fs::remove_file(test_file) {
                        Ok(_) => {},
                        Err(e) => return Err(LpnlError::RemoveLocError { 
                            message: format!("Unable to remove the testing file: {e}."),
                            kind: RemoveLocErrorKind::FsFailure
                        })
                    }
                    match fs::write(&sites_enabled_conf_str, &final_conf) {
                        Ok(_) => proceed_nginx()?,
                        Err(e) => return Err(LpnlError::RemoveLocError { 
                            message: format!("Unable to update the config in '{sites_enabled_conf_str}': {e}"),
                            kind: RemoveLocErrorKind::FsFailure
                        })
                    }
                }
                Err(e) => {
                    match fs::remove_file(test_file) {
                        Ok(_) => {},
                        Err(e) => return Err(LpnlError::RemoveLocError { 
                                message: format!("Unable to remove the testing file: {e}."),
                                kind: RemoveLocErrorKind::FsFailure
                            })
                        }
                    return Err(e)
                }
            }
        }
        Err(e) => return Err(LpnlError::RemoveLocError { 
            message: format!("Unable to check the config in '{test_file}': {e}"),
            kind: RemoveLocErrorKind::FsFailure
        })
    }

    Ok(println!("Location '{location}' delete from '{domain}' successful."))
}