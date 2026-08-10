use std::{fs, io, path::PathBuf, process::Command};

use crate::error::{InitErrorKind, LpnlError};

const LPNL_DIR_STR: &str = "/etc/lpnl";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_DOMAIN: &str = "localhost";
const DEFAULT_ROOT: &str = "/var/www";

pub fn init_lpnl(is_default: bool) -> Result<(), LpnlError> {
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
        match get_root_dir(domain.clone()) {
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

fn init_backup_dir(domain: String) -> Result<(), LpnlError> {
    let mut backup_dir = PathBuf::from(LPNL_DIR_STR);
    backup_dir.push("backups");

    let domain_dir = PathBuf::from(&domain);
    let mut current_backup_dir = backup_dir.clone();
    current_backup_dir.push(&domain_dir);
    if current_backup_dir.exists() { 
        return Err(LpnlError::InitError { 
            message: "This config is already initialized.".to_string(), 
            kind: InitErrorKind::AlreadyExists
        })
    }
    match fs::create_dir(current_backup_dir) {
        Ok(_)  => Ok(()),
        Err(_) => Err(LpnlError::InitError { 
            message: format!("Unable to create '{domain}' backup directory."), 
            kind: InitErrorKind::FsFailure
        })
    }
}

// returns String so that root path can be used in main initialization later
fn init_root_dir(domain: &str) -> Result<String, LpnlError> {
    let mut root_dir = PathBuf::from(DEFAULT_ROOT);
    let domain_as_dir = PathBuf::from(domain);
    root_dir.push(domain_as_dir);
    match &root_dir.exists() {
        true  => Ok(root_dir.to_str().unwrap().to_string()),
        false => {
            match fs::create_dir_all(&root_dir) {
                Ok(_)  => Ok(root_dir.to_str().unwrap().to_string()),
                Err(_) => Err(LpnlError::InitError { 
                    message: "Unable to create root directory.".to_string(), 
                    kind: InitErrorKind::FsFailure
                })
            }
        }
    }
}

fn get_domain() -> Result<String, LpnlError> {
    loop {
        let mut input = String::new();
        println!("Domain (default is 'localhost'): ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(_) => return Err(LpnlError::InitError { 
                message: "Could not get the domain.".to_string(), 
                kind: InitErrorKind::IoFailure
            })
        }
        print!("\n");

        if input.contains("/") || input.contains("..") {
            eprintln!("'/' and '..' are not allowed.");
            continue;
        }

        if input.trim().is_empty() {
            let domain: String = DEFAULT_DOMAIN.to_string();
            return Ok(domain);
        };

        let domain = input.trim().to_string();
        return Ok(domain)
    }
}

fn get_port() -> Result<u16, LpnlError> {
    loop {
        let mut input = String::new();
        println!("Port (default is '8080'): ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(_) => return Err(LpnlError::InitError { 
                message: "Could not get the port.".to_string(), 
                kind: InitErrorKind::IoFailure
            })
        }
        print!("\n");

        if input.trim().is_empty() {
            let port: u16 = DEFAULT_PORT;
            return Ok(port);
        };

        let port: u16 = match input.trim().parse() {
            Ok(p) => p,
            Err(_) => {eprintln!("Expected an integer value."); continue;}
        };

        return Ok(port)
    }
}

fn get_root_dir(domain: String) -> Result<String, LpnlError> {
    loop {
        let mut input = String::new();
        println!("Server root directory (default is '/var/www'): ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(_) => return Err(LpnlError::InitError { 
                message: "Could not get the root directory.".to_string(), 
                kind: InitErrorKind::FsFailure
            })
        }
        print!("\n");

        if input.trim().is_empty() {
            return init_root_dir(&domain)
        }

        if input.trim().contains("..") {
            eprintln!("Server root directory should not contain '..'.");
            continue;
        }

        let dir = PathBuf::from(input.trim());

        if !dir.exists() {
            eprintln!("This directory does not exist.");
            continue;
        }

        return Ok(dir.to_str().unwrap().to_string())
    }
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
fn init_nginx(domain: String, config: String, test_cofing: String, root: String) -> Result<(), LpnlError> {

    // * testing
    let mut test_dir = PathBuf::from("/etc/lpnl/tmp");
    match &test_dir.exists() {
        true  => {},
        false => {
            match fs::create_dir_all(&test_dir) {
                Ok(_)  => {},
                Err(_) => return Err(LpnlError::InitError { 
                    message: "Unable to create testing '/tmp' directory.".to_string(), 
                    kind: InitErrorKind::FsFailure
                })
            }
        }
    }
    test_dir.push("run_test.txt");
    match fs::write(test_dir.clone(), &test_cofing) {
        Ok(_) => {}
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Writing a config into an initialization file failed: {e}."),
            kind: InitErrorKind::FsFailure
        })
    }
    let test_dir_str = match test_dir.to_str() {
        Some(s) => s,
        None => return Err(LpnlError::InitError { 
            message: "Initialization path convertion failed.".to_string(),
            kind: InitErrorKind::ConvertionFailure
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
                return Err(LpnlError::InitError { 
                    message: format!("Config file checking failed with status code: {status_code}."),
                    kind: InitErrorKind::InvalidCmdResult
                })
            }
        },
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Config file checking process failed: {e}."),
            kind: InitErrorKind::InvalidCmdResult
        })
    }
    match fs::remove_file(test_dir) {
        Ok(_) => {},
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Unable to remove the testing file: {e}."),
            kind: InitErrorKind::InvalidCmdResult
        })
    }

    // * creating a backup file in /etc/lpnl/backups
    let mut backup_dir = PathBuf::from("/etc/lpnl/backups");
    backup_dir.push(&domain);
    let backup_file = format!("{domain}.txt");
    match fs::create_dir_all(&backup_dir) {
        Ok(_) => {},
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Creating '{domain}' backup folder failed: {e}."),
            kind: InitErrorKind::FsFailure
        })
    }
    backup_dir.push(backup_file);
    match fs::write(&backup_dir, &config) {
        Ok(_) => {}
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Writing a config copy into a '{domain}' backup file failed: {e}."),
            kind: InitErrorKind::FsFailure
        })
    }

    // * initializing to sites enabled
    let mut init_dir = PathBuf::from("/etc/nginx/sites-enabled");
    let conf_name = format!("{domain}.conf");
    init_dir.push(conf_name);
    match fs::write(init_dir.clone(), &config) {
        Ok(_) => {}
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Writing a config into an initialization file failed: {e}."),
            kind: InitErrorKind::FsFailure
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
                return Err(LpnlError::InitError { 
                    message: format!("Config file final testing failed with status code: {status_code}."),
                    kind: InitErrorKind::InvalidCmdResult
                })
            }
        },
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Config file final testing process failed: {e}."),
            kind: InitErrorKind::InvalidCmdResult
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
                return Err(LpnlError::InitError { 
                    message: format!("Config file launching failed with status code: {status_code}."),
                    kind: InitErrorKind::InvalidCmdResult
                })
            }
        },
        Err(e) => return Err(LpnlError::InitError { 
            message: format!("Config file launching process failed: {e}."),
            kind: InitErrorKind::InvalidCmdResult
        })
    }

    Ok(println!("Generated nginx config to: '{root}'."))
}

