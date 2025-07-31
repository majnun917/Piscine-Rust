pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound =>
            match server {
                Ok(url) => url.to_string(),
                Err(e) => format!("Not found: {}", e),
            }
        Security::UnexpectedUrl => panic!("{}", server.unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_expect() {
        fetch_data(Err(""), Security::Message);
    }
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\"")]
    fn test_unwrap() {
        fetch_data(Err("ERROR CRITICAL"), Security::Unknown);
    }
    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn test_unwrap_err() {
        fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
    }
    #[test]
    fn test_unwrap_or() {
        assert_eq!(fetch_data(Err(""), Security::Warning), "WARNING: check the server".to_string());
    }
    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(
            fetch_data(Err("another_server.com"), Security::NotFound),
            "Not found: another_server.com".to_string()
        );
    }
    #[test]
    fn test_ok() {
        assert_eq!(fetch_data(Ok("server.com"), Security::Message), "server.com");
        assert_eq!(fetch_data(Ok("server.com"), Security::Warning), "server.com");
        assert_eq!(fetch_data(Ok("server.com"), Security::NotFound), "server.com");
        assert_eq!(fetch_data(Ok("server.com"), Security::Unknown), "server.com");
    }
}
