use std::{fs, io, path::PathBuf, process::Command};

use crate::error::InitializationError;

const LPNL_DIR_STR: &str = "/etc/lpnl";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_DOMAIN: &str = "localhost";
const DEFAULT_ROOT: &str = "/var/www";

pub fn init_lpnl(is_default: bool) -> Result<(), InitializationError> {
    // * domain recieving
    let domain = if !is_default {
        match get_domain() {
            Ok(s) => s,
            Err(e) => return Err(e)
        }
    } else {
        DEFAULT_DOMAIN.to_string()
    };

    // * port recieving
    let port: u16 = if !is_default {
        match get_port() {
            Ok(p) => p,
            Err(e) => return Err(e)
        }
    } else {
        DEFAULT_PORT
    };

    // ! root init and path recieving
    let root: String = if !is_default {
        match get_root_dir() {
            Ok(p) => p,
            Err(e) => return Err(e)
        }
    } else {
        match init_root_dir(&domain) {
            Ok(p) => p,
            Err(e) => return Err(e),
        }
    };

    // ! creating the backup itself
    match init_backup_dir(domain.clone()) {
        Ok(_) => {},
        Err(e) => return Err(e),
    }
    
    let config = generate_config(domain.clone(), port, root.clone(), false);
    let test_config = generate_config(domain.clone(), port, root.clone(), true);

    match init_nginx(domain, config, test_config, root) {
        Ok(_) => {},
        Err(e) => return Err(e),
    }

    Ok(println!("initialization finished successfully!"))
}

fn init_backup_dir(domain: String) -> Result<(), InitializationError> {
    let mut backup_dir = PathBuf::from(LPNL_DIR_STR);
    backup_dir.push("backups");

    let domain_dir = PathBuf::from(&domain);
    let mut current_backup_dir = backup_dir.clone();
    current_backup_dir.push(&domain_dir);
    if current_backup_dir.exists() { 
        return Err(InitializationError{
            message: "This config is already initialized.".to_string()
        })
    }
    match fs::create_dir(current_backup_dir) {
        Ok(_)  => Ok(()),
        Err(_) => Err(InitializationError { 
            message: format!("Unable to create '{domain}' backup directory.")
        })
    }
}

// returns String so that root path can be used in main initialization later
fn init_root_dir(domain: &str) -> Result<String, InitializationError> {
    let mut root_dir = PathBuf::from(DEFAULT_ROOT);
    let domain_as_dir = PathBuf::from(domain);
    root_dir.push(domain_as_dir);
    match &root_dir.exists() {
        true  => Ok(root_dir.to_str().unwrap().to_string()),
        false => {
            match fs::create_dir_all(&root_dir) {
                Ok(_)  => Ok(root_dir.to_str().unwrap().to_string()),
                Err(_) => Err(InitializationError { 
                    message: "Unable to create root directory.".to_string() 
                })
            }
        }
    }
}