#[cfg(test)]
mod tests {
    use super::*;

    // * domain logic test copy
    fn check_domain(domain: String) -> Result<String, LpnlError> {
        if domain.contains("/") || domain.contains("..") {
            return Err(LpnlError::InitError { 
                message: "'/', '..' and empty strings are not allowed.".to_string(), 
                kind: InitErrorKind::InvalidDomain
            })
        }

        if domain.trim().is_empty() {
            let domain: String = DEFAULT_DOMAIN.to_string();
            return Ok(domain);
        };

        let domain = domain.trim().to_string();
        Ok(domain)
    }

    // * port logic test copy
    fn check_port(port: String) -> Result<u16, LpnlError>  {
        if port.trim().is_empty() {
            let port: u16 = DEFAULT_PORT;
            return Ok(port);
        };

        let uport: u16 = match port.trim().parse() {
            Ok(p) => p,
            Err(_) => {
                return Err(LpnlError::InitError{
                    message: "Expected an integer value.".to_string(),
                    kind: InitErrorKind::InvalidPort
                })
            }
        };

        return Ok(uport)
    }

    // * root logic test copy
    fn check_root(root: String) -> Result<String, LpnlError> {
    if root.trim().is_empty() {
            return init_root_dir(&root)
        }

        if root.trim().contains("..") {
            return Err(LpnlError::InitError {
                message: "Server root directory should not contain '..'.".to_string(),
                kind: InitErrorKind::InvalidRoot
            })
        }

        let dir = PathBuf::from(root.trim());

        return Ok(dir.to_str().unwrap().to_string())
    }

    // ! domain tests

    #[test]
    fn test_init_get_domain_ok() {
        let domain = check_domain("hello_world".to_string());
        match domain {
            Err(_) => panic!("Unexpected Error."),
            Ok(d) => assert_eq!(d, "hello_world".to_string())
        }
    }

    #[test]
    fn test_init_get_domain_err() {
        let domain = check_domain("/..domain".to_string());
        match domain {
            Err(e) => {
                match e {
                    LpnlError::InitError { kind, .. } => {
                        assert!(matches!(kind, InitErrorKind::InvalidDomain))
                    }
                    _ => panic!("Expected InitError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_init_default_domain() {
        let domain = check_domain("".to_string());
        match domain {
            Err(_) => panic!("Unexpected Error."),
            Ok(d) => assert_eq!(d, "localhost")
        }
    }

    // ! port tests

    #[test]
    fn test_init_get_port_ok() {
        let port = check_port("80".to_string());
        match port {
            Err(_) => panic!("Unexpected Error."),
            Ok(p) => assert_eq!(p, 80)
        }
    }

    #[test]
    fn test_init_get_port_err() {
        let port = check_port("hello_world".to_string());
        match port {
            Err(e) => {
                match e {
                    LpnlError::InitError { kind, .. } => {
                        assert!(matches!(kind, InitErrorKind::InvalidPort))
                    }
                    _ => panic!("Expected InitError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_init_default_port() {
        let port = check_port("".to_string());
        match port {
            Err(_) => panic!("Unexpected Error."),
            Ok(s) => assert_eq!(s, 8080)
        }
    }

    // ! root tests

    #[test]
    fn test_init_get_root_ok() {
        let root = check_root("/var/www".to_string());
        match root {
            Err(_) => panic!("Unexpected Error."),
            Ok(d) => assert_eq!(d, "/var/www".to_string())
        }
    }

    #[test]
    fn test_init_get_root_err() {
        let root = check_root("/../hello".to_string());
        match root {
            Err(e) => {
                match e {
                    LpnlError::InitError { kind, .. } => {
                        assert!(matches!(kind, InitErrorKind::InvalidRoot))
                    }
                    _ => panic!("Expected InitError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_init_default_root() {
        let root = check_root("".to_string());
        match root {
            Err(_) => panic!("Unexpected Error."),
            Ok(r) => assert_eq!(r, "/var/www/")
        }
    }
}