"use client"
import * as React from "react"
import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { UrlInput } from "./UrlInput"
import { ResolutionSelector } from "./ResolutionSelector"
import { ProcessingStatus } from "./ProcessingStatus"
import { HlsPlayer } from "../player/HlsPlayer"
import { RefreshCw, Download } from "lucide-react"
import apiService from "@/services/apiservices"

type ToolState = "INPUT" | "METADATA_SELECT" | "PROCESSING" | "COMPLETED" | "ERROR"

interface Format {
  bitrate: string;
  content_length: string;
  vcodec: string;
  resolution?: string;
}

export function ConverterTool() {
  const [state, setState] = useState<ToolState>("INPUT")
  const [url, setUrl] = useState("")
  const [metadata, setMetadata] = useState<Format[]>([])
  const [selectedQuality, setSelectedQuality] = useState<Format | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [streamUrl, setStreamUrl] = useState<string>("")
  const [isLoading, setIsLoading] = useState(false)

  // Handler for URL submission - fetches available formats
  const handleUrlSubmit = async (videoUrl: string) => {
    setIsLoading(true)
    setError(null)
    setUrl(videoUrl)

    try {
      const response = await apiService.getFormatService(videoUrl)

      if (response && response.data) {
        // Assuming the API returns an array of formats
        setMetadata(response.data.formats || response.data)
        setState("METADATA_SELECT")
      } else {
        throw new Error("No formats received from server")
      }
    } catch (err: any) {
      setError(err.message || "Failed to fetch video formats. Please check the URL and try again.")
      setState("INPUT")
    } finally {
      setIsLoading(false)
    }
  }

  // Handler for processing the video with selected quality
  const handleProcess = async () => {
    if (!selectedQuality) return

    setIsLoading(true)
    setError(null)
    setState("PROCESSING")

    try {
      const response = await apiService.getStreamService(
        url,
        selectedQuality.bitrate,
        selectedQuality.content_length,
        selectedQuality.vcodec
      )

      if (response && response.data) {
        // Assuming the API returns the stream URL or path
        const streamPath = response.data.stream_url || response.data.path || "/vidoutput/index.m3u8"
        setStreamUrl(streamPath)

        // Simulate processing delay for better UX
        setTimeout(() => {
          setState("COMPLETED")
        }, 2000)
      } else {
        throw new Error("Failed to start video processing")
      }
    } catch (err: any) {
      setError(err.message || "Failed to process video. Please try again.")
      setState("METADATA_SELECT")
    } finally {
      setIsLoading(false)
    }
  }

  // Reset handler to start over
  const reset = () => {
    setState("INPUT")
    setUrl("")
    setMetadata([])
    setSelectedQuality(null)
    setError(null)
    setStreamUrl("")
    setIsLoading(false)
  }




  return (
    <Card className="w-full max-w-3xl mx-auto overflow-hidden border-0 shadow-2xl bg-white/90 backdrop-blur-xl ring-1 ring-gray-200/50">
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

          {state === "METADATA_SELECT" && (
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

              <ResolutionSelector
                metadata={metadata}
                selected={selectedQuality}
                onSelect={setSelectedQuality}
              />

              <div className="flex justify-end pt-4">
                <Button
                  size="lg"
                  variant="happy"
                  disabled={!selectedQuality}
                  onClick={handleProcess}
                  className="w-full md:w-auto min-w-[200px]"
                >
                  Convert Video
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
              <div className="flex items-center justify-between pb-4">
                <h3 className="text-xl font-bold text-green-600 flex items-center gap-2">
                  <span className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                  Stream Ready
                </h3>
                <div className="flex gap-2">
                  <Button variant="outline" size="sm" onClick={() => window.open(streamUrl, '_blank')}>
                    <Download className="w-4 h-4 mr-2" /> .m3u8
                  </Button>
                  <Button variant="ghost" size="sm" onClick={reset}>
                    <RefreshCw className="w-4 h-4 mr-2" /> New Video
                  </Button>
                </div>
              </div>

              <HlsPlayer src={streamUrl} />

              <div className="p-4 bg-gray-50 rounded-xl border border-gray-100 text-sm font-mono text-gray-600 break-all">
                {typeof window !== 'undefined' ? `${window.location.protocol}//${window.location.host}${streamUrl}` : streamUrl}
              </div>
            </motion.div>
          )}

        </AnimatePresence>
      </CardContent>
    </Card>
  )
}
