# StreamWeaver 🎬

A powerful Rust-based video streaming application that converts videos into HLS (HTTP Live Streaming) format with customizable chunk sizes and quality settings. StreamWeaver downloads videos from various sources (using yt-dlp), processes them with FFmpeg, and generates adaptive bitrate streaming segments for optimal playback across different network conditions.

## 🌟 Features

- **Video Download & Processing**: Automatically downloads videos from supported platforms using yt-dlp
- **HLS Streaming**: Converts videos into HLS format with `.m3u8` playlist and `.ts` segment files
- **Adaptive Bitrate Streaming**: Supports multiple quality levels (144p to 1080p)
- **Customizable Chunk Duration**: User-defined segment length based on content requirements
- **Format Detection**: Automatically detects available video formats and resolutions
- **Custom HTTP Server**: Built-from-scratch TCP-based HTTP server in Rust (no frameworks)
- **RESTful API**: Clean API endpoints for metadata retrieval and video processing

## 🏗️ Architecture

### Project Structure

```
StreamWeaver/
├── backend/
│   ├── src/
│   │   ├── main.rs              # HTTP server implementation
│   │   ├── controller/          # Request handlers
│   │   │   ├── extractor.rs     # Video download & HLS conversion
│   │   │   ├── metadata.rs      # Format detection & metadata
│   │   │   └── sendresponse.rs  # Response utilities
│   │   ├── routes/              # Route definitions
│   │   │   └── routes.rs        # Route moderator & mapping
│   │   ├── services/            # Business logic
│   │   │   ├── codeextractor.rs # Format code extraction
│   │   │   └── formathandler.rs # Resolution detection
│   │   └── utils/               # Utilities
│   │       ├── requesthandler.rs
│   │       ├── responsesystem.rs
│   │       ├── errorhandler.rs
│   │       └── jsondeserializer.rs
│   ├── public/                  # Downloaded videos
│   ├── vidoutput/               # HLS output (m3u8 + ts files)
│   └── Cargo.toml
└── frontend/                    # (To be implemented)
```

### Technology Stack

- **Language**: Rust (Edition 2024)
- **Video Processing**: FFmpeg
- **Video Download**: yt-dlp
- **Async Runtime**: Tokio
- **HTTP Client**: reqwest
- **Serialization**: serde, serde_json
- **Environment**: dotenv

## 📡 API Endpoints

### 1. Get Video Metadata
**Endpoint**: `POST /metadata`

Retrieves available video formats and resolutions for a given URL.

**Request Body**:
```json
{
  "data": {
    "url": "https://example.com/video"
  }
}
```

**Response**:
```json
{
  "success": true,
  "message": "format successfully received",
  "data": ["1080p", "720p", "480p", "360p"]
}
```

### 2. Process Video to HLS
**Endpoint**: `POST /extractor`

Downloads video and converts it to HLS format with specified quality and chunk duration.

**Request Body**:
```json
{
  "data": {
    "url": "https://example.com/video",
    "bitrate": "720p",
    "vcodec": "avc1.64002a",
    "content-length": "10"
  }
}
```

**Parameters**:
- `url`: Video URL (must start with "http")
- `bitrate`: Resolution (`1080p`, `720p`, `480p`, `360p`, `240p`, `144p`)
- `vcodec`: Video codec (`avc1.64002a` or `av01.0.09M.08`)
- `content-length`: Chunk duration in seconds (as string number)

**Output**: Generates HLS files in `backend/vidoutput/`:
- `index.m3u8` - Master playlist
- `index0.ts`, `index1.ts`, ... - Video segments

### 3. Test Endpoint
**Endpoint**: `POST /create`

Simple test endpoint for server verification.

**Request Body**:
```json
{
  "data": {
    "name": "adam"
  }
}
```

## 🚀 Installation

### Prerequisites

