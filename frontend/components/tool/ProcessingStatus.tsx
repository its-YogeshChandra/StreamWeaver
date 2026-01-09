import { motion } from "framer-motion"
import { Loader2 } from "lucide-react"

export function ProcessingStatus() {
    const steps = [
        "Analyzing video metadata...",
        "Extracting video streams...",
        "Converting to HLS format...",
        "Generating playlist segments...",
    ]

    return (
        <div className="flex flex-col items-center justify-center py-12 space-y-6">
            <div className="relative">
                <motion.div
                    animate={{ rotate: 360 }}
                    transition={{ duration: 2, repeat: Infinity, ease: "linear" }}
                    className="relative z-10"
                >
                    <div className="w-24 h-24 rounded-full border-4 border-gray-100 border-t-happy-blue" />
                </motion.div>
                <div className="absolute inset-0 flex items-center justify-center">
                    <Loader2 className="w-8 h-8 text-happy-blue animate-pulse" />
                </div>
            </div>

            <div className="text-center space-y-2">
                <h3 className="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-happy-blue to-happy-purple">
                    Processing Video
                </h3>
                <p className="text-muted-foreground w-64 mx-auto">
                    This might take a minute depending on the video length.
                </p>
            </div>

            <div className="w-full max-w-sm bg-gray-100 rounded-full h-2 overflow-hidden">
                <motion.div
                    initial={{ width: "0%" }}
                    animate={{ width: "100%" }}
                    transition={{ duration: 30, ease: "linear" }}
                    className="h-full bg-happy-blue rounded-full"
                />
            </div>
        </div>
    )
}
