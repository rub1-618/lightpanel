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
pub fn get_port(port: Option<u16>) -> Result<u16, LpnlError> {

    if let Some(p) = port {
        if port_validates(p.clone()) {
            return Ok(p)
        } else {
            return Err(LpnlError::ValidationError{
                message: "Port should be an 'u16' integer value and not zero.".to_string(),
                kind: ValidationErrorKind::InvalidPort
            })
        }
    }

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
            Err(_) => {eprintln!("Port should be an 'u16' integer value."); continue;}
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::error::{LpnlError, ValidationErrorKind};
    use crate::validation::{
        get_domain, domain_validates,
        get_port, port_validates,
        get_root, root_validates,
        get_proxy, proxy_validates,
        get_location, location_validates,
    };

    // ! domain tests

    #[test]
    fn test_get_domain_ok() {
        let domain = get_domain(Some("example.com".to_string()));
        match domain {
            Err(_) => panic!("Unexpected Error."),
            Ok(d) => assert_eq!(d, "example.com".to_string())
        }
    }

    #[test]
    fn test_get_domain_err() {
        let domain = get_domain(Some("/..domain".to_string()));
        match domain {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidDomain))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_get_domain_empty_err() {
        let domain = get_domain(Some("".to_string()));
        match domain {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidDomain))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_domain_validation_ok() {
        let domain = "example.com".to_string();
        assert_eq!(domain_validates(domain), true);
    }

    #[test]
    fn test_domain_validation_err() {
        let domain = "..".to_string();
        assert_eq!(domain_validates(domain), false);
    }

    #[test]
    fn test_domain_validation_empty_err() {
        let domain = "".to_string();
        assert_eq!(domain_validates(domain), false);
    }

    // ! port tests

    #[test]
    fn test_get_port_ok() {
        let port = get_port(Some(80));
        match port {
            Err(_) => panic!("Unexpected Error."),
            Ok(p) => assert_eq!(p, 80)
        }
    }

    #[test]
    fn test_get_port_err() {
        let port = get_port(Some(0));
        match port {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidPort))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_port_validation_empty_ok() {
        let port: u16 = 8080;
        assert_eq!(port_validates(port), true);
    }

    #[test]
    fn test_port_validation_empty_err() {
        let port: u16 = 0;
        assert_eq!(port_validates(port), false);
    }

    // ! root tests

    #[test]
    fn test_get_root_ok() {
        let valid_path = PathBuf::from("/etc");
        let root = get_root(Some(valid_path));
        match root {
            Err(_) => panic!("Unexpected Error."),
            Ok(r) => assert_eq!(r, "/etc".to_string())
        }
    }

    #[test]
    fn test_get_root_err() {
        let invalid_path = PathBuf::from("/../hello");
        let root = get_root(Some(invalid_path));
        match root {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidRoot))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_root_validation_ok() {
        let root = PathBuf::from("/etc");
        assert_eq!(root_validates(root), true);
    }

    #[test]
    fn test_root_validation_err() {
        let root = PathBuf::from("..");
        assert_eq!(root_validates(root), false);
    }

    #[test]
    fn test_root_validation_empty_err() {
        let root = PathBuf::from("");
        assert_eq!(root_validates(root), false);
    }

    #[test]
    fn test_root_validation_does_not_exist_err() {
        let root = PathBuf::from("/dir-that-does-not-exist");
        assert_eq!(root_validates(root), false);
    }

    // ! proxy tests

    #[test]
    fn test_get_proxy_ok() {
        let proxy = get_proxy(Some("localhost:8080".to_string()));
        match proxy {
            Err(_) => panic!("Unexpected Error."),
            Ok(p) => assert_eq!(p, "localhost:8080".to_string())
        }
    }

    #[test]
    fn test_get_proxy_err() {
        let proxy = get_proxy(Some("invalid_proxy".to_string()));
        match proxy {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidProxy))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_proxy_validation_ok() {
        let proxy = "http://example.com".to_string();
        assert_eq!(proxy_validates(proxy), true);
    }

    #[test]
    fn test_proxy_validation_err() {
        let proxy = "".to_string();
        assert_eq!(proxy_validates(proxy), false);
    }

    // ! location tests

    #[test]
    fn test_get_location_ok() {
        let location = get_location(Some("/api".to_string()));
        match location {
            Err(_) => panic!("Unexpected Error."),
            Ok(l) => assert_eq!(l, "/api".to_string())
        }
    }

    #[test]
    fn test_get_location_err() {
        let location = get_location(Some("/..".to_string()));
        match location {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidLocation))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_get_location_empty_err() {
        let location = get_location(Some("".to_string()));
        match location {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidLocation))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_get_location_no_slash_err() {
        let location = get_location(Some("invalid_location".to_string()));
        match location {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidLocation))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_get_location_is_root_err() {
        let location = get_location(Some("/".to_string()));
        match location {
            Err(e) => {
                match e {
                    LpnlError::ValidationError { kind, .. } => {
                        assert!(matches!(kind, ValidationErrorKind::InvalidLocation))
                    }
                    _ => panic!("Expected ValidationError.")
                }
            }
            Ok(_) => panic!("Expected Error.")
        }
    }

    #[test]
    fn test_location_validation_ok() {
        let location = "/api".to_string();
        assert_eq!(location_validates(location), true);
    }

    #[test]
    fn test_location_validation_err() {
        let location = "/..".to_string();
        assert_eq!(location_validates(location), false);
    }

    #[test]
    fn test_location_validation_empty_err() {
        let location = "".to_string();
        assert_eq!(location_validates(location), false);
    }

    #[test]
    fn test_location_validation_no_slash_err() {
        let location = "invalid_location".to_string();
        assert_eq!(location_validates(location), false);
    }

    #[test]
    fn test_location_validation_is_root_err() {
        let location = "/".to_string();
        assert_eq!(location_validates(location), false);
    }
}