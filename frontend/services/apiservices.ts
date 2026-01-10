import axios from "axios";

// API service for StreamWeaver backend
const formatUrl = process.env.NEXT_PUBLIC_FORMAT_URL;
const streamUrl = process.env.NEXT_PUBLIC_STREAM_URL;

export interface StreamPayload {
  url: String
  bitrate: String // This maps to resolution like "1080p"
  content_length: String
  vcodec: String
}

export class ApiClass {

  // Fetch available video formats from the backend
  // Returns: { success: true, message: "...", data: ["1080p", "720p", "480p", ...] }
  async getFormatService(url: String) {
    try {
      if (!formatUrl) {
        throw new Error("Format URL is not configured in environment variables")
      }

      const payload = { url: url };
      console.log("Calling format API:", formatUrl, "with payload:", payload);

      const formats = await axios.post(formatUrl, payload, {
        headers: {
          "Content-Type": "application/json"
        }
      })

      console.log("Format API response:", formats.data);
      return formats

    } catch (error) {
      console.error("Error in getFormatService:", error);
      throw error;
    }
  }

  // Initiate video streaming with full configuration
  async getStreamService(payload: StreamPayload) {
    try {
      if (!streamUrl) {
        throw new Error("Stream URL is not configured in environment variables")
      }

      console.log("Calling stream API:", streamUrl, "with payload:", payload);

      const value = await axios.post(streamUrl, payload, {
        headers: {
          "Content-Type": "application/json"
        },
        responseType: 'blob' // Important for binary files (TAR, ZIP, etc.)
      })

      console.log("Stream API response:", value);
      return value;

    } catch (error) {
      console.error("Error in getStreamService:", error);
      throw error;
    }
  }
}

const apiService = new ApiClass();

export default apiService;
