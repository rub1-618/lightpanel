use crate::constants::{
    DEFAULT_DOMAIN, DEFAULT_PORT, 
    LPNL_TMP_DIR, NGINX_SITES_ENABLED_DIR, 
    NGINX_SITES_DISABLED_DIR, WWW_ROOT_DIR,
    LPNL_BACKUP_DIR
};
use crate::error::{InitErrorKind, LpnlError};
use crate::validation::{get_domain, get_root, get_port};
use crate::commands::{proceed_nginx, proceed_check_nginx_with_dir};
use std::{fs, path::PathBuf};

pub fn init_lpnl(domain: Option<String>, root: Option<PathBuf>, port: Option<u16>, is_default: bool) -> Result<(), LpnlError> {
    // * domain recieving
    let domain = match domain {
        Some(_) => get_domain(domain)?,
        None => {
            if is_default {
                DEFAULT_DOMAIN.to_string()
            } else {
                get_domain(domain)?
            }
        }
    };

    // * port recieving
    let port = match port {
        Some(_) => get_port(port)?,
        None => {
            if is_default {
                DEFAULT_PORT
            } else {
                get_port(port)?
            }
        }
    };

    // ! root init and path recieving
    let root = match root {
        Some(r) => get_root(Some(r))?,
        None => {
            if is_default {
                let mut root_dir = PathBuf::from(WWW_ROOT_DIR);
                let domain_as_dir = PathBuf::from(&domain);
                root_dir.push(domain_as_dir);
                match &root_dir.exists() {
                    true  => root_dir.to_str().unwrap().to_string(),
                    false => { 
                        match fs::create_dir_all(&root_dir) {
                            Ok(_)  => root_dir.to_str().unwrap().to_string(),
                            Err(_) => return Err(LpnlError::InitError { 
                                message: format!("Unable to create '{domain}' root directory."), 
                                kind: InitErrorKind::FsFailure
                            })
                        }
                    }
                }
            } else {
                get_root(root)?
            }
        }
    };
    
    let config = generate_config(domain.clone(), port, root.clone(), false);
    let test_config = generate_config(domain.clone(), port, root.clone(), true);

    match init_nginx(domain, config, test_config, root) {
        Ok(_) => {},
        Err(e) => return Err(e),
    }

    Ok(println!("Initialization finished successfully!"))
}

fn generate_config(domain: String, port: u16, root: String, is_for_test: bool) -> String {

    let root_oriented_conf = format!("
server {{ 
    listen {port};
    server_name {domain};
    add_header X-Server: \"global\";
    add_header X-Security: \"strict\";
            
    location / {{
        root {root};
    }}
}}");

    let test_conf = format!("events {{  }}  http {{ {root_oriented_conf} }}");

    if is_for_test {
        test_conf
    } else {
        root_oriented_conf
    }
}

// ! creating nginx.conf
fn init_nginx(domain: String, final_conf: String, test_conf: String, root: String) -> Result<(), LpnlError> {

    // * testing
    let test_file = format!("{LPNL_TMP_DIR}/run_test.txt");

    match fs::write(test_file.clone(), &test_conf) {
        Ok(_) => {}
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Writing a config into an initialization file failed: {e}."),
            kind: InitErrorKind::FsFailure
        })
    }
    match proceed_check_nginx_with_dir(&test_file) {
        Ok(_) => {
            match fs::remove_file(test_file) {
                Ok(_) => {},
                Err(e) => return Err(LpnlError::InitError { 
                    message: format!("Unable to remove the testing file: {e}."),
                    kind: InitErrorKind::InvalidCmdResult
                })
            }
        }
        Err(e) => {
            match fs::remove_file(test_file) {
                Ok(_) => {},
                Err(e) => return Err(LpnlError::InitError { 
                    message: format!("Unable to remove the testing file: {e}."),
                    kind: InitErrorKind::InvalidCmdResult
                })
            }
            return Err(e)
        }
    };


    let mut backup_dir = PathBuf::from(LPNL_BACKUP_DIR);
    backup_dir.push(&domain);
    let backup_dir_str = backup_dir.to_str().unwrap().to_string();
    match fs::create_dir_all(&backup_dir) {
        Ok(_) => println!("Created backup folder in '{backup_dir_str}' successfully."),
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Unable to create a backup folder: {e}"),
            kind: InitErrorKind::FsFailure
        }) 
    }

    // * initializing to sites enabled
    let nginx_sites_enabled_dir_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let nginx_sites_disabled_dir_str = format!("{NGINX_SITES_DISABLED_DIR}/{domain}.conf");
    let enabled_file = PathBuf::from(&nginx_sites_enabled_dir_str);
    let disabled_file = PathBuf::from(&nginx_sites_disabled_dir_str);

    if enabled_file.exists() && !disabled_file.exists() {
        return Err(LpnlError::InitError { 
            message: format!("'{domain}' is already initialized and enabled."),
            kind: InitErrorKind::AlreadyExists
        })
    } else if !enabled_file.exists() && disabled_file.exists() {
        return Err(LpnlError::InitError { 
            message: format!("'{domain}' is already initialized and disabled."),
            kind: InitErrorKind::AlreadyExists
        })
    } else if enabled_file.exists() && disabled_file.exists() {
        return Err(LpnlError::InitError { 
            message: format!("'{domain}' is already initialized, enabled and disabled. Try removing file and initializing it again."),
            kind: InitErrorKind::AlreadyExists
        })
    } else {
        match fs::write(nginx_sites_enabled_dir_str.clone(), &final_conf) {
            Ok(_) => {}
            Err(e) => return Err(LpnlError::InitError { 
                message: format!("Writing a config into an initialization file failed: {e}."),
                kind: InitErrorKind::FsFailure
            })
        }
    }

    proceed_nginx()?;

    Ok(println!("Generated nginx config to: '{root}'."))
}

#[cfg(test)]
mod tests {
    use crate::error::ValidationErrorKind;
    use super::*;

    // ! domain tests

    #[test]
    fn test_init_get_domain_ok() {
        let domain = get_domain(Some("example.com".to_string()));
        match domain {
            Err(_) => panic!("Unexpected Error."),
            Ok(d) => assert_eq!(d, "example.com".to_string())
        }
    }

    #[test]
    fn test_init_get_domain_err() {
        let domain = get_domain(Some("/..domain".to_string()));
        match domain {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidDomain))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    // ! port tests

    #[test]
    fn test_init_get_port_ok() {
        let port = get_port(Some(80));
        match port {
            Err(_) => panic!("Unexpected Error."),
            Ok(p) => assert_eq!(p, 80)
        }
    }

    #[test]
    fn test_init_get_port_err() {
        let port = get_port(Some(0));
        match port {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidPort))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    // ! root tests

    #[test]
    fn test_init_get_root_ok() {
        let valid_path = PathBuf::from("/var/www");
        let root = get_root(Some(valid_path));
        match root {
            Err(_) => panic!("Unexpected Error."),
            Ok(d) => assert_eq!(d, "/var/www".to_string())
        }
    }

    #[test]
    fn test_init_get_root_err() {
        let invalid_path = PathBuf::from("/../hello");
        let root = get_root(Some(invalid_path));
        match root {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidRoot))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }
}