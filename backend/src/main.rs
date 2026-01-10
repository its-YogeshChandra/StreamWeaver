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
use crate::utils::Request;

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

fn handle_connection(mut stream: TcpStream) {
    //creating buffer
    //Buffers in Rust: In Rust, a buffer is typically a block of memory used for temporary storage of data. Buffers are commonly used when reading or writing data to or from sources like files, network sockets, or memory
    let mut buffer = [0; 512];

    let mut reader = BufReader::new(&mut stream);

    // stream.read(&mut buffer).unwrap();
    //
    // println!("raw stream data : {:?}", &stream);

    //read the buffer put and seperate different parts of it

    let request = String::from_utf8_lossy(&buffer[..]);

    //print the request
    println!("Request : {}", request);

    //make instance of request and update the values
    let mut request_data = Request::new(
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
    );

    //read the request line
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).unwrap() == 0 {
        return;
    }

    //parse the first line
    let firstline: Vec<&str> = request_line.split_whitespace().collect();
    if firstline.len() > 3 {
        request_data.method = firstline[0].to_string();
        request_data.route = firstline[1].to_string();
        request_data.httpversion = firstline[2].to_string();
    }

    //for content length
    let mut content_length = 0;
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).unwrap();

        if bytes_read == 0 || line == "\r\n" {
            break;
        }
        //parse handler
    }

    //extract the values from the request
    for part in request.split("\r\n") {
        println!("------printing parts ------");
        println! {"{}", part};
        println!("------------");

        //match the data and update the request_data object
        if part.starts_with("GET")
            || part.starts_with("POST")
            || part.starts_with("DELETE")
            || part.starts_with("PUT")
            || part.starts_with("OPTIONS")
        {
            for items in part.split(" ") {
                match items {
                    "GET" | "POST" | "PUT" | "DELETE" | "OPTIONS" => {
                        request_data.method = items.to_string()
                    }
                    s if s.starts_with("HTTP") => request_data.httpversion = s.to_string(),
                    m if m.starts_with("/") => request_data.route = m.to_string(),
                    _ => {
                        eprintln!("the data is empty");
                    }
                }
            }
        }
        // for content type
        else if part.starts_with("Content-Type") {
            request_data.content_type = part.to_string()
        }
        // for host
        else if part.starts_with("Host") {
            for items in part.split(" ") {
                println!("items: {}", items);
                match items {
                    s if s.starts_with("1") => request_data.host = s.to_string(),
                    _ => {
                        println!(" ");
                    }
                }
            }
        }
        //for body data if present
        else if part.starts_with("{") {
            request_data.body_data = part.to_string()
        }
    }

    //call the route moderator function
    routes_moderator(request_data, stream);
}

//request format
//HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
