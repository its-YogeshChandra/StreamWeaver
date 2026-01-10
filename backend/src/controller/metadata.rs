//controller for getting meta data
//bringig the reuest sturct  crate ;
use crate::utils::handle_response;
use std::net::TcpStream;
use std::process::Stdio;
use std::result;
// bringing  sirealization crate serde
use crate::services::format_handler;
use crate::utils::errorhandler;
use crate::utils::{Request, Response, ResponseBody, json_deserializer};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
pub async fn meta_data_and_options(request: Request, stream: TcpStream) -> () {
    // get the data from the request
    let ResponseBody { data }: ResponseBody<Value> = json_deserializer(&request.body_data);

    //match the data for;
    let iterable_data = data.as_object();

    //create value through the Data
    // let mut body_data = Data {
    //     url: String::from("marco")
    // };

    //create a map of serde value to put the dat into
    let obj = serde_json::Map::new();
    let mut body_data = Value::Object(obj);

    //match for the key
    let keys = ["url"];
    if let Some(data) = iterable_data {
        //iterate the keys array
        for key in keys {
            //match the key with the obj
            if !data.contains_key(key) {
                //throw the error
                let error = format!("{} not found ", key);
                errorhandler(&stream, &error)
            } else {
                // add the data into the data value obj
                body_data[key] = data[key].clone()
            }
        }
    }

    //call the youtube api for the metadata available
    let video_url = body_data["url"].as_str().expect("expecting a string");
    println!("video_url : {}", &video_url);

    //call the yt-dlp
    let ytdlp_process = Command::new("yt-dlp")
        .arg("--list-formats")
        .arg(video_url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute the process");

    //fix the data to that string
    let output = String::from_utf8_lossy(&ytdlp_process.stdout);

    //fetch the error
    let error = String::from_utf8_lossy(&ytdlp_process.stderr);

    // check if error isn't empty
    if !error.is_empty() {
        //call the error function
        for lines in error.lines() {
            match &lines {
                //if lines start with ERROR then throw error
                s if s.starts_with("ERROR") => errorhandler(&stream, &s),
                _ => println!("readin error"),
            }
        }
    }

    let mut result_output: Vec<String> = Vec::new();
    //check if error present and then send the error
    if !output.is_empty() {
        result_output = format_handler(&output.to_string(), &stream)
            .expect("error while getting value from format handler ");
    } else {
        println!("option is empty ");
    }

    //make struct to return
    let message = String::from("format successfully received");
    let response = Response::new_struct(true, message, result_output);
    handle_response(response, stream);
}
