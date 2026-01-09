import * as React from "react"
import { motion } from "framer-motion"
import { Check, Settings2 } from "lucide-react"
import { cn } from "@/lib/utils"

interface ResolutionSelectorProps {
    metadata: string[]
    selected: string | null
    onSelect: (res: string) => void
}

export function ResolutionSelector({ metadata, selected, onSelect }: ResolutionSelectorProps) {
    // Sort resolutions desc (1080p -> 144p)
    const sorted = [...metadata].sort((a, b) => {
        const valA = parseInt(a.replace('p', ''))
        const valB = parseInt(b.replace('p', ''))
        return valB - valA
    })

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

            <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
                {sorted.map((res) => {
                    const isSelected = selected === res
                    return (
                        <motion.button
                            key={res}
                            whileHover={{ scale: 1.02 }}
                            whileTap={{ scale: 0.98 }}
                            onClick={() => onSelect(res)}
                            className={cn(
                                "relative flex items-center justify-between p-4 rounded-2xl border-2 transition-all duration-200",
                                isSelected
                                    ? "border-happy-blue bg-happy-blue/5 shadow-md"
                                    : "border-gray-100 hover:border-happy-blue/30 bg-white"
                            )}
                        >
                            <div className="flex flex-col items-start">
                                <span className={cn(
                                    "font-bold text-lg",
                                    isSelected ? "text-happy-blue" : "text-gray-700"
                                )}>
                                    {res}
                                </span>
                                <span className="text-xs text-muted-foreground capitalize">
                                    MP4 • H.264
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
