import { DashboardShell } from "@/components/layout/dashboard-shell";
import { ConverterTool } from "@/components/tool/ConverterTool";

export default function Home() {
  return (
    <DashboardShell>
      <div className="flex flex-col items-center justify-center min-h-[calc(100vh-8rem)] space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-700">
        <div className="text-center space-y-2 max-w-2xl mx-auto px-4">
          <h1 className="text-4xl md:text-6xl font-extrabold tracking-tight lg:text-7xl bg-clip-text text-transparent bg-gradient-to-r from-gray-900 via-gray-700 to-gray-900 dark:from-white dark:via-gray-200 dark:to-gray-400">
            StreamWeaver
          </h1>
          <p className="text-muted-foreground text-lg md:text-xl font-light">
            Professional-grade HLS video transcoding for modern streaming applications.
          </p>
        </div>

        <div className="w-full relative z-10">
          <div className="absolute inset-0 bg-gradient-to-r from-indigo-500/10 via-purple-500/10 to-pink-500/10 blur-3xl -z-10 rounded-full transform scale-75 opacity-50 dark:opacity-30" />
          <ConverterTool />
        </div>
      </div>
    </DashboardShell>
  );
}
