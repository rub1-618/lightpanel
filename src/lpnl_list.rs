use crate::constants::{NGINX_SITES_ENABLED_DIR, NGINX_SITES_DISABLED_DIR, LPNL_BACKUP_DIR};
use crate::error::{LpnlError, ListErrorKind};
use std::fs;
use std::path::{PathBuf};



pub fn list_enabled() -> Result<String, LpnlError> {
    if let Err(e) = check_sites_enabled() {
        return Err(e)
    }

    let enabled =  match get_list(
        NGINX_SITES_ENABLED_DIR, 
        "You have 0 enabled config files.", 
        "enabled config files"
    ) {
        Ok(b) => b,
        Err(e) => return Err(e)
    };
    Ok(enabled)
}

pub fn list_disabled() -> Result<String, LpnlError> {
    if let Err(e) = check_sites_disabled() {
        return Err(e)
    }

    let disabled =  match get_list(
        NGINX_SITES_DISABLED_DIR, 
        "You have 0 disabled config files.\n", 
        "disabled config files"
    ) {
        Ok(b) => b,
        Err(e) => return Err(e)
    };
    Ok(disabled)
}

pub fn list_backups() -> Result<String, LpnlError> {
    let backups = match get_list(
        LPNL_BACKUP_DIR, 
        "You have 0 backups.", 
        "backup config copies"
    ) {
        Ok(b) => b,
        Err(e) => return Err(e)
    };
    Ok(backups)
}

fn get_list(dir: &str, message_one: &str, message_two: &str) -> Result<String, LpnlError> {
    let mut str = String::new();
    let mut count: u32 = 0;
    for entry in fs::read_dir(dir).map_err(|e| LpnlError::ListError{
        message: format!("Unable to read '{dir}': {e}"),
        kind: ListErrorKind::FsFailure
    })? {
        match entry {
            Ok(entry) => {
                let file_name = match PathBuf::from(&entry.path()).file_name() {
                    Some(e) => format!("{}", e.to_string_lossy()),
                    None => "unknown".to_string()
                };
                let fname = format!("  - {}\n", file_name);
                str.push_str(&fname);
                count += 1;
            },
            Err(e) => {
                count += 1;
                let funknown = format!("  - unknown: {e}\n");
                str.push_str(&funknown);
            }
        }
    }

    if count == 0 {
        return Ok(message_one.to_string())
    }

    Ok(format!("{count} {message_two} found:\n{}", str))
}

fn check_sites_enabled() -> Result<(), LpnlError> {
    let sites_enabled_dir = PathBuf::from(NGINX_SITES_ENABLED_DIR);
    if !sites_enabled_dir.exists() {
        return Err(LpnlError::ListError { 
            message: format!("Unable to find an enabled configs files' folder in '{NGINX_SITES_ENABLED_DIR}'. Suggest using setup command."),
            kind: ListErrorKind::FsFailure
        })
    }
    Ok(())
}

fn check_sites_disabled() -> Result<(), LpnlError> {
    let sites_disabled_dir = PathBuf::from(NGINX_SITES_DISABLED_DIR);
    if !sites_disabled_dir.exists() {
        return Err(LpnlError::ListError { 
            message: format!("Unable to find a disabled configs files' folder in '{NGINX_SITES_DISABLED_DIR}'. Suggest using setup command."),
            kind: ListErrorKind::FsFailure
        })
    }
    Ok(())
}