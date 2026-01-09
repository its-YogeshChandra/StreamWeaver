//extract the bitrate codes

pub fn code_handler(output: &str, bitrate: &str, vcodec: &str) -> (String, String) {
    println!("printing output : {}", output);

    let mut vidcode: String = String::new();
    let mut audcode: String = String::new();
    //loop the output line by line
    for line in output.lines() {
        //check for different formats
        if line.contains(bitrate) && line.contains("mp4") && vcodec.contains(vcodec) {
            //split the line and extact the code out of it
            let parts: Vec<&str> = line.split_whitespace().collect();
            let code = parts.get(0).clone().expect("not a string value");
            vidcode = code.to_string();
        }

        //check the the audio format
        if line.contains("audio only")
            && line.contains("English")
            && (line.contains("opus") || line.contains("map4a.40.5"))
        {
            //split the line and get the
            let parts: Vec<&str> = line.split_whitespace().collect();
            let code = parts.get(0).clone().expect("not a string value");
            audcode = code.to_string();
        }
    }

    (vidcode, audcode)
}
