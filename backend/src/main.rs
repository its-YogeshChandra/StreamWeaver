use std::clone;
use std::io::{BufRead, BufReader, Read, Write};
// use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::num::ParseIntError;
//import the route module from routes folder
mod routes;
//import utils module
mod utils;
//import controller module
mod controller;
//import services module
mod services;

use crate::routes::routes_moderator;
use crate::utils::{Request, handle_options_response, handle_response};

//main funciton that run the program
fn main() {
    //creating a simple webserver
    let listener = TcpListener::bind("127.0.0.1:8080");

    // error handling if error while handling error
    match listener {
        Ok(connection) => {
            println! {"connection successfully established"};

            // loop and check the stream from listener
            for stream in connection.incoming() {
                //error handling for stream
                match stream {
                    Ok(streamdata) => {
                        println! {
                        " data stream is receiving : {:?}", streamdata};

                        //connect to the handling stream
                        handle_connection(streamdata);
                    }
                    Err(error) => {
                        eprint!("error while getting stream ");
                        eprint!("error: {}", error);
                    }
                }
            }
        }
        Err(error) => {
            println! {"error while establishing connection "};
            println! {"error: {}", error};
        }
    }
}

//option using impl
//
// fn handle_connection(mut stream: TcpStream) {
//     //creating buffer
//     //Buffers in Rust: In Rust, a buffer is typically a block of memory used for temporary storage of data. Buffers are commonly used when reading or writing data to or from sources like files, network sockets, or memory
//
//     let mut reader = BufReader::new(&mut stream);
//
//     //read value
//
//     //make instance of request and update the values
//     let mut request_data = Request::new(
//         "random".to_string(),
//         "random".to_string(),
//         "random".to_string(),
//         "random".to_string(),
//         "random".to_string(),
//         "random".to_string(),
//         "random".to_string(),
//     );
//
//     //read the request line
//     let mut request_line = String::new();
//     if reader.read_line(&mut request_line).unwrap() == 0 {
//         //eror while reading valule
//         println!("error while reading value");
//         return;
//     }
//
//     //parse the first line
//     let firstline: Vec<&str> = request_line.split_whitespace().collect();
//     println!("first line is : {:?}", firstline);
//
//     if firstline.len() > 3 {
//         //print th vaule
//         println!("first line value : {:?}", firstline);
//
//         request_data.method = firstline[0].to_string();
//         request_data.route = firstline[1].to_string();
//         request_data.httpversion = firstline[2].to_string();
//     }
//
//     //for content length
//     let mut content_length = 0;
//     loop {
//         let mut line = String::new();
//         let bytes_read = reader.read_line(&mut line).unwrap();
//
//         if bytes_read == 0 || line == "\r\n" {
//             break;
//         }
//         //parse handler
//         if line.to_lowercase().starts_with("content_length:") {
//             let parts: Vec<&str> = line.split_whitespace().collect();
//             if parts.len() > 1 {
//                 content_length = parts[1].parse::<usize>().unwrap();
//             } else if line.starts_with("Content-Type:") {
//                 request_data.content_type = line.trim().to_string();
//             } else if line.starts_with("Host: ") {
//                 request_data.host = line.trim().to_string();
//             }
//         }
//     }
//
//     if content_length > 0 {
//         let mut body_buffer = vec![0; content_length];
//
//         //read the block until we get all the bytes
//         reader.read_exact(&mut body_buffer).unwrap();
//         request_data.body_data = String::from_utf8_lossy(&body_buffer).to_string();
//     }
//
//     //println the final rqeust object
//     println!("final request object : {:?}", request_data);
//
//     //drop the reaqder to release borrow on stream
//     drop(reader);
//
//     //call the router function
//     routes_moderator(request_data, stream);
// }

//request format
//HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body/
//
fn handle_connection(mut stream: TcpStream) {
    let mut buf_read = BufReader::new(&mut stream);
    let mut line = String::new();
    let mut content_length: usize = 0;

    // Step 1: Read all headers
    loop {
        line.clear();
        let bytes_read = buf_read.read_line(&mut line).unwrap();

        if bytes_read == 0 || line.trim().is_empty() {
            break; // End of headers
        }

        println!("Received: {}", line);

        // Step 2: Parse Content-Length header
        if line.to_lowercase().starts_with("content-length:") {
            content_length = line.split(':').nth(1).unwrap().trim().parse().unwrap_or(0);
        }

        if line.starts_with("OPTIONS") {
            handle_options_response(stream);
            return;
        }
    }

    // Step 3: Read the body (THIS IS WHAT YOU'RE MISSING!)
    if content_length > 0 {
        let mut body = vec![0u8; content_length];
        buf_read.read_exact(&mut body).unwrap();

        let body_str = String::from_utf8(body).unwrap();
        println!("Body: {}", body_str); // ← Now you'll see your JSON!
    }
}
