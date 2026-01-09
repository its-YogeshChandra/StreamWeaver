use crate::utils::errorhandler;
use std::net::TcpStream;

pub fn format_handler(output: &str, stream: &TcpStream) -> Result<Vec<String>, ()> {
    println!("printing output : {}", output);
    //making the hashMap variable
    let mut formats: Vec<String> = Vec::new();

    //fetch the data
    if output.contains("1920x1080") {
        formats.push("1080p".to_string());
    }
    if output.contains("1280x720") {
        formats.push("720p".to_string());
    }
    if output.contains("854x480") {
        formats.push("480p".to_string());
    }
    if output.contains("640x360") {
        formats.push("360p".to_string());
    }
    if output.contains("426x240") {
        formats.push("240p".to_string());
    }
    if output.contains("256x144") {
        formats.push("144p".to_string());
    }

    //return the vector
    println!("formats: {:?}", formats);
    Ok(formats)
}
