use crate::main;
use crate::services::{code_handler, codeextractor};
use crate::utils::{
    Request, Response, ResponseBody, errorhandler, handle_response, json_deserializer,
};
use core::error;
use serde_json::Value;
use std::fs::create_dir_all;
use std::io::{stderr, stdout};
use std::path;
use std::path::Path;
use std::process::Stdio;
use std::{collections::HashMap, net::TcpStream, process::Command};
#[tokio::main]
pub async fn extractor(request: Request, stream: TcpStream) {
    //get the data from the reqeust
    let ResponseBody { data }: ResponseBody<Value> = json_deserializer(&request.body_data);

    // make the object out of the data
    let iterable_data = data.as_object();

    let mut main_data: HashMap<String, String> = HashMap::new();

    // check for the keys in the data
    let keys = ["bitrate", "url", "content-length", "vcodec"];
    let vcodec: Vec<&str> = vec!["avc1.64002a", "av01.0.09M.08"];
    if let Some(data) = iterable_data {
        //iterate the value
        for key in keys {
            //check if fetched object has particular key
            if !data.contains_key(key) {
                //throw the error to the frontend
                let error = format!("{}: key is missing", key,);
                errorhandler(&stream, &error)
            } else {
                match (key, &data[key]) {
                    //add the bitrate
                    ("bitrate", Value::String(s))
                        if matches!(
                            s.as_str(),
                            "1080p" | "720p" | "480p" | "360p" | "240p" | "144p"
                        ) =>
                    {
                        main_data.insert(key.to_string(), s.clone());
                    }

                    // add the url
                    ("url", Value::String(s)) if s.starts_with("http") => {
                        main_data.insert(key.to_string(), s.clone());
                    }

                    //add the content-length
                    ("content-length", Value::String(s)) => {
                        if s.parse::<u64>().is_ok() {
                            main_data.insert(key.to_string(), s.clone());
                        } else {
                            let error = "content length must be a number";
                            errorhandler(&stream, error);
                        }
                    }

                    //add the video codec
                    ("vcodec", Value::String(s)) if vcodec.contains(&s.as_str()) => {
                        main_data.insert(key.to_string(), s.clone());
                    }

                    ("bitrate", _) => {
                        errorhandler(&stream, "invalid bitrate value");
                        return;
                    }

                    ("url", _) => {
                        errorhandler(&stream, "invalid url value");
                        return;
                    }
                    ("content-length", _) => {
                        errorhandler(&stream, "invalid content-length value");
                    }

                    ("vcodec", _) => {
                        errorhandler(&stream, "invalid vcodec value");
                        return;
                    }
                    (_, _) => errorhandler(&stream, "invalid payload"),
                }
            }
        }
    }

    //call the yt-dlp
    let video_url = match main_data.get("url") {
        Some(url) => url,
        None => {
            let error = String::from("url key missing");
            errorhandler(&stream, &error);
            return;
        }
    };

    //call the ytdlp_process to fetch the data
    let ytdlp_process = Command::new("yt-dlp")
        .arg("--list-formats")
        .arg(video_url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute the process");

    //fix the data to that string
    let output = String::from_utf8_lossy(&ytdlp_process.stdout);

    //get the error
    let error = String::from_utf8_lossy(&ytdlp_process.stderr);

    //fetch the important code according to the
    if !output.is_empty() {
        //call the codeextractor
        //the bitrate
        let bitrate = match main_data.get("bitrate") {
            Some(bitrate) => bitrate,
            None => {
                let error = String::from("birate key is missing");
                errorhandler(&stream, &error);
                return;
            }
        };

        //the vidoe codec
        let vcodec = match main_data.get("vcodec") {
            Some(vcodec) => vcodec,
            None => {
                let error = String::from("vcodec key is missing");
                errorhandler(&stream, &error);
                return;
            }
        };

        //destructure the tuple
        let (vidcode, audcode) = code_handler(&output.to_string(), bitrate, vcodec);

        let combined_code = format! {"{vidcode} + {audcode}"};

        let video_download_folder = "public";
        let _ = std::fs::create_dir_all(video_download_folder);

        // Define output file template
        let output_template = format!("{}/%(title)s.%(ext)s", video_download_folder);

        //download the video using the codes
        let downlaoder_ytdlp = Command::new("yt-dlp")
            .arg("-f")
            .arg(combined_code)
            .arg(video_url)
            .arg("-o")
            .arg(output_template)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("failed to execute the process");

        //get the output
        let downloader_output = String::from_utf8_lossy(&downlaoder_ytdlp.stdout);

        //get the error if any
        let downloader_error = String::from_utf8_lossy(&downlaoder_ytdlp.stderr);
        if !downloader_error.is_empty() {
            println!("downloader_error: {}", downloader_error)
        }

        if !downloader_output.is_empty() {
            println!("downloader_output: {}", downloader_output);
            //make the path buff
            let mut main_path = std::path::PathBuf::from("public");

            //output dir
            let fileval = read_dir(video_download_folder.to_string());
            main_path.push(fileval.expect("fileval might not be a string"));

            //call the chunking function
            chunk_video(main_path.to_string_lossy().to_string(), vcodec.to_string());
        }
    }
}

//function to change the video into chunks
//the length of chunks will be specified by client

fn chunk_video(video_path: String, vidcodec: String) {
    //create the output dir
    let vidoutput = "vidoutput";
    std::fs::create_dir_all(vidoutput).expect("failed to create dir");

    let final_destination = Path::new(vidoutput).join("index.m3u8");

    //call the ffmpeg on video
    let options = [
        "-preset medium",
        "-crf 24",
        //"-vf scale=-2:720", // Scale to 720p height, maintain aspect ratio
        //"-b:v 2500k",     // Video bitrate for 720p
        "-maxrate 45000k", // Maximum bitrate
        "-bufsize 5000k",  // Buffer size
        "-b:a 128k",       // Audio bitrate
        "-ar 44100",       // Audio sample rate
        "-start_number 0",
        //"-hls_time 10",
        "-hls_list_size 0",
        "-hls_playlist_type vod",
        "-f hls",
    ];
    //vector of options
    let mut vecoptions: Vec<String> = Vec::new();
    for option in options {
        let individual_val = option.split_whitespace();
        for part in individual_val {
            vecoptions.push(part.to_string());
        }
    }

    let mut video_chunker = Command::new("ffmpeg");

    video_chunker
        .arg("-i")
        .arg(video_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac");

    // .arg(output_options)
    // .arg(vidoutput)
    // .stdout(Stdio::piped())
    // .stderr(Stdio::piped())
    // .output()
    // .expect("failed to run ffmped");
    //
    // //iterate over the option and feed it to video_chunker
    // for option in options {
    //     //individual value
    //     let individual_val = option.split_whitespace();
    //     video_chunker.args(individual_val);
    // }

    let cmd = video_chunker
        .arg(final_destination)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run ffmpeg");

    println!("cmd is called ");
    //check the output and the erorr
    if !cmd.stdout.is_empty() {
        println!("ffmpeg output is : {:?}", cmd.stdout)
    }

    if !cmd.stderr.is_empty() {
        let error = String::from_utf8_lossy(&cmd.stderr);
        for line in error.lines() {
            println!("ffmpeg error is : {:?}", line)
        }
    }
}

//function to get path from folder
fn read_dir(folder_string: String) -> std::io::Result<String> {
    let mut fileval = String::new();

    for entry in std::fs::read_dir(folder_string)? {
        let entry = entry?; // handle Result
        let path = entry.path();
        if let Some(file_name) = path.file_name() {
            println!("{}", file_name.to_string_lossy());
            fileval = file_name.to_string_lossy().into_owned();
        }
    }

    Ok(fileval)
}
