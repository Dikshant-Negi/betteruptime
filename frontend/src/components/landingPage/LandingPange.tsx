import { Link } from "react-router-dom";
import Logo from "../../assets/logo";

export default function LandingPage() {
    return (
        <div className="min-h-screen w-full bg-primary-100 text-white flex flex-col font-sans">
            <nav className="flex items-center justify-between px-6 py-6 max-w-7xl mx-auto w-full">
                <div className="h-10 w-10 flex items-center gap-1">
                    <Logo />
                    <span className="text-xl font-bold tracking-tight">UptimeWork</span>
                </div>

                <div className="flex items-center gap-4">
                    <Link to="/signin" className="text-sm font-medium text-gray-400 hover:text-white transition">
                        Sign in
                    </Link>
                    <Link to="/signup" className="px-4 py-2 text-sm font-bold bg-white text-primary-100 rounded-lg hover:bg-gray-200 transition">
                        Sign up
                    </Link>
                </div>
            </nav>

            <main className="flex-1 flex flex-col items-center justify-center text-center px-4 mt-10">

                <div className="mb-8 inline-flex items-center rounded-full border border-border-main bg-card-header/50 px-3 py-1 text-xs text-gray-300 backdrop-blur-xl">
                    <span className="flex h-2 w-2 rounded-full bg-green-500 mr-2 animate-pulse"></span>
                    Website Monitoring
                </div>

                <h1 className="text-5xl md: text-7xl font-bold tracking-tight mb-6 max-w-4xl">
                    Monitor Your Websites <br/>
                    <span className="text-transparent bg-clip-text bg-gradient-to-r from-brand-blue to-purple-500">
                        without the chaos.
                    </span>
                </h1>

                <p className="text-lg text-gray-400 max-w-2xl mb-10 leading-relaxed">
                    Get instant Downtime alerts, view daily historical uptime graphs, and manage all yout monitor int one dashboard.
                </p>
                <div className="flex flex-col sm:flex-row gap-4 mb-20">
                    <Link to="singup" className="px-8 py-4 rounded-xl bg-brand-blue font-bold text-primary-100 hover:opacity-90 transition shadow-brand-blue/20">
                        Start Monitoring Your Websites for Free
                    </Link>
                </div>
                <div className="w-full max-w-5xl h-30 rounder-t-3xl relative overflow-hidden"></div>

                <footer className="py-8 text-center text-gary-600 text-sm border-t border-border-main bg-primary-100">
                    <span>&copy; 2026 UptimeWork. Built with Rust and React</span>
                </footer>

            </main>
        </div>
    );
}