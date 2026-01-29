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
    let listener = TcpListener::bind("0.0.0.0:8080");

    // error handling if error while handling error
    match listener {
        Ok(connection) => {
            println!("Server listening on 0.0.0.0:8080");

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
            eprintln!("ERROR: Failed to bind to 0.0.0.0:8080");
            eprintln!("Error details: {}", error);
            std::process::exit(1);
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

        // ===========================================
        // NEW: Safe header parsing with if-let instead of unwrap()
        // OLD CODE (commented out - caused "option is empty" panic):
        // if line_lower.starts_with("content-length:") {
        //     content_length = line.split(':').nth(1).unwrap().trim().parse().unwrap_or(0);
        // } else if line_lower.starts_with("content-type:") {
        //     content_type = line.split(':').nth(1).unwrap().trim().to_string();
        // } else if line_lower.starts_with("host:") {
        //     host = line.split(':').nth(1).unwrap().trim().to_string();
        // }
        // ===========================================
        
        // NEW CODE START
        if line_lower.starts_with("content-length:") {
            if let Some(val) = line.split(':').nth(1) {
                content_length = val.trim().parse().unwrap_or(0);
            }
        } else if line_lower.starts_with("content-type:") {
            if let Some(val) = line.split(':').nth(1) {
                content_type = val.trim().to_string();
            }
        } else if line_lower.starts_with("host:") {
            // Host header may contain port (e.g., "localhost:8080"), so join remaining parts
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() > 1 {
                host = parts[1].trim().to_string();
            }
        } else if line_lower.starts_with("transfer-encoding:") {
            // Check for chunked transfer encoding
            if line_lower.contains("chunked") {
                eprintln!("[DEBUG] Chunked transfer encoding detected");
            }
        }
        // NEW CODE END

        // Log all headers for debugging (can be removed in production)
        eprintln!("[DEBUG] Header: {}", line.trim());
    }

    // Step 2: Read the body
    let mut body_data = String::new();
    if content_length > 0 {
        let mut body = vec![0u8; content_length];
        if let Err(e) = buf_read.read_exact(&mut body) {
            eprintln!("[ERROR] Error reading request body: {}", e);
            return;
        }
        body_data = String::from_utf8_lossy(&body).to_string();
    } else if method == "POST" {
        // ===========================================
        // NEW: Fallback for POST without Content-Length (e.g., chunked encoding)
        // This handles cases where Cloudflare tunnel might strip Content-Length
        // ===========================================
        eprintln!("[DEBUG] POST with no Content-Length, attempting to read available data...");
        let mut temp_buf = [0u8; 8192];
        match buf_read.read(&mut temp_buf) {
            Ok(n) if n > 0 => {
                body_data = String::from_utf8_lossy(&temp_buf[..n]).to_string();
                eprintln!("[DEBUG] Read {} bytes without Content-Length", n);
            }
            Ok(_) => {
                eprintln!("[DEBUG] No data available to read");
            }
            Err(e) => {
                eprintln!("[DEBUG] Error reading: {}", e);
            }
        }
    }

    // Debug logging
    eprintln!("[DEBUG] Method: {}, Route: {}, Content-Length: {}", method, route, content_length);
    eprintln!("[DEBUG] Body data (first 200 chars): {}", &body_data.chars().take(200).collect::<String>());

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
