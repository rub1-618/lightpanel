use crate::constants::{NGINX_SITES_ENABLED_DIR};
use crate::error::{LpnlError, AddLocErrorKind};
use crate::validation::{get_domain, get_location, get_proxy, get_root};
use crate::commands::{proceed_nginx, proceed_check_nginx_tmp};
use std::{io, fs, path::PathBuf};

pub fn add_loc(
    domain: Option<String>, nloc: Option<String>, 
    nroot: Option<PathBuf>, nproxy: Option<String>
) -> Result<(), LpnlError> {

    if nproxy.is_some() && nroot.is_some() {
        return Err(LpnlError::AddLocError { 
            message: "You can use only 1 flag at a time.".to_string(),
            kind: AddLocErrorKind::InvalidInput
        })
    }

    let domain = get_domain(domain)?;
    let location = get_location(nloc)?;

    let sites_enabled_dir = PathBuf::from(NGINX_SITES_ENABLED_DIR);
    if !sites_enabled_dir.exists() {
        return Err(LpnlError::AddLocError { 
            message: format!("Unable to find a config files' folder in '{NGINX_SITES_ENABLED_DIR}'. Consider using the setup command."),
            kind: AddLocErrorKind::FsFailure
        })
    }

    let sites_enabled_conf_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let sites_enabled_conf_dir = PathBuf::from(sites_enabled_conf_str);
    if !sites_enabled_conf_dir.exists() {
        return Err(LpnlError::AddLocError { 
            message: format!("Unable to find a config file folder in '{NGINX_SITES_ENABLED_DIR}'. Consider using the setup command."),
            kind: AddLocErrorKind::NotFound
        })
    }

    if nproxy.is_none() && nroot.is_none() {
        if is_proxy_mode()? {
            let proxy = get_proxy(nproxy)?;
            return add_proxy_loc(domain, location, proxy)
        } else {
            let root  = get_root(nroot.clone())?;
            return add_root_loc(domain, location, root);
        }
    }

    if nroot.is_some() {
        let root  = get_root(nroot.clone())?;
        return add_root_loc(domain, location, root);
    };

    if nproxy.is_some() {
        let proxy  = get_proxy(nproxy.clone())?;
        return add_proxy_loc(domain, location, proxy);
    };

    Err(LpnlError::AddLocError { 
        message: "Root / proxy selection stage went wrong.".to_string(),
        kind: AddLocErrorKind::NotFound
    })
}

fn add_root_loc(domain: String, location: String, root: String) -> Result<(), LpnlError> {

    let sites_enabled_conf_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let sites_enabled_conf_dir = PathBuf::from(sites_enabled_conf_str.clone());
    let sites_enabled_conf_as_str = match fs::read_to_string(&sites_enabled_conf_dir) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::AddLocError { 
            message: format!("Unable to read the '{sites_enabled_conf_str}' to get the config: {e}"),
            kind: AddLocErrorKind::FsFailure
        })
    };

    let conf_loc = format!("location {location}");
    if sites_enabled_conf_as_str.lines().find(|&l| l.contains(&conf_loc)).is_none() {
        match sites_enabled_conf_as_str.lines().find(|&l| l.trim().starts_with("location / {")) {
            Some(l) => {
                let conf_part = format!("
                location {location} {{ root {root}; }}
                
                location / {{");
                let final_conf = sites_enabled_conf_as_str.replace(l, &conf_part);
                let test_conf = format!("events {{  }} http {{ {final_conf} }}");
                proceed_check_nginx_tmp(&test_conf)?;
                match fs::write(&sites_enabled_conf_str, &final_conf) {
                    Ok(_) => {
                        proceed_nginx()?;
                    }
                    Err(e) => return Err(LpnlError::AddLocError { 
                            message: format!("Unable to update the config in '{sites_enabled_conf_str}': {e}"),
                            kind: AddLocErrorKind::FsFailure
                       })
                    }
            }
            None => return Err(LpnlError::AddLocError { 
                message: format!("Unable to find the 'location /' block in '{sites_enabled_conf_str}'."),
                kind: AddLocErrorKind::NotFound
            })
        }
    } else {
        return Err(LpnlError::AddLocError { 
            message: format!("The config location '{location}' already exists."),
            kind: AddLocErrorKind::AlreadyExists
        })
    }


    Ok(println!("Location '{location}' with its root '{root}' created."))
}

fn add_proxy_loc(domain: String, location: String, proxy: String) -> Result<(), LpnlError> {

    let sites_enabled_conf_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let sites_enabled_conf_dir = PathBuf::from(sites_enabled_conf_str.clone());
    let sites_enabled_conf_as_str = match fs::read_to_string(&sites_enabled_conf_dir) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::AddLocError { 
            message: format!("Unable to read the '{sites_enabled_conf_str}' to get the config: {e}"),
            kind: AddLocErrorKind::FsFailure
        })
    };

    let conf_loc = format!("location {location}");
    if sites_enabled_conf_as_str.lines().find(|&l| l.contains(&conf_loc)).is_none() {
        match sites_enabled_conf_as_str.lines().find(|&l| l.trim().starts_with("location / {")) {
            Some(l) => {
                let conf_part = format!("
                location {location} {{
                    proxy_pass {proxy};
                    proxy_set_header Host $host;
                    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                    proxy_set_header X-Real-IP $remote_addr;
                }}
                
                location / {{");
                let final_conf = sites_enabled_conf_as_str.replace(l, &conf_part);
                let test_conf = format!("events {{  }} http {{ {final_conf} }}");
                proceed_check_nginx_tmp(&test_conf)?;
                match fs::write(&sites_enabled_conf_str, &final_conf) {
                    Ok(_) => {
                        proceed_nginx()?;
                    }
                    Err(e) => return Err(LpnlError::AddLocError { 
                        message: format!("Unable to update the config in '{sites_enabled_conf_str}': {e}"),
                        kind: AddLocErrorKind::FsFailure
                    })
                }
            },
            None => return Err(LpnlError::AddLocError { 
                message: format!("Unable to find the 'location /' block in '{sites_enabled_conf_str}'."),
                kind: AddLocErrorKind::NotFound
            })
        }
    } else {
        return Err(LpnlError::AddLocError { 
            message: format!("The config location '{location}' already exists."),
            kind: AddLocErrorKind::AlreadyExists
        })
    }

    Ok(println!("Location '{location}' with its proxy '{proxy}' created."))
}

fn is_proxy_mode() -> Result<bool, LpnlError> {
    loop {
        let mut input = String::new();
        println!("Select mode (proxy/root): ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(e) => return Err(LpnlError::AddLocError { 
                message: format!("Unable to get the location: {e}"), 
                kind: AddLocErrorKind::IoFailure
            })
        }
        print!("\n");

        if input.trim() == "proxy" {
            return Ok(true)
        }

        if input.trim() == "root" {
            return Ok(false)
        }

        eprintln!("You can only answer 'proxy' or 'root'.");
        continue;
    }
}