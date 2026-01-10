import * as React from "react"
import { motion } from "framer-motion"
import { Check, Settings2 } from "lucide-react"
import { cn } from "@/lib/utils"

interface Format {
    bitrate: string;
    content_length: string;
    vcodec: string;
    resolution?: string;
}

interface ResolutionSelectorProps {
    metadata: Format[]
    selected: Format | null
    onSelect: (format: Format) => void
}

export function ResolutionSelector({ metadata, selected, onSelect }: ResolutionSelectorProps) {
    // Sort by bitrate (higher first)
    const sorted = [...metadata].sort((a, b) => {
        const bitrateA = parseInt(a.bitrate) || 0
        const bitrateB = parseInt(b.bitrate) || 0
        return bitrateB - bitrateA
    })

    // Helper to format file size
    const formatSize = (bytes: string) => {
        const size = parseInt(bytes)
        if (size >= 1024 * 1024 * 1024) {
            return `${(size / (1024 * 1024 * 1024)).toFixed(1)} GB`
        } else if (size >= 1024 * 1024) {
            return `${(size / (1024 * 1024)).toFixed(1)} MB`
        } else if (size >= 1024) {
            return `${(size / 1024).toFixed(1)} KB`
        }
        return `${size} B`
    }

    // Helper to format bitrate
    const formatBitrate = (bitrate: string) => {
        const rate = parseInt(bitrate)
        if (rate >= 1000) {
            return `${(rate / 1000).toFixed(1)} Mbps`
        }
        return `${rate} Kbps`
    }

    return (
        <div className="w-full space-y-4">
            <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold flex items-center gap-2">
                    <Settings2 className="w-5 h-5 text-happy-coral" />
                    Select Quality
                </h3>
                <span className="text-sm text-muted-foreground">
                    {metadata.length} formats found
                </span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                {sorted.map((format, index) => {
                    const isSelected = selected?.bitrate === format.bitrate && selected?.vcodec === format.vcodec
                    return (
                        <motion.button
                            key={`${format.bitrate}-${format.vcodec}-${index}`}
                            whileHover={{ scale: 1.02 }}
                            whileTap={{ scale: 0.98 }}
                            onClick={() => onSelect(format)}
                            className={cn(
                                "relative flex items-center justify-between p-4 rounded-2xl border-2 transition-all duration-200",
                                isSelected
                                    ? "border-happy-blue bg-happy-blue/5 shadow-md"
                                    : "border-gray-100 hover:border-happy-blue/30 bg-white"
                            )}
                        >
                            <div className="flex flex-col items-start gap-1">
                                <span className={cn(
                                    "font-bold text-lg",
                                    isSelected ? "text-happy-blue" : "text-gray-700"
                                )}>
                                    {format.resolution || formatBitrate(format.bitrate)}
                                </span>
                                <span className="text-xs text-muted-foreground">
                                    {format.vcodec} • {formatSize(format.content_length)}
                                </span>
                                <span className="text-xs text-muted-foreground font-mono">
                                    {formatBitrate(format.bitrate)}
                                </span>
                            </div>

                            <div className={cn(
                                "w-6 h-6 rounded-full flex items-center justify-center transition-colors",
                                isSelected ? "bg-happy-blue text-white" : "bg-gray-100 text-transparent"
                            )}>
                                <Check className="w-4 h-4" />
                            </div>
                        </motion.button>
                    )
                })}
            </div>
        </div>
    )
}
