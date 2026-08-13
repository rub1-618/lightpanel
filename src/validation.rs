use crate::error::{LpnlError, ValidationErrorKind};
use std::{io, path::PathBuf};
use url::Url;

pub fn domain_validates(domain: String) -> bool {
    if domain.trim().contains("..") || domain.trim().is_empty() {
        false
    } else { true }
}

pub fn port_validates(port: u16) -> bool {
    if port == 0 {
        return false
    }
    true
}

pub fn root_validates(root: PathBuf) -> bool {
    if root.exists() {
        let str = root.to_str().unwrap().to_string();
        if str.trim().contains("..") || str.trim().is_empty() {
            return false
        }
        return true
    } else { false }
}

pub fn proxy_validates(nproxy: String) -> bool {
    match Url::parse(&nproxy) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn location_validates(loc: String) -> bool {
    if loc.trim().contains("..") || loc.trim().is_empty() || !loc.trim().starts_with("/")  {
        return false
    } else if loc.trim() == "/" {
        return false
    } else { true }
}

pub fn get_domain(domain: Option<String>) -> Result<String, LpnlError> {

    if let Some(d) = domain {
        if domain_validates(d.clone()) {
            return Ok(d)
        } else {
            return Err(LpnlError::ValidationError{
                message: "'..' and empty strings are not allowed.".to_string(),
                kind: ValidationErrorKind::InvalidDomain
            })
        }
    }

    loop {
        let mut input = String::new();
        println!("Domain (default is 'localhost'): ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(e) => return Err(LpnlError::ValidationError { 
                message: format!("Unable to get the domain: {e}"), 
                kind: ValidationErrorKind::IoFailure
            })
        }
        print!("\n");

        if !domain_validates(input.clone()) {
            eprintln!(" '..' and empty strings are not allowed.");
            continue;
        }

        let domain = input.trim().to_string();
        return Ok(domain)
    }
}

pub fn get_root(root: Option<PathBuf>) -> Result<String, LpnlError> {

    if let Some(r) = root {
        if root_validates(r.clone()) {
            return Ok(r.to_str().unwrap().to_string())
        } else {
            return Err(LpnlError::ValidationError{
                message: "This root directory does not exist, contains '..' or is an empty string.".to_string(),
                kind: ValidationErrorKind::InvalidRoot
            })
        }
    }

    loop {
        let mut input = String::new();
        println!("Root directory: ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(e) => return Err(LpnlError::ValidationError { 
                message: format!("Unable to get the root directory: {e}"), 
                kind: ValidationErrorKind::IoFailure
            })
        }
        print!("\n");

        let dir = PathBuf::from(input.trim());

        if !root_validates(dir.clone()) {
            eprintln!("This root directory does not exist, contains '..' or is an empty string.");
            continue;
        }

        return Ok(dir.to_str().unwrap().to_string())
    }
}

#[allow(dead_code)]
pub fn get_port() -> Result<u16, LpnlError> {
    loop {
        let mut input = String::new();
        println!("Port: ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(_) => return Err(LpnlError::ValidationError { 
                message: "Could not get the port.".to_string(), 
                kind: ValidationErrorKind::IoFailure
            })
        }
        print!("\n");

        let port: u16 = match input.trim().parse() {
            Ok(p) => p,
            Err(_) => {eprintln!("Expected an 'u16' integer value."); continue;}
        };

        if !port_validates(port) {
            eprintln!("Port cannot be zero."); 
            continue;
        }

        return Ok(port)
    }
}

pub fn get_proxy(proxy: Option<String>) -> Result<String, LpnlError> {

    if let Some(p) = proxy {
        if !proxy_validates(p.to_string()) {
            return Err(LpnlError::ValidationError{
                message: "Not a valid proxy URL.".to_string(),
                kind: ValidationErrorKind::InvalidProxy
            })
        } else {
            return Ok(p);
        }
    }
    loop {
        let mut input = String::new();
        println!("Proxy link: ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(e) => return Err(LpnlError::ValidationError { 
                message: format!("Unable to get the proxy link: {e}"), 
                kind: ValidationErrorKind::IoFailure
            })
        }
        print!("\n");

        if !proxy_validates(input.clone()) {
            eprintln!("Not a valid proxy URL.");
            continue;
        }

        let proxy = input.trim().to_string();
        return Ok(proxy)
    }
}

pub fn get_location(location: Option<String>) -> Result<String, LpnlError> {

    if let Some(loc) = location {
        if location_validates(loc.clone()) {
            return Ok(loc)
        } else {
            return Err(LpnlError::ValidationError{
                message: "This location contains '..', does not start with '/', is an empty string or is a root directory.".to_string(),
                kind: ValidationErrorKind::InvalidLocation
            })
        }
    }

    loop {
        let mut input = String::new();
        println!("Location's directory: ");
        match io::stdin().read_line(&mut input) {
            Ok (_) => {},
            Err(e) => return Err(LpnlError::ValidationError { 
                message: format!("Unable to get the location: {e}"), 
                kind: ValidationErrorKind::IoFailure
            })
        }
        print!("\n");

        if !location_validates(input.clone()) {
            eprintln!("This location contains '..', does not start with '/', is an empty string or is a root directory.");
            continue;
        }

        return Ok(input.trim().to_string())
    }
}