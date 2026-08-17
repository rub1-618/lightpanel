use crate::{constants::TEST_FILE_DIR, error::{CommandErrorKind, LpnlError}};
use std::{ fs, process::Command};

pub fn proceed_nginx() -> Result<(), LpnlError> {
    let mut test_cmd = Command::new("nginx");
    test_cmd.arg("-t");
    match proceed_cmd(
        &mut test_cmd, 
        "Config file tested successfully.".to_string(), 
        "Config file testing".to_string(),
        "Unable to test the config file.".to_string()
    ) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    let mut launch_cmd = Command::new("nginx");
    launch_cmd.args(["-s", "reload"]);
    match proceed_cmd(
        &mut launch_cmd, 
        "Config file launched successfully.".to_string(), 
        "Config file launching".to_string(),
        "Unable to launch the config file.".to_string()
    ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

pub fn proceed_cmd(command: &mut Command, first_message: String, second_emessage: String, third_emessage: String) -> Result<(), LpnlError> {
    match command.status() {
        Ok(status) => {
            if status.success() {
                Ok(println!("{}", first_message))
            } else {
                let status_code = match status.code() {
                    Some(c) => format!("{}", c),
                    None => "unknown".to_string()
                };
                return Err(LpnlError::CommandError { 
                    message: format!("{second_emessage} failed with status code: {status_code}"), 
                    kind: CommandErrorKind::InvalidCmdResult
                })
            }
        }
        Err(e) => return Err(LpnlError::CommandError { 
            message: format!("{third_emessage}: {e}"), 
            kind: CommandErrorKind::InvalidCmdResult
        })
    }
}

pub fn proceed_check_nginx() -> Result<(), LpnlError> {
    let mut test_cmd = Command::new("nginx");
    test_cmd.arg("-t");
    match proceed_cmd(
        &mut test_cmd, 
        "Config file tested successfully.".to_string(), 
        "Config file testing".to_string(),
        "Unable to test the config file.".to_string()
    ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

pub fn proceed_check_nginx_tmp(contents: &str) -> Result<(), LpnlError> {
    let path = &TEST_FILE_DIR.to_string();

    match fs::write(path, contents) {
        Ok(_) => {}
        Err(e) => return Err(LpnlError::CommandError { 
            message: format!("Unable to create a test file: {e}"), 
            kind: CommandErrorKind::FsFailure
        })
    }

    let mut test_cmd = Command::new("nginx");
    test_cmd.args(["-t", "-c", path]);
    match proceed_cmd(
        &mut test_cmd, 
        "Config file tested successfully.".to_string(), 
        "Config file testing".to_string(),
        "Unable to test the config file.".to_string()
    ) {
        Ok(_) => {},
        Err(e) => {
            match fs::remove_file(path) {
                Ok(_) => {},
                Err(e) => return Err(LpnlError::CommandError { 
                    message: format!("Unable to remove a test file: {e}"), 
                    kind: CommandErrorKind::FsFailure
                })
            }
            return Err(e)
        }
    }

    match fs::remove_file(path) {
        Ok(_) => {},
        Err(e) => return Err(LpnlError::CommandError { 
            message: format!("Unable to remove a test file: {e}"), 
            kind: CommandErrorKind::FsFailure
        })
    }

    Ok(())
}