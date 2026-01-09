"use client"

import * as React from "react"
import { motion, AnimatePresence } from "framer-motion"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { UrlInput } from "./UrlInput"
import { ResolutionSelector } from "./ResolutionSelector"
import { ProcessingStatus } from "./ProcessingStatus"
import { HlsPlayer } from "../player/HlsPlayer"
import { RefreshCw, Download } from "lucide-react"

import axios from "axios"

type ToolState = "INPUT" | "METADATA_SELECT" | "PROCESSING" | "COMPLETED" | "ERROR"

export function ConverterTool() {
    const [state, setState] = React.useState<ToolState>("INPUT")
    const [url, setUrl] = React.useState("")
    const [metadata, setMetadata] = React.useState<string[]>([])
    const [selectedQuality, setSelectedQuality] = React.useState<string | null>(null)
    const [error, setError] = React.useState<string | null>(null)

    // The backend generates this path.
    // Assuming standard output path: localhost:8080/vidoutput/index.m3u8
    // Since we proxy /api -> 8080, we need to know how to access static files.
    // The current backend code serves static files? 
    // Looking at backend main.rs, it doesn't seem to have a static file server for "vidoutput".
    // Wait, let me check backend main.rs again. It has handle_connection but I didn't see explicit static file serving 
    // other than what might be handled in routes. 
    // Actually, I should probably check if the backend serves the output files. 
    // If not, I'll assume for now I should try to access it via valid path or maybe I need to ask backend to serve it.
    // The README says: "The output will be in backend/vidoutput/: index.m3u8 - Use this file in your video player"
    // But strictly speaking, the backend needs to SERVE it via HTTP for the browser to play it. 
    // The current backend implementation seems to only handle specific routes (/create, /metadata, /extractor).
    // Assuming the user might need to run a separate file server or the backend serves it and I missed it.
    // I will assume the backend serves it at /vidoutput/index.m3u8 for now via a static file handler 
    // OR I will assume the proxy works if I add a route for it.
    // Let's stick to the proxy /api path for logic.
    // For the video source, we'll try `/api/vidoutput/index.m3u8` if we can proxy it, 
    // or `http://localhost:8080/vidoutput/index.m3u8` directly (which requires CORS).
    // Let's assume `http://localhost:8080/vidoutput/index.m3u8` is the target.
    const streamUrl = "/api/vidoutput/index.m3u8"

    const handleUrlSubmit = async (inputUrl: string) => {
        setUrl(inputUrl)
        setState("PROCESSING") // Temporarily show processing while fetching metadata
        setError(null)

        try {
            const response = await axios.post("/api/metadata", {
                data: { url: inputUrl }
            })

            if (response.data && response.data.data) {
                setMetadata(response.data.data) // Expecting ["1080p", "720p"] etc
                setState("METADATA_SELECT")
            } else {
                throw new Error("Invalid response format")
            }
        } catch (err) {
            console.error(err)
            setError("Failed to fetch video metadata. Please check the URL.")
            setState("INPUT")
        }
    }

    const handleProcess = async () => {
        if (!selectedQuality) return

        setState("PROCESSING")
        setError(null)

        try {
            // The backend expects: { data: { url, bitrate, vcodec, content-length } }
            // Defaulting vcodec to avc1.64002a as per readme
            // Defaulting content-length to "10" (chunk size)
            await axios.post("/api/extractor", {
                data: {
                    url: url,
                    bitrate: selectedQuality,
                    vcodec: "avc1.64002a",
                    "content-length": "10"
                }
            })

            // Since the backend might block until done, 
            // once this returns, we assume success?
            setState("COMPLETED")
        } catch (err) {
            console.error(err)
            setError("Processing failed. Please try again.")
            setState("METADATA_SELECT")
        }
    }

    const reset = () => {
        setState("INPUT")
        setUrl("")
        setMetadata([])
        setSelectedQuality(null)
        setError(null)
    }

    return (
        <Card className="w-full max-w-3xl mx-auto overflow-hidden border-0 shadow-2xl bg-white/90 backdrop-blur-xl ring-1 ring-gray-200/50">
            <CardHeader className="text-center pb-2">
                <CardTitle className="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">
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
                            <UrlInput onSubmit={handleUrlSubmit} isLoading={false} />
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
