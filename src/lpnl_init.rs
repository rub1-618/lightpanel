use crate::constants::{
    DEFAULT_DOMAIN, DEFAULT_PORT, 
    NGINX_SITES_ENABLED_DIR, LPNL_BACKUP_DIR,
    NGINX_SITES_DISABLED_DIR, WWW_ROOT_DIR,
};
use crate::error::{InitErrorKind, LpnlError};
use crate::lpnl_backup::set_backup;
use crate::validation::{get_domain, get_root, get_port};
use crate::commands::{proceed_nginx, proceed_check_nginx_tmp};
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
    proceed_check_nginx_tmp(&test_conf)?;

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


    let mut backup_dir = PathBuf::from(LPNL_BACKUP_DIR);
    let domain_as_dir = PathBuf::from(&domain);
    backup_dir.push(domain_as_dir);
    match &backup_dir.exists() {
        true  => {},
        false => { 
            match fs::create_dir_all(&backup_dir) {
                Ok(_)  => {},
                Err(_) => return Err(LpnlError::InitError { 
                        message: format!("Unable to create '{domain}' backup directory."), 
                        kind: InitErrorKind::FsFailure
                    })
                }
        }
    }

    set_backup(Some(domain))?;

    Ok(println!("Generated nginx config to: '{root}'."))
}