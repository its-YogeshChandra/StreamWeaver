import * as React from "react"
import { motion, AnimatePresence } from "framer-motion"
import { ArrowRight, Link as LinkIcon } from "lucide-react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"

interface UrlInputProps {
    onSubmit: (url: string) => void
    isLoading?: boolean
}

export function UrlInput({ onSubmit, isLoading }: UrlInputProps) {
    const [url, setUrl] = React.useState("")

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()
        if (url && !isLoading) {
            onSubmit(url)
        }
    }

    return (
        <form onSubmit={handleSubmit} className="w-full max-w-2xl mx-auto space-y-4">
            <div className="relative group">
                <div className="absolute inset-y-0 left-4 flex items-center pointer-events-none text-muted-foreground group-focus-within:text-happy-blue transition-colors">
                    <LinkIcon className="w-5 h-5" />
                </div>
                <Input
                    placeholder="Paste YouTube or video URL here..."
                    value={url}
                    onChange={(e) => setUrl(e.target.value)}
                    className="pl-12 h-16 text-lg shadow-sm border-gray-200 focus-visible:ring-happy-blue/50"
                    autoFocus
                />
                <AnimatePresence>
                    {url && (
                        <motion.div
                            initial={{ opacity: 0, scale: 0.8 }}
                            animate={{ opacity: 1, scale: 1 }}
                            exit={{ opacity: 0, scale: 0.8 }}
                            className="absolute inset-y-2 right-2"
                        >
                            <Button
                                size="lg"
                                variant="happy"
                                type="submit"
                                disabled={isLoading}
                                className="h-12 px-6 rounded-lg text-base font-semibold"
                            >
                                {isLoading ? (
                                    <span className="flex items-center gap-2">
                                        <span className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                                        Checking...
                                    </span>
                                ) : (
                                    <span className="flex items-center gap-2">
                                        Start <ArrowRight className="w-4 h-4" />
                                    </span>
                                )}
                            </Button>
                        </motion.div>
                    )}
                </AnimatePresence>
            </div>
            <p className="text-center text-sm text-muted-foreground">
                Supported sources: YouTube, direct video links
            </p>
        </form>
    )
}
