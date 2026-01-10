"use client"
import * as React from "react"
import { motion } from "framer-motion"
import { Check, Settings2, Tv } from "lucide-react"
import { cn } from "@/lib/utils"

interface ResolutionSelectorProps {
    resolutions: string[]
    selected: string | null
    onSelect: (resolution: string) => void
}

// Map resolutions to quality labels and icons
const getQualityInfo = (resolution: string) => {
    const res = resolution.toLowerCase()
    if (res.includes("1080") || res.includes("hd")) {
        return { label: "Full HD", tier: "premium" }
    } else if (res.includes("720")) {
        return { label: "HD", tier: "high" }
    } else if (res.includes("480")) {
        return { label: "SD", tier: "medium" }
    } else if (res.includes("360")) {
        return { label: "Low", tier: "low" }
    } else if (res.includes("240") || res.includes("144")) {
        return { label: "Mobile", tier: "low" }
    }
    return { label: "Standard", tier: "medium" }
}

export function ResolutionSelector({ resolutions, selected, onSelect }: ResolutionSelectorProps) {
    return (
        <div className="w-full space-y-4">
            <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold flex items-center gap-2">
                    <Settings2 className="w-5 h-5 text-happy-coral" />
                    Select Quality
                </h3>
                <span className="text-sm text-muted-foreground">
                    {resolutions.length} formats available
                </span>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
                {resolutions.map((resolution, index) => {
                    const isSelected = selected === resolution
                    const { label, tier } = getQualityInfo(resolution)

                    return (
                        <motion.button
                            key={`${resolution}-${index}`}
                            whileHover={{ scale: 1.02 }}
                            whileTap={{ scale: 0.98 }}
                            onClick={() => onSelect(resolution)}
                            className={cn(
                                "relative flex flex-col items-center justify-center p-5 rounded-2xl border-2 transition-all duration-200",
                                isSelected
                                    ? "border-happy-blue bg-happy-blue/5 shadow-lg"
                                    : "border-gray-100 hover:border-happy-blue/30 bg-white hover:shadow-md"
                            )}
                        >
                            <Tv className={cn(
                                "w-8 h-8 mb-2",
                                isSelected ? "text-happy-blue" : "text-gray-400"
                            )} />

                            <span className={cn(
                                "font-bold text-xl",
                                isSelected ? "text-happy-blue" : "text-gray-700"
                            )}>
                                {resolution}
                            </span>

                            <span className={cn(
                                "text-xs mt-1",
                                tier === "premium" ? "text-happy-coral font-medium" :
                                    tier === "high" ? "text-happy-green" :
                                        "text-gray-400"
                            )}>
                                {label}
                            </span>

                            {isSelected && (
                                <motion.div
                                    initial={{ scale: 0 }}
                                    animate={{ scale: 1 }}
                                    className="absolute top-2 right-2 w-6 h-6 rounded-full bg-happy-blue text-white flex items-center justify-center"
                                >
                                    <Check className="w-4 h-4" />
                                </motion.div>
                            )}
                        </motion.button>
                    )
                })}
            </div>
        </div>
    )
}
