use regex::Regex;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

const STATUS_OK: &str = "200 OK";
const STATUS_NOT_FOUND: &str = "404 NOT FOUND";
const STATUS_SERVER_ERROR: &str = "500 Internal Server Error";

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {:#?}", http_request);

    let response = match parse_request(http_request.as_str()) {
        Some(query) => {
            let meta = fs::metadata(&query[..]).unwrap();
            if meta.is_file() {
                let contents = fs::read_to_string(query).unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.0 {}\r\nContent-Length: {}\r\n\r\n{}",
                    STATUS_OK, length, contents
                )
            } else {
                format!("HTTP/1.0 {}\r\nContent-Length: 0\r\n\r\n", STATUS_NOT_FOUND)
            }
        }
        None => format!(
            "HTTP/1.0 {}\r\nContent-Length: 0\r\n\r\n",
            STATUS_SERVER_ERROR
        ),
    };
    stream.write_all(response.as_bytes()).unwrap();
}

pub fn parse_request(request: &str) -> Option<String> {
    let re = Regex::new(r"GET /(.*) HTTP/1.[01]").unwrap();
    let caps = re.captures(request)?;
    let query = caps.get(1)?.as_str();
    Some(query.to_string())
}
