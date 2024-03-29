use regex::Regex;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
    path::Path,
};

const STATUS_OK: &str = "200 OK";
const STATUS_NOT_FOUND: &str = "404 NOT FOUND";
const STATUS_SERVER_ERROR: &str = "500 Internal Server Error";

enum RequestStatus {
    Ok,
    NotFound,
    ServerError,
}

struct Response {
    status: RequestStatus,
    content: Vec<u8>,
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = match buf_reader.lines().next() {
        Some(line) => line.unwrap(),
        None => {
            return;
        }
    };

    if cfg!(debug_assertions) {
        println!("Request: {:#?}", http_request);
    }

    let mut parsed_request = parse_request(&http_request);

    let response = match parsed_request.status {
        RequestStatus::Ok => format!(
            "HTTP/1.0 {}\r\nContent-Length: {}\r\n\r\n",
            STATUS_OK,
            parsed_request.content.len(),
        ),
        RequestStatus::NotFound => {
            format!("HTTP/1.0 {}\r\nContent-Length: 0\r\n\r\n", STATUS_NOT_FOUND)
        }
        RequestStatus::ServerError => format!(
            "HTTP/1.0 {}\r\nContent-Length: 0\r\n\r\n",
            STATUS_SERVER_ERROR
        ),
    };
    let mut response = response.into_bytes();
    response.append(&mut parsed_request.content);

    match stream.write_all(&response) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    };
}

fn parse_request(request: &str) -> Response {
    let parsed_path = parse_path(request);
    let path = match parsed_path {
        Some(path) => path,
        None => {
            return Response {
                status: RequestStatus::ServerError,
                content: "".as_bytes().to_vec(),
            };
        }
    };

    if cfg!(debug_assertions) {
        println!("Path: {:#?}", path);
    }

    let path = Path::new(&path);
    if path.is_dir() {
        return Response {
            status: RequestStatus::ServerError,
            content: "".as_bytes().to_vec(),
        };
    }

    match fs::read(path) {
        Ok(content) => Response {
            status: RequestStatus::Ok,
            content,
        },
        Err(_) => Response {
            status: RequestStatus::NotFound,
            content: "".as_bytes().to_vec(),
        },
    }
}

fn parse_path(request: &str) -> Option<String> {
    let re = Regex::new(r"GET /(.*[^/]) HTTP/[12].[01]").unwrap();
    let caps = re.captures(request)?;
    let query = caps.get(1)?.as_str();
    Some(query.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_path("GET / HTTP/1.0"), Some("".to_string()));
        assert_eq!(
            parse_path("GET /dir/index0.html HTTP/1.0"),
            Some("dir/index0.html".to_string())
        );
        assert_eq!(parse_path("GET /.a/ HTTP/1.0"), Some(".a/".to_string()));
        assert_eq!(parse_path("GET /b/ HTTP/1.0"), Some("b/".to_string()));
        assert_eq!(
            parse_path("GET /ind.html/ HTTP/1.0"),
            Some("ind.html/".to_string())
        );
        assert_eq!(parse_path("POST /index.html HTTP/1.0"), None);
        assert_eq!(parse_path("GET /index.html HTTP/1"), None);
    }
}
