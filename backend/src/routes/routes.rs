use crate::controller::{extractor, meta_data_and_options, send_data};
use crate::utils::Request;
use crate::utils::errorhandler::errorhandler;
use std::net::TcpStream;

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

    if method == "POST" || path == "GET" || path == "DELETE" || path == "PUT" {
        match path.as_str() {
            "/create" => send_data(request, stream),
            "/metadata" => meta_data_and_options(request, stream),
            "/extractor" => extractor(request, stream),
            _ => errorhandler(&stream, blank_route_error.as_str()),
        };
    } else {
        let error = "invalid method";
        errorhandler(&stream, &error)
    }
}
