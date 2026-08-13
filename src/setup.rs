use crate::constants::{NGINX_SITES_ENABLED_DIR, OS_RELEASE_DIR, LAST_MAJOR_VER, LAST_MINOR_VER, LAST_PATCH_VER, NGINX_CONFIG};
use crate::error::{LpnlError, SetupErrorKind};
use crate::commands::{proceed_nginx, proceed_cmd};
use std::fs::{self, read_to_string};
use std::{path::PathBuf, process::{Command}, str};

pub fn nginx_setup() -> Result<String, LpnlError> {

    let mut nginx_check_cmd = Command::new("nginx");
        nginx_check_cmd.arg("-v");

    match nginx_check_cmd.output() {
        Ok(o) => {
            match str::from_utf8(&o.stderr) {
                Ok(s) => {
                    let str_split = s.split("/");
                    let ver_opt = str_split.last();
                    match ver_opt {
                        Some(ver_str) => {
                            let version = ver_str.trim();
                            println!("Found {:?} version of nginx.", version);
                            let ver_vec: Vec<&str> = version.split(".").collect();
                            let major = match ver_vec.get(0) {
                                Some(ma) => match ma.parse::<u32>() {
                                    Ok(m) => m,
                                    Err(e) => return Err(LpnlError::SetupError { 
                                        message: format!("Unable to convert major version from 'nginx -v': {e}"),
                                        kind: SetupErrorKind::ConvertionFailure
                                    })
                                }
                                None => 0
                            };

                            let minor = match ver_vec.get(1) {
                                Some(mi) => match mi.parse::<u32>() {
                                    Ok(m) => m,
                                    Err(e) => return Err(LpnlError::SetupError { 
                                        message: format!("Unable to convert minor version from 'nginx -v': {e}"),
                                        kind: SetupErrorKind::ConvertionFailure
                                    })
                                }
                                None => 0
                            };
                            
                            let patch = match ver_vec.get(2) {
                                Some(pa) => match pa.parse::<u32>() {
                                    Ok(p) => p,
                                    Err(e) => return Err(LpnlError::SetupError { 
                                        message: format!("Unable to convert patch version from 'nginx -v': {e}"),
                                        kind: SetupErrorKind::ConvertionFailure
                                    })
                                }
                                None => 0
                            };

                            if (major, minor, patch) < (LAST_MAJOR_VER, LAST_MINOR_VER, LAST_PATCH_VER) {
                                return install_nginx()
                            } else { return proceed() }
                        }
                        None => return Err(LpnlError::SetupError {
                            message: "Nginx version not found.".to_string(),
                            kind: SetupErrorKind::NotFound
                        })
                    }
                },
                Err(e) => return Err(LpnlError::SetupError { 
                    message: format!("Unable to convert stdout from 'nginx -v': {e}"),
                    kind: SetupErrorKind::ConvertionFailure
                })
            }
        },
        Err(_) => {
            println!("Nginx not found. Downloading nginx.");
            return install_nginx()
        }
    }
}

fn proceed() -> Result<String, LpnlError> {

    let sites_enabled_dir = PathBuf::from(NGINX_SITES_ENABLED_DIR);
    if !sites_enabled_dir.exists() {
        match fs::create_dir_all(sites_enabled_dir) {
            Ok(_) => {}
            Err(e) => return Err(LpnlError::SetupError { 
                message: format!("Unable to create a config files' folder in '{NGINX_SITES_ENABLED_DIR}': {e}"),
                kind: SetupErrorKind::FsFailure
            })
        }
    }
    
    let conf_path = PathBuf::from(NGINX_CONFIG);
    let conf_as_str = match read_to_string(&conf_path) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::SetupError { 
            message: format!("Unable to read the '{NGINX_CONFIG}' to get the config: {e}"),
            kind: SetupErrorKind::ReadError
        })
    };

    let sites_enabled_str = format!("{NGINX_SITES_ENABLED_DIR}/*.conf;");

    if conf_as_str.lines().find(|&l| l.contains(&sites_enabled_str)).is_none() {
        match conf_as_str.lines().find(|&l| l.trim().starts_with("http")) {
            Some(l) => {
                let conf_part = format!("http {{ include {sites_enabled_str}");
                let final_conf = conf_as_str.replace(l, &conf_part);
                match fs::write(conf_path, final_conf) {
                    Ok(_) => {}
                    Err(e) => return Err(LpnlError::SetupError { 
                        message: format!("Unable to update the config in '{NGINX_CONFIG}': {e}"),
                        kind: SetupErrorKind::FsFailure
                    })
                }
            }
            None => return Err(LpnlError::SetupError { 
                message: format!("Unable to find the 'http' block in '{NGINX_CONFIG}'."),
                kind: SetupErrorKind::NotFound
            })
        }
    }

    match proceed_nginx() {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    Ok("Setup complete!".to_string())
}

