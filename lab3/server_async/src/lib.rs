use regex::Regex;
use std::path::Path;
use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
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
    content: String,
}

pub async fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request = String::new();
    buf_reader.read_line(&mut http_request).await.unwrap();

    if cfg!(debug_assertions) {
        println!("Request: {:#?}", http_request);
    }

    let parsed_request = parse_request(&http_request).await;

    let response = match parsed_request.status {
        RequestStatus::Ok => format!(
            "HTTP/1.0 {}\r\nContent-Length: {}\r\n\r\n{}",
            STATUS_OK,
            parsed_request.content.len(),
            parsed_request.content
        ),
        RequestStatus::NotFound => {
            format!("HTTP/1.0 {}\r\nContent-Length: 0\r\n\r\n", STATUS_NOT_FOUND)
        }
        RequestStatus::ServerError => format!(
            "HTTP/1.0 {}\r\nContent-Length: 0\r\n\r\n",
            STATUS_SERVER_ERROR
        ),
    };

    stream.write_all(response.as_bytes()).await.unwrap();
}

async fn parse_request(request: &str) -> Response {
    let parsed_path = parse_path(request);
    let path = match parsed_path {
        Some(path) => path,
        None => {
            return Response {
                status: RequestStatus::ServerError,
                content: "".to_string(),
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
            content: "".to_string(),
        };
    }

    match fs::read_to_string(path).await {
        Ok(content) => Response {
            status: RequestStatus::Ok,
            content,
        },
        Err(_) => Response {
            status: RequestStatus::NotFound,
            content: "".to_string(),
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
