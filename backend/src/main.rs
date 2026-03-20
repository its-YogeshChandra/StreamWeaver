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
    println!("Starting backend server...");
    
    //creating a simple webserver
    let listener = TcpListener::bind("0.0.0.0:9000");

    // error handling if error while handling error
    match listener {
        Ok(connection) => {
            println!("Server listening on 0.0.0.0:9000");

            // loop and check the stream from listener
            for stream in connection.incoming() {
                //error handling for stream
                match stream {
                    Ok(streamdata) => {
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
            eprintln!("ERROR: Failed to bind to 0.0.0.0:9000");
            eprintln!("Error details: {}", error);
            std::process::exit(1);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_read = BufReader::new(&mut stream);
    let mut line = String::new();

    // Variables to collect request data
    let mut content_length: usize = 0;
    let mut method = String::new();
    let mut route = String::new();
    let mut httpversion = String::new();
    let mut host = String::new();
    let mut content_type = String::new();
    let mut params_data = String::new();
    let mut is_first_line = true;

    // Step 1: Read all headers
    loop {
        line.clear();
        let bytes_read = match buf_read.read_line(&mut line) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading request line: {}", e);
                return;
            }
        };

        if bytes_read == 0 || line.trim().is_empty() {
            break; // End of headers
        }



        // Parse the first line: "POST /metadata HTTP/1.1"
        if is_first_line {
            is_first_line = false;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() >= 3 {
                method = parts[0].to_string();

                // Parse route and query params
                let full_path = parts[1];
                if let Some(query_start) = full_path.find('?') {
                    route = full_path[..query_start].to_string();
                    params_data = full_path[query_start + 1..].to_string();
                } else {
                    route = full_path.to_string();
                }

                httpversion = parts[2].to_string();
            }

            // Handle OPTIONS preflight request immediately
            if method == "OPTIONS" {
                handle_options_response(stream);
                return;
            }
            continue;
        }

        // Parse headers (case-insensitive matching)
        let line_lower = line.to_lowercase();

        if line_lower.starts_with("content-length:") {
            content_length = line.split(':').nth(1).unwrap().trim().parse().unwrap_or(0);
        } else if line_lower.starts_with("content-type:") {
            content_type = line.split(':').nth(1).unwrap().trim().to_string();
        } else if line_lower.starts_with("host:") {
            host = line.split(':').nth(1).unwrap().trim().to_string();
        }
    }

    // Step 2: Read the body
    let mut body_data = String::new();
    if content_length > 0 {
        let mut body = vec![0u8; content_length];
        if let Err(e) = buf_read.read_exact(&mut body) {
            eprintln!("Error reading request body: {}", e);
            return;
        }
        body_data = String::from_utf8_lossy(&body).to_string();
    }

    // Step 3: Create the Request struct with all parsed values
    let request = Request::new(
        httpversion,
        host,
        route,
        method,
        body_data,
        content_type,
        params_data,
    );



    // Step 4: Drop buf_read to release the borrow on stream
    drop(buf_read);

    // Step 5: Call your router with the populated request
    routes_moderator(request, stream);
}
