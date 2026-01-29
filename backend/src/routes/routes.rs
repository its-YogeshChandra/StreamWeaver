use crate::controller::{extractor, meta_data_and_options, send_data};
use crate::utils::Request;
use crate::utils::errorhandler::errorhandler;
use crate::utils::responsesystem::handle_options_response;
use std::io::Write;
use std::net::TcpStream;

// ===========================================
// NEW: Health check response for GET / and /health
// Added to fix Cloudflare tunnel 502 errors (health checks)
// ===========================================
fn handle_health_check(mut stream: TcpStream) {
    let body = r#"{"status":"ok","message":"StreamWeaver backend is running"}"#;
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

//
// pub fn routes_creator(request: Request) -> Vec<RouteData> {
//     // call the routes_data struct with instance function
//     let httpverb = String::from("POST");
//     let path = String::from("/Random");
//     let function_string = String::from("send_data");
//
//     let responseroute = RouteData::new(path, send_data(request), httpverb);
//     //return the vector out of it
//     let resultantpaths = vec![responseroute];
//     resultantpaths
// }

pub fn routes_moderator(request: Request, stream: TcpStream) -> () {
    //check the path in the request object and  then add respective function to it;
    let path = &request.route;
    let method = &request.method;

    //match the route and call differnt function
    let blank_route_error = String::from("route not found");
    
    // ===========================================
    // OLD CODE (without GET route support):
    // if method == "POST" {
    //     match path.as_str() {
    //         "/create" => send_data(request, stream),
    //         "/metadata" => meta_data_and_options(request, stream),
    //         "/extractor" => extractor(request, stream),
    //         _ => errorhandler(&stream, blank_route_error.as_str()),
    //     };
    // } else if method == "OPTIONS" {
    //     handle_options_response(stream);
    // } else {
    //     let error = "invalid method";
    //     errorhandler(&stream, &error)
    // }
    // ===========================================
    
    // NEW CODE START - Added GET route support for health checks
    if method == "POST" {
        match path.as_str() {
            "/create" => send_data(request, stream),
            "/metadata" => meta_data_and_options(request, stream),
            "/extractor" => extractor(request, stream),
            _ => errorhandler(&stream, blank_route_error.as_str()),
        };
    } else if method == "GET" {
        // NEW: Handle GET requests for health checks
        match path.as_str() {
            "/" | "/health" => handle_health_check(stream),
            _ => errorhandler(&stream, blank_route_error.as_str()),
        };
    } else if method == "OPTIONS" {
        handle_options_response(stream);
    } else {
        let error = "invalid method";
        errorhandler(&stream, &error)
    }
    // NEW CODE END
}
