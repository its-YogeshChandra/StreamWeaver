extern crate tar;
use crate::services::code_handler;
use crate::utils::{Request, ResponseBody, errorhandler, handle_tar_response, json_deserializer};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Stdio;
use std::{collections::HashMap, net::TcpStream, process::Command};
use tar::Builder;

#[tokio::main]
pub async fn extractor(request: Request, stream: TcpStream) {
    //get the data from the reqeust
    let ResponseBody { data }: ResponseBody<Value> = json_deserializer(&request.body_data);

    // make the object out of the data
    let iterable_data = data.as_object();

    let mut main_data: HashMap<String, String> = HashMap::new();

    // check for the keys in the data
    let keys = ["bitrate", "url", "content_length", "vcodec"];
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

                    //add the content_length
                    ("content_length", Value::String(s)) => {
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
                    ("content_length", _) => {
                        errorhandler(&stream, "invalid content_length value");
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

        //the content_length
        let content_length = match main_data.get("content_length") {
            Some(content_length) => content_length,
            None => {
                let error = String::from("content length is missing");
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
            chunk_video(
                main_path.to_string_lossy().to_string(),
                vcodec.to_string(),
                bitrate.to_string(),
                content_length.to_string(),
            );

            //start creating a tar file
            let mut buffer = Vec::new();
            {
                let mut archive = Builder::new(&mut buffer);
                let dirpath = Path::new("vidoutput");
                let _ = archive.append_dir_all("playlist", dirpath);
                let _ = archive.finish();
            }
            //delete the public and vidoutput folder
            let vidoutputpath = Path::new("vidoutput");
            let publicpath = Path::new("public");
            let _ = fs::remove_dir_all(vidoutputpath);
            let _ = fs::remove_dir_all(publicpath);

            //call the handle tar response
            handle_tar_response(stream, buffer);
        }
    }
}

//create struct to handle the bitrate
#[derive(Debug, Clone)]
pub struct VideoConfig {
    pub bitrate: String,
    pub maxrate: String,
    pub bufsize: String,
    pub scale_filter: String,
}

impl VideoConfig {
    // This method takes a string slice and returns the Config
    // We returns Option<Self> because the user might send an invalid resolution
    pub fn from_resolution(resolution: &str) -> Option<Self> {
        match resolution {
            "4k" | "2160p" => Some(Self {
                bitrate: "45000k".to_string(),
                maxrate: "48000k".to_string(),
                bufsize: "90000k".to_string(),
                scale_filter: "scale=-2:2160".to_string(),
            }),
            "1080p" => Some(Self {
                bitrate: "8000k".to_string(),
                maxrate: "8560k".to_string(),
                bufsize: "16000k".to_string(),
                scale_filter: "scale=-2:1080".to_string(),
            }),
            "720p" => Some(Self {
                bitrate: "2500k".to_string(),
                maxrate: "2675k".to_string(),
                bufsize: "5000k".to_string(),
                scale_filter: "scale=-2:720".to_string(),
            }),
            "480p" => Some(Self {
                bitrate: "1400k".to_string(),
                maxrate: "1498k".to_string(),
                bufsize: "2800k".to_string(),
                scale_filter: "scale=-2:480".to_string(),
            }),
            "360p" => Some(Self {
                bitrate: "800k".to_string(),
                maxrate: "856k".to_string(),
                bufsize: "1600k".to_string(),
                scale_filter: "scale=-2:360".to_string(),
            }),
            _ => None, // Handle invalid input
        }
    }
}

//function to change the video into chunks
//the length of chunks will be specified by client

fn chunk_video(video_path: String, vidcodec: String, bitrate: String, content_length: String) {
    //create the output dir
    let vidoutput = "vidoutput";
    std::fs::create_dir_all(vidoutput).expect("failed to create dir");

    let final_destination = Path::new(vidoutput).join("index.m3u8");

    //call the ffmpeg on video
    let options = [
        "-preset medium",
        "-crf 24",
        "-b:a 128k", // Audio bitrate
        "-ar 44100", // Audio sample rate
        "-start_number 0",
        "-hls_list_size 0",
        "-hls_playlist_type vod",
        "-f hls",
    ];

    //vector of options
    let mut vid_options: Vec<String> = Vec::new();
    for option in options {
        let individual_val = option.split_whitespace();
        for part in individual_val {
            vid_options.push(part.to_string());
        }
    }

    //get the bitrate , maxrate and buffer size accroding to the resolution by user
    let vidconfigval = match VideoConfig::from_resolution(&bitrate) {
        Some(cfg) => cfg,
        None => {
            //error handling
            VideoConfig::from_resolution("720p").unwrap()
        }
    };

    // 1. Add Bitrate
    vid_options.push("-b:v".to_string());
    vid_options.push(vidconfigval.bitrate);

    // 2. Add Maxrate
    vid_options.push("-maxrate".to_string());
    vid_options.push(vidconfigval.maxrate);

    // 3. Add Buffer Size
    vid_options.push("-bufsize".to_string());
    vid_options.push(vidconfigval.bufsize);

    // 4. Add Scale Filter
    vid_options.push("-vf".to_string());
    vid_options.push(vidconfigval.scale_filter);

    //5  Add the content
    vid_options.push("-hls_time".to_string());
    vid_options.push(content_length.to_string());

    //create the command
    let mut video_chunker = Command::new("ffmpeg");

    video_chunker
        .arg("-i")
        .arg(video_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .args(vid_options);

    // .arg(output_options)
    // .arg(vidoutput)
    // .stdout(Stdio::piped())
    // .stderr(Stdio::piped())
    // .output()
    // .expect("failed to run ffmped");

    let cmd = video_chunker
        .arg(final_destination)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run ffmpeg");

    println!("cmd is called ");

    //check the output and the erorr
    if cmd.status.success() {
        println!("ffmpeg output is : {:?}", cmd.stdout);
        return;
    } else {
        eprint!(" ffmpeg faild with status : {}", cmd.status);
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
