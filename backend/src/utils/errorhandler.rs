use std::io::Write;
use std::net::TcpStream;

//error response handler
pub fn errorhandler(mut stream: &TcpStream, error: &str) {
    let error = error.to_string();

    let body = format!("{{\"error\" : \"{}\"}}", error);
    //creating a error response
    let response = format!(
        "HTTP/1.1 400 Request Error\r\n\
        Content-Type: application/json\r\n\
        Content-Length : {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        body.len(),
        body
    );

    // sending data through stream
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
