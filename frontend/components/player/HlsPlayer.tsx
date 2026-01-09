import React, { useEffect, useRef } from 'react';
import Hls from 'hls.js';
import { Card } from "@/components/ui/card";

interface HlsPlayerProps {
    src: string;
}

export function HlsPlayer({ src }: HlsPlayerProps) {
    const videoRef = useRef<HTMLVideoElement>(null);

    useEffect(() => {
        const video = videoRef.current;
        if (!video) return;

        if (Hls.isSupported()) {
            const hls = new Hls();
            hls.loadSource(src);
            hls.attachMedia(video);
            hls.on(Hls.Events.MANIFEST_PARSED, () => {
                video.play().catch(e => console.log("Auto-play prevented:", e));
            });

            return () => {
                hls.destroy();
            };
        } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
            // Native support (Safari)
            video.src = src;
            video.addEventListener('loadedmetadata', () => {
                video.play().catch(e => console.log("Auto-play prevented:", e));
            });
        }
    }, [src]);

    return (
        <div className="relative w-full aspect-video rounded-2xl overflow-hidden shadow-2xl bg-black">
            <video
                ref={videoRef}
                controls
                className="w-full h-full object-contain"
                poster="/placeholder-video.jpg" // You might want to generate a thumbnail later
            />
        </div>
    );
}
