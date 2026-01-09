//bringig the reuest sturct  crate ;
use crate::utils::Request;
use crate::utils::Response;
use crate::utils::handle_response;
use std::net::TcpStream;
// bringing  sirealization crate serde
use crate::utils::errorhandler;
use crate::utils::json_deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize)]

// response data that has to be send
pub struct Data {
    name: String,
    username: String,
    age: i32,
}

// impl data for Data block
impl Data {
    pub fn new(name: String, username: String, age: i32) -> Self {
        Self {
            name,
            username,
            age,
        }
    }
}

// controller function that send_data when called with certain data;
pub fn send_data(request: Request, stream: TcpStream) -> () {
    // match the argument with the defined set
    let user = json_deserializer(&request.body_data);

    //take the data out of it
    let body: Value = user.data;

    //make a value obj to put on the json value for the key
    let obj = serde_json::Map::new();
    let mut body_data = Value::Object(obj);

    // create a match system that match for the particular keys in the gained struct data;
    let keys = ["name"];
    if let Some(obj) = body.as_object() {
        for key in keys {
            if !obj.contains_key(key) {
                let error = format!("key is missing : {}", key);
                errorhandler(&stream, &error);
            } else {
                match key {
                    "name" => body_data["name"] = obj[key].clone(),
                    _ => {
                        println!("error value of the obj ");
                    }
                }
            }
        }
    }

    // check that if data field match to this name ;
    println!("body_data : {}", body_data["name"]);
    if body_data["name"] == "adam" {
        let adam = Data::new("adam".to_string(), "Levine".to_string(), 21);

        //create the response struct
        let message = String::from("successfully send data");
        let response = Response::new_struct(true, message, adam);

        handle_response(response, stream)
    } else {
        let error_val = String::from("name doesn't match to the payload");

        //calling the error handler
        errorhandler(&stream, &error_val);
    }
}