fn install_nginx() -> Result<String, LpnlError> {
    let os_release_path = PathBuf::from(OS_RELEASE_DIR);
    let os_release_str = match read_to_string(&os_release_path) {
        Ok(s) => s,
        Err(e) => return Err(LpnlError::SetupError { 
            message: format!("Unable to read the '/etc/os-release' to get the distro: {e}"),
            kind: SetupErrorKind::ReadError
        })
    };

    let id = match os_release_str.lines().find(|&l| l.contains("ID=\"") ) {
        Some(i) => i,
        None => return Err(LpnlError::SetupError { 
            message: "Unable to find the distro name.".to_string(),
            kind: SetupErrorKind::NotFound
        })
    };

    let id_like = match os_release_str.lines().find(|&l| l.contains("ID_LIKE=\"") ) {
        Some(i) => i,
        None => return Err(LpnlError::SetupError { 
            message: "Unable to find the distro name.".to_string(),
            kind: SetupErrorKind::NotFound
        })
    };


    if id_like.contains("arch") {

        let mut pacman_install = Command::new("pacman");
          pacman_install.args(["-S", "nginx","--noconfirm"]);
        match proceed_cmd(
            &mut pacman_install, 
            "Nginx downloaded succesfully.".to_string(), 
            "Nginx downloading".to_string(), 
            "Unable to download nginx".to_string()
        ) {
            Ok(_) => return proceed(),
            Err(e) => return Err(e)
        }

    } else if id_like.contains("debian") {

        let mut apt_update = Command::new("apt");
          apt_update.arg("update");
        match proceed_cmd(
            &mut apt_update, 
            "Updated 'apt' repositories successfully.".to_string(), 
            "Updating 'apt' repositories".to_string(), 
            "Unable to update 'apt' repositories".to_string()
        ) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        let mut apt_install = Command::new("apt");
          apt_install.args(["install", "-y", "nginx"]);
        match proceed_cmd(
            &mut apt_install, 
            "Nginx downloaded succesfully.".to_string(), 
            "Nginx downloading".to_string(), 
            "Unable to download nginx".to_string()
        ) {
            Ok(_) => return proceed(),
            Err(e) => return Err(e)
        }

    } else if id_like.contains("fedora") {
        
        let mut dnf_install = Command::new("dnf");
          dnf_install.args(["install", "-y", "nginx"]);
        match proceed_cmd(
            &mut dnf_install, 
            "Nginx downloaded succesfully.".to_string(), 
            "Nginx downloading".to_string(), 
            "Unable to download nginx".to_string()
        ) {
            Ok(_) => return proceed(),
            Err(e) => return Err(e)
        }

    } else if id_like.contains("gentoo") {

        return Err(LpnlError::SetupError { 
            message: "Gentoo is not supported. (yet)".to_string(), 
            kind: SetupErrorKind::NotFound
        })

    } else if id.contains("nixos") {

        return Err(LpnlError::SetupError { 
            message: "NixOS is not supported. (yet)".to_string(), 
            kind: SetupErrorKind::NotFound
        })

    } else {

        return Err(LpnlError::SetupError { 
            message: "Unsupported distro. (yet)".to_string(), 
            kind: SetupErrorKind::NotFound
        })

    }
}