use crate::error::{LpnlError, CommandErrorKind};
use std::{ process::Command};

pub fn proceed_nginx() -> Result<(), LpnlError> {
    let mut test_cmd = Command::new("nginx");
    test_cmd.arg("-t");
    match proceed_cmd(
        &mut test_cmd, 
        "Config file tested succesfully.".to_string(), 
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
        "Config file launched succesfully.".to_string(), 
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
        "Config file tested succesfully.".to_string(), 
        "Config file testing".to_string(),
        "Unable to test the config file.".to_string()
    ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

pub fn proceed_nginx_with_dir(path: &str) -> Result<(), LpnlError> {
    let mut test_cmd = Command::new("nginx");
    test_cmd.args(["-t", "-c", path]);
    match proceed_cmd(
        &mut test_cmd, 
        "Config file tested succesfully.".to_string(), 
        "Config file testing".to_string(),
        "Unable to test the config file.".to_string()
    ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}