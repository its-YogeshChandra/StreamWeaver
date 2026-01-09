"use client";

import { motion } from "framer-motion";

export function BackgroundShapes() {
    return (
        <div className="fixed inset-0 -z-10 overflow-hidden pointer-events-none">
            {/* Top Left Yellow Blob */}
            <motion.div
                initial={{ opacity: 0, scale: 0.8 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 1.5, ease: "easeOut" }}
                className="absolute -top-[10%] -left-[5%] w-[40vw] h-[40vw] rounded-full bg-happy-yellow blur-3xl opacity-60"
            />

            {/* Right Coral Blob */}
            <motion.div
                initial={{ opacity: 0, x: 100 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ duration: 1.5, delay: 0.2, ease: "easeOut" }}
                className="absolute top-[20%] -right-[10%] w-[35vw] h-[35vw] rounded-full bg-happy-coral blur-3xl opacity-40"
            />

            {/* Bottom Left Mint Blob */}
            <motion.div
                initial={{ opacity: 0, y: 100 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 1.5, delay: 0.4, ease: "easeOut" }}
                className="absolute -bottom-[10%] -left-[10%] w-[45vw] h-[45vw] rounded-full bg-happy-mint blur-3xl opacity-40"
            />
        </div>
    );
}
