import { User, Video } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ModeToggle } from "@/components/ui/mode-toggle";
import Link from "next/link";

export function Header() {
    return (
        <header className="sticky top-0 z-30 flex h-16 items-center justify-between gap-4 border-b bg-background/95 px-6 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <Link href="/" className="flex items-center gap-2 font-semibold">
                <Video className="h-6 w-6 text-indigo-500" />
                <span className="text-xl tracking-tight">StreamWeaver</span>
            </Link>

            <div className="flex items-center gap-4">
                <ModeToggle />
                <Button variant="secondary" size="icon" className="rounded-full">
                    <User className="h-5 w-5" />
                    <span className="sr-only">User menu</span>
                </Button>
            </div>
        </header>
    );
}
