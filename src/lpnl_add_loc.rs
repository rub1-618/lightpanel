use crate::constants::{NGINX_SITES_ENABLED_DIR, NGINX_SITES_DISABLED_DIR};
use crate::error::{LpnlError, AddLocErrorKind};
use crate::validation::{get_domain, get_location, get_proxy, get_root};
use crate::commands::{proceed_nginx, proceed_check_nginx_tmp};
use std::{io, fs, path::PathBuf};

#[derive(Debug, Clone)]
enum LocMode {
    Root, Proxy
}

pub fn add_loc( // todo: sites-disabled dir checking and proceeding
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

    let sites_disabled_dir = PathBuf::from(NGINX_SITES_DISABLED_DIR);
    if !sites_disabled_dir.exists() {
        return Err(LpnlError::AddLocError { 
            message: format!("Unable to find a config files' folder in '{NGINX_SITES_DISABLED_DIR}'. Consider using the setup command."),
            kind: AddLocErrorKind::FsFailure
        })
    }

    let sites_enabled_conf_str = format!("{NGINX_SITES_ENABLED_DIR}/{domain}.conf");
    let sites_disabled_conf_str = format!("{NGINX_SITES_DISABLED_DIR}/{domain}.conf");
    let sites_enabled_conf_dir = PathBuf::from(sites_enabled_conf_str);
    let sites_disabled_conf_dir = PathBuf::from(sites_disabled_conf_str);
    
    if sites_enabled_conf_dir.exists() && sites_disabled_conf_dir.exists() {
        return Err(LpnlError::AddLocError { 
            message: format!("'{domain}' is enabled and disabled. Try removing file and initializing it again."),
            kind: AddLocErrorKind::AlreadyExists
        })
    }

    if !sites_enabled_conf_dir.exists() && !sites_disabled_conf_dir.exists() {
        return Err(LpnlError::AddLocError { 
            message: format!("Unable to find '{domain}' config."),
            kind: AddLocErrorKind::NotFound
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

    if nproxy.is_none() && nroot.is_none() {
        if is_proxy_mode()? {
            let proxy = get_proxy(nproxy)?;
            return proceed_add_loc(location, proxy, target_dir, LocMode::Proxy);
        } else {
            let root  = get_root(nroot.clone())?;
            return proceed_add_loc(location, root, target_dir, LocMode::Root);
        }
    }

    if nroot.is_some() {
        let root  = get_root(nroot.clone())?;
            return proceed_add_loc(location, root, target_dir, LocMode::Root);
    };

    if nproxy.is_some() {
        let proxy  = get_proxy(nproxy.clone())?;
        return proceed_add_loc(location, proxy, target_dir, LocMode::Proxy);
    };

    Err(LpnlError::AddLocError { 
        message: "Root/proxy selection stage went wrong.".to_string(),
        kind: AddLocErrorKind::NotFound
    })
}

fn proceed_add_loc(location: String, data: String, target_dir: PathBuf, mode: LocMode) -> Result<(), LpnlError> {

   let target_conf_str = target_dir.to_str().unwrap().to_string(); 
    let target_conf_as_str = match fs::read_to_string(&target_dir) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::AddLocError { 
            message: format!("Unable to read the '{target_conf_str}' to get the config: {e}"),
            kind: AddLocErrorKind::FsFailure
        })
    };

    let conf_loc = format!("location {location}");
    if target_conf_as_str.lines().find(|&l| l.contains(&conf_loc)).is_none() {
        match target_conf_as_str.lines().find(|&l| l.trim().starts_with("location / {")) {
            Some(l) => {
                let conf_part = match mode {
                    LocMode::Root  => {
                        format!("
                location {location} {{ root {data}; }}
                
                location / {{")
                    }
                    LocMode::Proxy => {
                        format!("
                location {location} {{
                    proxy_pass {data};
                    proxy_set_header Host $host;
                    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                    proxy_set_header X-Real-IP $remote_addr;
                }}
                
                location / {{")
                    }
                };
                let final_conf = target_conf_as_str.replace(l, &conf_part);
                let test_conf = format!("events {{  }} http {{ {final_conf} }}");
                proceed_check_nginx_tmp(&test_conf)?;
                match fs::write(&target_dir, &final_conf) {
                    Ok(_) => {
                        proceed_nginx()?;
                    }
                    Err(e) => return Err(LpnlError::AddLocError { 
                            message: format!("Unable to update the config in '{target_conf_str}': {e}"),
                            kind: AddLocErrorKind::FsFailure
                       })
                    }
            }
            None => return Err(LpnlError::AddLocError { 
                message: format!("Unable to find the 'location /' block in '{target_conf_str}'."),
                kind: AddLocErrorKind::NotFound
            })
        }
    } else {
        return Err(LpnlError::AddLocError { 
            message: format!("The config location '{location}' already exists."),
            kind: AddLocErrorKind::AlreadyExists
        })
    }


    Ok(println!("Location '{location}' with the content of '{data}' created."))
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