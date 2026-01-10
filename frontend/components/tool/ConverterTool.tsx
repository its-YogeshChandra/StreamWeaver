"use client"
import * as React from "react"
import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { UrlInput } from "./UrlInput"
import { StreamConfigurationCard, StreamConfig } from "./StreamConfigurationCard"
import { ProcessingStatus } from "./ProcessingStatus"
import { HlsPlayer } from "../player/HlsPlayer"
import { RefreshCw, Download } from "lucide-react"
import apiService from "@/services/apiservices"

type ToolState = "INPUT" | "CONFIG_SELECT" | "PROCESSING" | "COMPLETED" | "ERROR"

export function ConverterTool() {
  const [state, setState] = useState<ToolState>("INPUT")
  const [url, setUrl] = useState("")
  const [resolutions, setResolutions] = useState<string[]>([])

  // New state object for full configuration
  const [streamConfig, setStreamConfig] = useState<StreamConfig>({
    resolution: "",
    vcodec: "avc1.64002a",
    content_length: "10"
  })

  const [error, setError] = useState<string | null>(null)
  const [downloadBlob, setDownloadBlob] = useState<Blob | null>(null)
  const [isLoading, setIsLoading] = useState(false)

  // Handler for URL submission - fetches available formats
  const handleUrlSubmit = async (videoUrl: string) => {
    setIsLoading(true)
    setError(null)
    setUrl(videoUrl)

    try {
      const response = await apiService.getFormatService(videoUrl)

      // API returns: { success: true, message: "...", data: ["1080p", "720p", ...] }
      if (response && response.data) {
        console.log("Full format response:", response.data);

        // Handle various response shapes to be safe
        let availableResolutions: string[] = [];

        if (Array.isArray(response.data.data)) {
          availableResolutions = response.data.data;
        } else if (Array.isArray(response.data)) {
          availableResolutions = response.data;
        } else if (response.data.formats && Array.isArray(response.data.formats)) {
          availableResolutions = response.data.formats;
        }

        console.log("Parsed resolutions:", availableResolutions);

        if (availableResolutions.length > 0) {
          setResolutions(availableResolutions)

          // Auto-select the highest resolution by default
          setStreamConfig(prev => ({
            ...prev,
            resolution: availableResolutions[0]
          }))

          setState("CONFIG_SELECT")

          // Save URL to session storage as requested
          if (typeof window !== 'undefined') {
            sessionStorage.setItem("stream_weaver_url", videoUrl)
          }
        } else {
          throw new Error("No formats found in the server response")
        }
      } else {
        throw new Error(response?.data?.message || "Invalid response from server")
      }
    } catch (err: any) {
      setError(err.message || "Failed to fetch video formats. Please check the URL and try again.")
      setState("INPUT")
    } finally {
      setIsLoading(false)
    }
  }

  // Handler for processing the video with selected configuration
  const handleProcess = async () => {
    if (!streamConfig.resolution) return

    setIsLoading(true)
    setError(null)
    setState("PROCESSING")

    try {
      const response = await apiService.getStreamService({
        url: url,
        bitrate: streamConfig.resolution, // Backend maps 'bitrate' to resolution string
        content_length: streamConfig.content_length,
        vcodec: streamConfig.vcodec
      })

      if (response && response.data) {
        // Handle Blob Response (TAR File)
        const blob = response.data
        setDownloadBlob(blob)

        // Trigger Auto-Download
        const downloadUrl = window.URL.createObjectURL(blob)
        const link = document.createElement('a')
        link.href = downloadUrl
        link.setAttribute('download', 'stream_weaver_output.tar') // or .zip
        document.body.appendChild(link)
        link.click()
        link.remove()

        // Simulate processing complete
        setState("COMPLETED")
      } else {
        throw new Error("Failed to start video processing")
      }
    } catch (err: any) {
      setError(err.message || "Failed to process video. Please try again.")
      setState("CONFIG_SELECT")
    } finally {
      setIsLoading(false)
    }
  }

  // Reset handler to start over
  const reset = () => {
    setState("INPUT")
    setUrl("")
    setResolutions([])
    setStreamConfig({
      resolution: "",
      vcodec: "avc1.64002a",
      content_length: "10"
    })

    // Clear Session Storage on reset as well for cleanup
    if (typeof window !== 'undefined') {
      sessionStorage.removeItem("stream_weaver_url")
    }
    setError(null)
    setDownloadBlob(null)
    setIsLoading(false)
  }

  return (
    <Card className="w-full max-w-3xl mx-auto border-0 shadow-2xl bg-white/90 backdrop-blur-xl ring-1 ring-gray-200/50">
      <CardHeader className="text-center pb-2">
        <CardTitle className="text-3xl font-bold bg-clip-text text-transparent bg-linear-to-r from-gray-900 to-gray-600">
          HLS Converter
        </CardTitle>
        <CardDescription className="text-base">
          Transform your videos into adaptive bitrates for seamless streaming
        </CardDescription>
      </CardHeader>

      <CardContent className="p-6 md:p-10 min-h-[400px] flex flex-col justify-center">
        <AnimatePresence mode="wait">

          {state === "INPUT" && (
            <motion.div
              key="input"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              className="w-full space-y-6"
            >
              <UrlInput onSubmit={handleUrlSubmit} isLoading={isLoading} />
              {error && (
                <p className="text-red-500 text-center bg-red-50 p-3 rounded-lg text-sm">
                  {error}
                </p>
              )}
            </motion.div>
          )}

          {state === "CONFIG_SELECT" && (
            <motion.div
              key="select"
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              className="w-full space-y-8"
            >
              <div className="flex items-center justify-between">
                <Button variant="ghost" onClick={reset} className="text-muted-foreground hover:text-foreground">
                  ← Change URL
                </Button>
              </div>

              <StreamConfigurationCard
                resolutions={resolutions}
                config={streamConfig}
                onChange={setStreamConfig}
              />

              <div className="flex justify-end pt-4">
                <Button
                  size="lg"
                  variant="happy"
                  disabled={!streamConfig.resolution}
                  onClick={handleProcess}
                  className="w-full md:w-auto min-w-[200px]"
                >
                  Start Conversion
                </Button>
              </div>

              {error && (
                <p className="text-red-500 text-center bg-red-50 p-3 rounded-lg text-sm">
                  {error}
                </p>
              )}
            </motion.div>
          )}

          {state === "PROCESSING" && (
            <motion.div
              key="processing"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              <ProcessingStatus />
            </motion.div>
          )}

          {state === "COMPLETED" && (
            <motion.div
              key="completed"
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              className="space-y-6"
            >
              <div className="text-center pb-4">
                <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                  <Download className="w-8 h-8 text-green-600" />
                </div>
                <h3 className="text-2xl font-bold text-gray-900">Conversion Complete!</h3>
                <p className="text-gray-500">Your download should have started automatically.</p>
              </div>

              {/* Original Video Preview */}
              <div className="aspect-video w-full rounded-xl overflow-hidden bg-black shadow-lg">
                <iframe
                  width="100%"
                  height="100%"
                  src={url.replace("watch?v=", "embed/").replace("shorts/", "embed/")}
                  title="Original Video"
                  frameBorder="0"
                  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                  allowFullScreen
                />
              </div>

              <div className="flex flex-col gap-3 pt-4">
                {downloadBlob && (
                  <Button
                    size="lg"
                    variant="happy"
                    className="w-full"
                    onClick={() => {
                      const downloadUrl = window.URL.createObjectURL(downloadBlob)
                      const link = document.createElement('a')
                      link.href = downloadUrl
                      link.setAttribute('download', 'stream_weaver_output.tar')
                      document.body.appendChild(link)
                      link.click()
                      link.remove()
                    }}
                  >
                    <Download className="w-5 h-5 mr-2" />
                    Download TAR File Again
                  </Button>
                )}

                <Button variant="ghost" size="lg" onClick={reset} className="w-full">
                  <RefreshCw className="w-4 h-4 mr-2" /> Convert Another Video
                </Button>
              </div>
            </motion.div>
          )}

        </AnimatePresence>
      </CardContent>
    </Card>
  )
}
