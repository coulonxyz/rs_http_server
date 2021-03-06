use super::server::Handler;
use crate::http::{Request, Response, StatusCode, Method};
use std::fs;

#[derive(Debug)]
pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Reversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method()  {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(body) => Response::new(StatusCode::Ok, Some(body)),
                    None => Response::new(StatusCode::NotFound, Some("<h1>404 -> Cant' find this mofo</h1>".to_string())),
                }
            }
            _ => Response::new(StatusCode::NotFound, Some("<h1>404</h1>".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_file_test_html() {
        let default_path = format!("{}/test_fixtures", env!("CARGO_MANIFEST_DIR"));
        let handler = WebsiteHandler::new(default_path);
        assert_eq!(handler.read_file("test.html"), Some("<html lang=\"en\"></html>\n".to_string()));
    }

    #[test]
    fn none_if_directory_reversal_attack() {
        let handler = WebsiteHandler::new("some_path".to_string());
        assert_eq!(handler.read_file("../Cargo.toml"), None);
    }

    #[test]
    fn none_if_invalid_path() {
        let handler = WebsiteHandler::new("some_path".to_string());
        assert_eq!(handler.read_file("some invalid path ????"), None);
    }
}