fn get_domain() -> Result<String, InitializationError> {
    let mut input = String::new();
    println!("Domain (default is 'localhost'): ");
    match io::stdin().read_line(&mut input) {
        Ok (_) => {},
        Err(_) => return Err(InitializationError { 
            message: "Could not get the domain.".to_string() 
        })
    }

    if input.contains("/") || input.contains("..") {
        return Err(InitializationError{ 
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

fn get_port() -> Result<u16, InitializationError> {
    let mut input = String::new();
    println!("Port (default is '8080'): ");
    match io::stdin().read_line(&mut input) {
        Ok (_) => {},
        Err(_) => return Err(InitializationError { 
            message: "Could not get the port.".to_string() 
        })
    }
    print!("\n");

    if input.trim().is_empty() {
        let port: u16 = DEFAULT_PORT;
        return Ok(port);
    };

    let port: u16 = match input.trim().parse() {
        Ok(p) => p,
        Err(_) => return Err(InitializationError { 
            message: "Expected an integer value.".to_string() 
        })
    };
    Ok(port)
}

fn get_root_dir() -> Result<String, InitializationError> {
    let mut input = String::new();
    println!("Server root directory: ");
    match io::stdin().read_line(&mut input) {
        Ok (_) => {},
        Err(_) => return Err(InitializationError { 
            message: "Could not get the root directory.".to_string() 
        })
    }
    print!("\n");

    if input.trim().is_empty() {
        let root = DEFAULT_ROOT.to_string();
        return Ok(root);
    }

    if input.trim().contains("..") {
        return Err(InitializationError { 
            message: "Server root directory should not contain '..'.".to_string()
        })
    }

    let dir = PathBuf::from(input.trim());

    if !dir.exists() {
        return Err(InitializationError { 
            message: "This directory does not exist.".to_string()
        })
    }

    Ok(dir.to_str().unwrap().to_string())
}

fn generate_config(domain: String, port: u16, root: String, is_for_test: bool) -> String {
    let config = if !is_for_test {
        format!("
            server {{ 
                listen {port};
                server_name {domain};
                add_header X-Server: \"global\";
                add_header X-Security: \"strict\";
            
                location / {{
                    root {root};
                }}
            }}
        ")
    } else {
        format!("
            events {{  }}

            http {{

                server {{ 
                    listen {port};
                    server_name {domain};

                    add_header X-Server: \"global\";
                    add_header X-Security: \"strict\";

                    location / {{
                        root {root};
                    }}
                }}

            }}
        ")
    };
    config
}

// ! creating nginx.conf
fn init_nginx(domain: String, config: String, test_cofing: String, root: String) -> Result<(), InitializationError> {

    // * testing
    let mut test_dir = PathBuf::from("/etc/lpnl/tmp");
    match &test_dir.exists() {
        true  => {},
        false => {
            match fs::create_dir_all(&test_dir) {
                Ok(_)  => {},
                Err(_) => return Err(InitializationError { 
                    message: "Unable to create testing '/tmp' directory.".to_string() 
                })
            }
        }
    }
    test_dir.push("run_test.txt");
    match fs::write(test_dir.clone(), &test_cofing) {
        Ok(_) => {}
        Err(e) => return Err(InitializationError { 
            message: format!("Writing a config into an initialization file failed: {e}.")
        })
    }
    let test_dir_str = match test_dir.to_str() {
        Some(s) => s,
        None => return Err(InitializationError { 
            message: "Initialization path convertion failed.".to_string()
        })
    };
    let mut check_cmd = Command::new("nginx");
    check_cmd.args(["-t", "-c", &test_dir_str]);
    match check_cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("Config file successfully checked.")
            } else {
                let status_code = status.code().unwrap_or_default();
                return Err(InitializationError { 
                    message: format!("Config file checking failed with status code: {status_code}.")
                })
            }
        },
        Err(e) => return Err(InitializationError { 
            message: format!("Config file checking process failed: {e}.")
        })
    }
    match fs::remove_file(test_dir) {
        Ok(_) => {},
        Err(e) => return Err(InitializationError { 
            message: format!("Unable to remove the testing file: {e}.") 
        })
    }

    // * creating a backup file in /etc/lpnl/backups
    let mut backup_dir = PathBuf::from("/etc/lpnl/backups");
    backup_dir.push(&domain);
    let backup_file = format!("{domain}.txt");
    match fs::create_dir_all(&backup_dir) {
        Ok(_) => {},
        Err(e) => return Err(InitializationError { 
            message: format!("Creating '{domain}' backup folder failed: {e}.")
        })
    }
    backup_dir.push(backup_file);
    match fs::write(&backup_dir, &config) {
        Ok(_) => {}
        Err(e) => return Err(InitializationError { 
            message: format!("Writing a config copy into a '{domain}' backup file failed: {e}.")
        })
    }

    // * initializing to sites available
    let mut init_dir = PathBuf::from("/etc/nginx/sites-available");
    let conf_name = format!("{domain}.conf");
    init_dir.push(conf_name);
    match fs::write(init_dir.clone(), &config) {
        Ok(_) => {}
        Err(e) => return Err(InitializationError { 
            message: format!("Writing a config into an initialization file failed: {e}.")
        })
    }
    let mut final_test_cmd = Command::new("nginx");
    final_test_cmd.arg("-t");
    match final_test_cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("Config file successfully tested.")
            } else {
                let status_code = status.code().unwrap_or_default();
                return Err(InitializationError { 
                    message: format!("Config file final testing failed with status code: {status_code}.")
                })
            }
        },
        Err(e) => return Err(InitializationError { 
            message: format!("Config file final testing process failed: {e}.")
        })
    }
    let mut launch_cmd = Command::new("nginx");
    launch_cmd.args(["-s", "reload"]);
    match launch_cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("Config file successfully launched.")
            } else {
                let status_code = status.code().unwrap_or_default();
                return Err(InitializationError { 
                    message: format!("Config file launching failed with status code: {status_code}.")
                })
            }
        },
        Err(e) => return Err(InitializationError { 
            message: format!("Config file launching process failed: {e}.")
        })
    }

    Ok(println!("Generated nginx config to: '{root}'."))
}