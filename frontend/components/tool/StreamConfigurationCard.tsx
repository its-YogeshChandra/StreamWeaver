"use client"
import * as React from "react"
import { motion } from "framer-motion"
import { Settings2, Film, Timer, Layers } from "lucide-react"
import { Select, SelectOption } from "@/components/ui/select-custom"
import { Slider } from "@/components/ui/slider-custom"
import { Card, CardContent } from "@/components/ui/card"

export interface StreamConfig {
    resolution: string
    vcodec: string
    content_length: string
}

interface StreamConfigurationCardProps {
    resolutions: string[]
    config: StreamConfig
    onChange: (config: StreamConfig) => void
}

const vcodecOptions: SelectOption[] = [
    { value: "avc1.64002a", label: "H.264 (AVC)", description: "Best compatibility, widely supported" },
    { value: "av01.0.08M.08", label: "AV1", description: "High efficiency, modern format" },
]

export function StreamConfigurationCard({ resolutions, config, onChange }: StreamConfigurationCardProps) {

    const resolutionOptions: SelectOption[] = resolutions.map(res => ({
        value: res,
        label: res,
        description: res.includes("1080") ? "Full HD" : res.includes("720") ? "HD" : "Standard Definition"
    }))

    const handleChange = (key: keyof StreamConfig, value: string) => {
        onChange({ ...config, [key]: value })
    }

    return (
        <Card className="w-full border-0 shadow-none bg-transparent">
            <CardContent className="p-0 space-y-8">

                {/* Header */}
                <div className="flex items-center gap-3 pb-2 border-b border-gray-100">
                    <div className="p-2 bg-happy-blue/10 rounded-lg text-happy-blue">
                        <Settings2 className="w-5 h-5" />
                    </div>
                    <div>
                        <h3 className="text-lg font-bold text-gray-900">Stream Configuration</h3>
                        <p className="text-sm text-gray-500">Customize your output parameters</p>
                    </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-8">

                    {/* Section 1: Resolution */}
                    <div className="space-y-4">
                        <div className="flex items-center gap-2 text-gray-700 font-semibold">
                            <Layers className="w-4 h-4 text-happy-purple" />
                            <h4>Resolution</h4>
                        </div>
                        <Select
                            options={resolutionOptions}
                            value={config.resolution}
                            onChange={(val) => handleChange("resolution", val)}
                            placeholder="Choose quality"
                            className="w-full"
                        />
                    </div>

                    {/* Section 2: VCodec */}
                    <div className="space-y-4">
                        <div className="flex items-center gap-2 text-gray-700 font-semibold">
                            <Film className="w-4 h-4 text-happy-pink" />
                            <h4>Video Codec</h4>
                        </div>
                        <Select
                            options={vcodecOptions}
                            value={config.vcodec}
                            onChange={(val) => handleChange("vcodec", val)}
                            placeholder="Select codec"
                            className="w-full"
                        />
                    </div>

                </div>

                {/* Section 3: Content Length (Full Width) */}
                <div className="pt-4 space-y-6">
                    <div className="flex items-center gap-2 text-gray-700 font-semibold">
                        <Timer className="w-4 h-4 text-happy-orange" />
                        <h4>Segment Length</h4>
                    </div>

                    <div className="bg-gray-50/50 p-6 rounded-2xl border border-gray-100/80">
                        <Slider
                            value={parseInt(config.content_length)}
                            min={2}
                            max={60}
                            step={1}
                            onChange={(val) => handleChange("content_length", val.toString())}
                            formatValue={(val) => `${val}s`}
                            label="HLS Segment Duration"
                        />
                        <p className="text-xs text-gray-400 mt-4 text-center">
                            Lower duration reduces latency but increases HTTP requests. <br />
                            <span className="font-semibold text-happy-blue">Recommended: 6s - 10s</span>
                        </p>
                    </div>
                </div>

            </CardContent>
        </Card>
    )
}