1. **Rust** (Edition 2024 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **FFmpeg** - For video processing
   ```bash
   # macOS
   brew install ffmpeg
   
   # Ubuntu/Debian
   sudo apt install ffmpeg
   
   # Windows
   # Download from https://ffmpeg.org/download.html
   ```

3. **yt-dlp** - For video downloading
   ```bash
   # macOS
   brew install yt-dlp
   
   # Ubuntu/Debian
   sudo apt install yt-dlp
   
   # Or via pip
   pip install yt-dlp
   ```

### Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd StreamWeaver/backend
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Run the server**:
   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:8080`

## 💻 Usage

### Example: Convert a Video to HLS

1. **Get available formats**:
   ```bash
   curl -X POST http://127.0.0.1:8080/metadata \
     -H "Content-Type: application/json" \
     -d '{"data": {"url": "https://www.youtube.com/watch?v=example"}}'
   ```

2. **Process video to HLS**:
   ```bash
   curl -X POST http://127.0.0.1:8080/extractor \
     -H "Content-Type: application/json" \
     -d '{
       "data": {
         "url": "https://www.youtube.com/watch?v=example",
         "bitrate": "720p",
         "vcodec": "avc1.64002a",
         "content-length": "10"
       }
     }'
   ```

3. **Access HLS stream**:
   The output will be in `backend/vidoutput/`:
   - `index.m3u8` - Use this file in your video player
   - `index0.ts`, `index1.ts`, etc. - Video segments

### Playing HLS Stream

You can use any HLS-compatible player:

**VLC**:
```bash
vlc backend/vidoutput/index.m3u8
```

**FFplay**:
```bash
ffplay backend/vidoutput/index.m3u8
```

**Web Browser** (with hls.js):
```html
<video id="video"></video>
<script src="https://cdn.jsdelivr.net/npm/hls.js@latest"></script>
<script>
  var video = document.getElementById('video');
  var hls = new Hls();
  hls.loadSource('http://localhost:8080/vidoutput/index.m3u8');
  hls.attachMedia(video);
</script>
```

## ⚙️ Configuration

### Video Quality Presets

The application uses optimized encoding settings for each resolution:

| Resolution | Bitrate | Max Rate | Buffer Size | Scale Filter |
|-----------|---------|----------|-------------|--------------|
| 1080p     | 8000k   | 8560k    | 16000k      | scale=-2:1080 |
| 720p      | 2500k   | 2675k    | 5000k       | scale=-2:720  |
| 480p      | 1400k   | 1498k    | 2800k       | scale=-2:480  |
| 360p      | 800k    | 856k     | 1600k       | scale=-2:360  |

### FFmpeg Encoding Parameters

- **Video Codec**: libx264
- **Audio Codec**: AAC
- **Preset**: medium
- **CRF**: 24
- **Audio Bitrate**: 128k
- **Audio Sample Rate**: 44100 Hz
- **HLS Playlist Type**: VOD (Video on Demand)

## 🔧 How It Works

1. **Request Reception**: Custom TCP server receives HTTP requests
2. **Request Parsing**: Extracts method, route, headers, and body from raw TCP stream
3. **Route Matching**: Routes request to appropriate controller
4. **Format Detection** (`/metadata`):
   - Calls yt-dlp to list available formats
   - Parses output to detect supported resolutions
   - Returns available quality options
5. **Video Processing** (`/extractor`):
   - Validates request parameters (bitrate, codec, URL)
   - Downloads video using yt-dlp with specified format codes
   - Converts to HLS using FFmpeg with adaptive bitrate settings
   - Generates `.m3u8` playlist and `.ts` segment files
   - Segments are created based on user-specified chunk duration

## 🛠️ Development

### Building from Source

```bash
cd backend
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Structure

- **main.rs**: Implements raw TCP server, request parsing, and connection handling
- **routes/**: Route definitions and request routing logic
- **controller/**: Business logic for each endpoint
- **services/**: Reusable services (format detection, code extraction)
- **utils/**: Helper functions (error handling, JSON parsing, response formatting)

## 📝 API Response Format

All API responses follow this structure:

```json
{
  "success": boolean,
  "message": string,
  "data": any
}
```

**Error Response**:
```json
{
  "success": false,
  "message": "error description"
}
```

## 🎯 Supported Video Codecs

- `avc1.64002a` - H.264 (AVC)
- `av01.0.09M.08` - AV1

## 📊 HLS Output Format

The generated HLS stream follows the standard format:

**index.m3u8** (Master Playlist):
```
#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:13
#EXT-X-MEDIA-SEQUENCE:0
#EXT-X-PLAYLIST-TYPE:VOD
#EXTINF:12.178833,
index0.ts
#EXTINF:11.911900,
index1.ts
...
#EXT-X-ENDLIST
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🔮 Future Enhancements

- [ ] Frontend web interface for easy video conversion
- [ ] Multi-bitrate adaptive streaming (multiple quality variants)
- [ ] Live streaming support
- [ ] Video thumbnail generation
- [ ] Progress tracking for long video conversions
- [ ] Database integration for job management
- [ ] Authentication and user management
- [ ] CDN integration for scalable delivery
- [ ] Support for more video codecs (VP9, HEVC)
- [ ] Subtitle/caption support

## 📞 Support

For issues, questions, or contributions, please open an issue on the GitHub repository.

---

**Built with ❤️ using Rust**
