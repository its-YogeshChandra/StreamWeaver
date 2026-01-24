use std::collections::HashSet;
use std::net::TcpStream;

pub fn format_handler(output: &str, _stream: &TcpStream) -> Result<Vec<String>, ()> {
    // Use HashSet to prevent duplicates
    let mut formats_set: HashSet<String> = HashSet::new();

    // Only check for exact resolution dimensions that yt-dlp outputs
    // Format: "WIDTHxHEIGHT" - these are the actual dimensions in yt-dlp output
    let resolution_checks: Vec<(&[&str], &str)> = vec![
        // 8K resolutions
        (&["7680x4320", "4320x7680"], "8K"),
        // 4K resolutions  
        (&["3840x2160", "2160x3840"], "4K"),
        // 1440p (2K) resolutions
        (&["2560x1440", "1440x2560"], "1440p"),
        // 1080p resolutions
        (&["1920x1080", "1080x1920"], "1080p"),
        // 720p resolutions
        (&["1280x720", "720x1280"], "720p"),
        // 480p resolutions
        (&["854x480", "480x854", "640x480", "480x640"], "480p"),
        // 360p resolutions
        (&["640x360", "360x640"], "360p"),
        // 240p resolutions
        (&["426x240", "240x426", "320x240", "240x320"], "240p"),
        // 144p resolutions
        (&["256x144", "144x256"], "144p"),
    ];

    // Check output for each resolution pattern - case sensitive for dimensions
    for (patterns, format_name) in resolution_checks {
        for pattern in patterns.iter() {
            // Check for exact dimension match (e.g., "1920x1080" as a whole word)
            if output.contains(pattern) {
                formats_set.insert(format_name.to_string());
                break; // Found this resolution, move to next format
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

    Ok(formats)
}


