use std::collections::HashSet;
use std::net::TcpStream;

pub fn format_handler(output: &str, _stream: &TcpStream) -> Result<Vec<String>, ()> {
    println!("printing output : {}", output);
    
    // Use HashSet to prevent duplicates
    let mut formats_set: HashSet<String> = HashSet::new();

    // Check for various resolution patterns (both landscape and portrait)
    // Each tuple: (patterns to check, format name)
    let resolution_checks: Vec<(&[&str], &str)> = vec![
        (&["7680x4320", "4320x7680", "4320p", "8k"], "8K"),
        (&["3840x2160", "2160x3840", "2160p", "4k"], "4K"),
        (&["2560x1440", "1440x2560", "1440p", "2k"], "1440p"),
        (&["1920x1080", "1080x1920", "1080p"], "1080p"),
        (&["1280x720", "720x1280", "720p"], "720p"),
        (&["854x480", "480x854", "480p"], "480p"),
        (&["640x360", "360x640", "360p"], "360p"),
        (&["426x240", "240x426", "240p"], "240p"),
        (&["256x144", "144x256", "144p"], "144p"),
    ];

    // Check output for each resolution pattern
    let output_lower = output.to_lowercase();
    for (patterns, format_name) in resolution_checks {
        for pattern in patterns.iter() {
            if output_lower.contains(&pattern.to_lowercase()) {
                formats_set.insert(format_name.to_string());
                break; // Found this resolution, move to next
            }
        }
    }

    // Convert to Vec and sort by quality (highest first)
    let mut formats: Vec<String> = formats_set.into_iter().collect();
    
    // Custom sort order (highest quality first)
    let order = ["8K", "4K", "1440p", "1080p", "720p", "480p", "360p", "240p", "144p"];
    formats.sort_by(|a, b| {
        let pos_a = order.iter().position(|x| x == a).unwrap_or(999);
        let pos_b = order.iter().position(|x| x == b).unwrap_or(999);
        pos_a.cmp(&pos_b)
    });

    println!("formats: {:?}", formats);
    Ok(formats)
}

