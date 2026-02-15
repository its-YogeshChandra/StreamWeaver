import { Header } from "./header";

export function DashboardShell({ children }: { children: React.ReactNode }) {
    return (
        <div className="flex min-h-screen flex-col bg-muted/20">
            <Header />
            <main className="flex-1 container mx-auto p-4 md:p-8">
                {children}
            </main>
        </div>
    );
}
