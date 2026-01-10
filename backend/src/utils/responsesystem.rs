// response struct and its functions
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;

//derive the serialize
#[derive(Serialize, Deserialize, Debug)]

//define the trait object

//Response struct
pub struct Response<T: Serialize> {
    success: bool,
    message: String,
    data: T,
}

impl<T: Serialize> Response<T> {
    pub fn new_struct(success: bool, message: String, data: T) -> Self {
        Self {
            success: success,
            message: message,
            data,
        }
    }
}

//function to send the response
pub fn handle_response<T: Serialize>(response: Response<T>, mut stream: TcpStream) {
    // use serde to serealize the data
    let response_data =
        serde_json::to_string(&response).expect("error while making json response ");

    //make the response string from it
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
        Access-Control-Allow-Headers: Content-Type\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        response_data.len(),
        response_data
    ); // send the data to the stream
    stream
        .write_all(response.as_bytes())
        .expect("error while writing");

    //flush the stream
    stream.flush().expect("error while sending data")
}

pub fn handle_options_response(mut stream: TcpStream) {
    //format of response
    let response = "HTTP/1.1 200 OK\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
Access-Control-Allow-Headers: Content-Type\r\n\
Content-Length: 0\r\n\
\r\n";

    //send the response back
    stream.write_all(response.as_bytes()).unwrap();
    return;
}

//handle tar reponse
pub fn handle_tar_response(mut stream: TcpStream, tar_buffer: Vec<u8>) {
    let headers = format!(
        "HTTP/1.1 200 OK\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
        Access-Control-Allow-Headers: Content-Type\r\n\
        Content-Type: application/x-tar\r\n\
        Content-Disposition: attachment; filename=\"backup.tar\"\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n",
        tar_buffer.len() // Use the byte length of the buffer
    );

    // Step A: Write Headers first (as bytes)
    stream
        .write_all(headers.as_bytes())
        .expect("error writing headers");

    // Step B: Write the Binary Body separately
    stream.write_all(&tar_buffer).expect("error writing body");

    stream.flush().expect("error flushing stream");
}
