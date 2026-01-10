import * as React from "react"
import { motion, AnimatePresence } from "framer-motion"
import { ChevronDown, Check } from "lucide-react"
import { cn } from "@/lib/utils"

export interface SelectOption {
    value: string
    label: string
    description?: string
}

interface SelectProps {
    options: SelectOption[]
    value: string | null
    onChange: (value: string) => void
    placeholder?: string
    label?: string
    className?: string
}

export function Select({ options, value, onChange, placeholder = "Select an option", label, className }: SelectProps) {
    const [isOpen, setIsOpen] = React.useState(false)
    const containerRef = React.useRef<HTMLDivElement>(null)

    const selectedOption = options.find((opt) => opt.value === value)

    React.useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
                setIsOpen(false)
            }
        }

        document.addEventListener("mousedown", handleClickOutside)
        return () => document.removeEventListener("mousedown", handleClickOutside)
    }, [])

    return (
        <div className={cn("relative space-y-2", className)} ref={containerRef}>
            {label && <label className="text-sm font-medium text-gray-700">{label}</label>}
            <button
                onClick={() => setIsOpen(!isOpen)}
                className={cn(
                    "w-full flex items-center justify-between p-3 rounded-xl border-2 transition-all duration-200 bg-white text-left",
                    isOpen ? "border-happy-blue ring-4 ring-happy-blue/10" : "border-gray-100 hover:border-happy-blue/30",
                    !selectedOption ? "text-gray-400" : "text-gray-900"
                )}
            >
                <span className="truncate font-medium">
                    {selectedOption ? selectedOption.label : placeholder}
                </span>
                <ChevronDown
                    className={cn(
                        "w-5 h-5 text-gray-400 transition-transform duration-200",
                        isOpen && "transform rotate-180 text-happy-blue"
                    )}
                />
            </button>

            <AnimatePresence>
                {isOpen && (
                    <motion.div
                        initial={{ opacity: 0, y: -10, scale: 0.95 }}
                        animate={{ opacity: 1, y: 0, scale: 1 }}
                        exit={{ opacity: 0, y: -10, scale: 0.95 }}
                        transition={{ duration: 0.2 }}
                        className="absolute z-50 w-full mt-2 bg-white rounded-xl shadow-xl border border-gray-100 overflow-hidden"
                    >
                        <div className="max-h-60 overflow-auto py-1 custom-scrollbar">
                            {options.map((option) => {
                                const isSelected = option.value === value
                                return (
                                    <button
                                        key={option.value}
                                        onClick={() => {
                                            onChange(option.value)
                                            setIsOpen(false)
                                        }}
                                        className={cn(
                                            "w-full flex items-center justify-between px-4 py-3 text-left transition-colors",
                                            isSelected ? "bg-happy-blue/5 text-happy-blue" : "hover:bg-gray-50 text-gray-700"
                                        )}
                                    >
                                        <div>
                                            <div className="font-medium">{option.label}</div>
                                            {option.description && (
                                                <div className="text-xs text-muted-foreground mt-0.5">{option.description}</div>
                                            )}
                                        </div>
                                        {isSelected && <Check className="w-4 h-4 text-happy-blue" />}
                                    </button>
                                )
                            })}
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    )
}
