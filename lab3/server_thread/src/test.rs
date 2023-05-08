mod tests {
    use crate::parse_request;

    #[test]
    fn test_parse() {
        assert_eq!(parse_request("GET / HTTP/1.0"), Some("".to_string()));
        assert_eq!(
            parse_request("GET /dir/index0.html HTTP/1.0"),
            Some("dir/index0.html".to_string())
        );
        assert_eq!(parse_request("GET /.a/ HTTP/1.0"), Some(".a/".to_string()));
        assert_eq!(parse_request("GET /b/ HTTP/1.0"), Some("b/".to_string()));
        assert_eq!(
            parse_request("GET /ind.html/ HTTP/1.0"),
            Some("ind.html/".to_string())
        );
        assert_eq!(parse_request("POST /index.html HTTP/1.0"), None);
        assert_eq!(parse_request("GET /index.html HTTP/1"), None);
    }
}
