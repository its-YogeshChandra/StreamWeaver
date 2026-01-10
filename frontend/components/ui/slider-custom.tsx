import * as React from "react"
import { motion, useMotionValue, useTransform } from "framer-motion"
import { cn } from "@/lib/utils"

interface SliderProps {
    value: number
    min: number
    max: number
    step?: number
    onChange: (value: number) => void
    label?: string
    formatValue?: (value: number) => string
    className?: string
}

export function Slider({ value, min, max, step = 1, onChange, label, formatValue, className }: SliderProps) {
    const trackRef = React.useRef<HTMLDivElement>(null)

    const percentage = ((value - min) / (max - min)) * 100

    const handleInteraction = (event: React.MouseEvent | React.TouchEvent | MouseEvent | TouchEvent) => {
        if (!trackRef.current) return

        const trackRect = trackRef.current.getBoundingClientRect()
        const clientX = 'touches' in event ? event.touches[0].clientX : (event as React.MouseEvent).clientX

        let newPercentage = ((clientX - trackRect.left) / trackRect.width) * 100
        newPercentage = Math.max(0, Math.min(100, newPercentage))

        const rawValue = min + (newPercentage / 100) * (max - min)
        const steppedValue = Math.round(rawValue / step) * step

        // Ensure we don't exceed bounds due to rounding
        const finalValue = Math.max(min, Math.min(max, steppedValue))

        if (finalValue !== value) {
            onChange(finalValue)
        }
    }

    const [isDragging, setIsDragging] = React.useState(false)

    React.useEffect(() => {
        const handleMouseUp = () => setIsDragging(false)
        const handleMouseMove = (e: MouseEvent) => {
            if (isDragging) handleInteraction(e)
        }

        if (isDragging) {
            window.addEventListener('mousemove', handleMouseMove)
            window.addEventListener('mouseup', handleMouseUp)
            window.addEventListener('touchmove', handleMouseMove as any)
            window.addEventListener('touchend', handleMouseUp)
        }

        return () => {
            window.removeEventListener('mousemove', handleMouseMove)
            window.removeEventListener('mouseup', handleMouseUp)
            window.removeEventListener('touchmove', handleMouseMove as any)
            window.removeEventListener('touchend', handleMouseUp)
        }
    }, [isDragging])

    return (
        <div className={cn("space-y-4", className)}>
            <div className="flex items-center justify-between">
                {label && <label className="text-sm font-medium text-gray-700">{label}</label>}
                <span className="text-sm font-bold text-happy-blue bg-happy-blue/10 px-2 py-1 rounded-md min-w-[3rem] text-center">
                    {formatValue ? formatValue(value) : value}
                </span>
            </div>

            <div
                ref={trackRef}
                onMouseDown={(e) => { setIsDragging(true); handleInteraction(e); }}
                onTouchStart={(e) => { setIsDragging(true); handleInteraction(e); }}
                className="relative h-6 flex items-center cursor-pointer group"
            >
                {/* Track Background */}
                <div className="absolute w-full h-2 bg-gray-100 rounded-full overflow-hidden">
                    <motion.div
                        className="h-full bg-happy-blue"
                        style={{ width: `${percentage}%` }}
                        initial={{ width: 0 }}
                        animate={{ width: `${percentage}%` }}
                        transition={{ type: "spring", stiffness: 300, damping: 30 }}
                    />
                </div>

                {/* Thumb */}
                <motion.div
                    className={cn(
                        "absolute w-6 h-6 bg-white border-2 border-happy-blue rounded-full shadow-md z-10 flex items-center justify-center transition-transform",
                        isDragging ? "scale-110 ring-4 ring-happy-blue/20" : "group-hover:scale-105"
                    )}
                    style={{ left: `calc(${percentage}% - 12px)` }}
                    initial={{ left: 0 }}
                    animate={{ left: `calc(${percentage}% - 12px)` }}
                    transition={{ type: "spring", stiffness: 300, damping: 30 }}
                >
                    <div className="w-2 h-2 bg-happy-blue rounded-full" />
                </motion.div>
            </div>

            <div className="flex justify-between text-xs text-gray-400 font-medium px-1">
                <span>{formatValue ? formatValue(min) : min}</span>
                <span>{formatValue ? formatValue(max) : max}</span>
            </div>
        </div>
    )
}
