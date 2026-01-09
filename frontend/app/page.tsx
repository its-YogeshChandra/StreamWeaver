import { BackgroundShapes } from "@/components/layout/BackgroundShapes";
import { ConverterTool } from "@/components/tool/ConverterTool";

export default function Home() {
  return (
    <main className="relative min-h-screen flex flex-col items-center justify-center p-6 md:p-12 overflow-hidden">
      <BackgroundShapes />

      <div className="z-10 w-full max-w-5xl mx-auto space-y-12">
        <header className="text-center space-y-4">
          <h1 className="text-5xl md:text-7xl font-extrabold tracking-tight text-gray-900">
            Stream<span className="text-happy-blue">Weaver</span>
          </h1>
          <p className="text-xl md:text-2xl text-gray-600 max-w-2xl mx-auto font-light">
            Convert any video into professional adaptive streaming formats instantly.
          </p>
        </header>

        <ConverterTool />

        <footer className="text-center text-sm text-gray-400 pt-12">
          &copy; {new Date().getFullYear()} StreamWeaver. Designed for creators.
        </footer>
      </div>
    </main>
  );
}
