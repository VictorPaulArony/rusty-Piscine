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
        Security::NotFound => match server {
            Ok(url) => url.to_string(),
            Err(msg) => format!("Not found: {}", msg),
        },
        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(msg) => msg.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_data_unknown() {
        let server = Ok("http://example.com");
        let result = fetch_data(server, Security::Unknown);
        assert_eq!(result, "http://example.com");
    }

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_fetch_data_message_panic() {
        let server = Err("error");
        fetch_data(server, Security::Message);
    }

    #[test]
    fn test_fetch_data_message() {
        let server = Ok("http://example.com");
        let result = fetch_data(server, Security::Message);
        assert_eq!(result, "http://example.com");
    }

    #[test]
    fn test_fetch_data_warning() {
        let server = Err("error");
        let result = fetch_data(server, Security::Warning);
        assert_eq!(result, "WARNING: check the server");
    }

    #[test]
    fn test_fetch_data_not_found_ok() {
        let server = Ok("http://example.com");
        let result = fetch_data(server, Security::NotFound);
        assert_eq!(result, "http://example.com");
    }

    #[test]
    fn test_fetch_data_not_found_err() {
        let server = Err("not found");
        let result = fetch_data(server, Security::NotFound);
        assert_eq!(result, "Not found: not found");
    }

    #[test]
    #[should_panic(expected = "http://example.com")]
    fn test_fetch_data_unexpected_url_panic() {
        let server = Ok("http://example.com");
        fetch_data(server, Security::UnexpectedUrl);
    }

    #[test]
    fn test_fetch_data_unexpected_url_err() {
        let server = Err("unexpected url");
        let result = fetch_data(server, Security::UnexpectedUrl);
        assert_eq!(result, "unexpected url");
    }
}

