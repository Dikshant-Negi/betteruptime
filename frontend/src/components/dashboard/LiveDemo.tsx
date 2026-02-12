import React from "react";
import { LayoutDashboard, Globe, Plus, ArrowLeft } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { subDays } from "date-fns";
import Logo from "../../assets/Logo"; 
import LineGraph from "./LineGraph";

// --- 1. TYPE DEFINITIONS (Fixes the 'any' error) ---

interface GraphDataPoint {
  date: string;
  up_seconds: number;
  down_seconds: number;
}

interface DemoWebsite {
  id: string;
  name: string;
  url: string;
  status: "UP" | "DOWN"; 
  last_checked_at: string;
  graphData: GraphDataPoint[];
}

// --- 2. DUMMY DATA GENERATION ---

// Helper to generate fake graph data for the last 7 days
const generateHistory = (): GraphDataPoint[] => {
  const data: GraphDataPoint[] = [];
  for (let i = 6; i >= 0; i--) {
    const date = subDays(new Date(), i).toISOString();
    // If stable, 24h uptime (86400s). If unstable, random drops.
    const totalSeconds = 86400;
    const down = Math.floor(Math.random() * 5000); 
    const up = totalSeconds - down;
    
    data.push({ date, up_seconds: up, down_seconds: down });
  }
  return data;
};


const dummyWebsites: DemoWebsite[] = [
  {
    id: "1",
    name: "Google Production",
    url: "https://google.com",
    status: "UP",
    last_checked_at: new Date().toISOString(),
    graphData: generateHistory(), 
  },
  {
    id: "2",
    name: "Backend API (Staging)",
    url: "https://api.staging.example.com",
    status: "DOWN",
    last_checked_at: new Date(Date.now() - 1000 * 60 * 2).toISOString(), // 2 mins ago
    graphData: generateHistory(), 
  },
  {
    id: "3",
    name: "Landing Page",
    url: "https://example.com",
    status: "UP",
    last_checked_at: new Date().toISOString(),
    graphData: generateHistory(),
  },
];

// --- 3. MAIN DASHBOARD COMPONENT ---

export default function DemoDashboard() {
  const navigate = useNavigate();

  // Calculate fake stats
  const website = dummyWebsites;
  const totalSites = website.length;
  const upSites = website.filter((w) => w.status === "UP").length;
  const downSites = website.filter((w) => w.status === "DOWN").length;
  
  // Handle divide by zero just in case
  const uptimePercentage = totalSites === 0 ? 0 : Math.round((upSites / totalSites) * 100);

  const handleRedirectToSignup = () => {
    navigate("/signup");
  };

  return (
    <div className="flex h-screen bg-primary-100 text-gray-300 font-sans overflow-hidden">
      
      <aside className="w-64 border-r border-border-main bg-primary-100 flex flex-col justify-between p-4 hidden md:flex">
        <div>
          <div className="mb-8 flex items-center gap-3">
            <Logo />
            <span className="text-xl font-bold tracking-tight">SkyWatch</span>
          </div>
          <nav className="space-y-2">
            <SidebarItem icon={<LayoutDashboard size={20} />} label="Demo Dashboard" active={true} />
            
            <SidebarItem icon={<ArrowLeft size={20} />} label="Back to Home" onClick={() => navigate('/')} />
          </nav>
        </div>

        <div className="bg-brand-blue/10 p-4 rounded-xl border border-brand-blue/20">
          <p className="text-xs text-brand-blue mb-2 font-bold">Liked the demo?</p>
          <button 
            onClick={handleRedirectToSignup} 
            className="w-full bg-brand-blue text-white text-sm py-2 rounded-lg font-bold hover:bg-brand-blue/90"
          >
            Create Account
          </button>
        </div>
      </aside>

      
      <main className="flex-1 overflow-y-auto bg-primary-100 p-8">
        
        
        <div className="bg-yellow-500/10 border border-yellow-500/20 text-yellow-400 px-4 py-3 rounded-xl mb-8 flex justify-between items-center">
            <span className="font-medium">⚠️ You are viewing a Live Demo with dummy data.</span>
            <button onClick={handleRedirectToSignup} className="text-sm font-bold underline hover:text-yellow-300">
                Start Monitoring for Real &rarr;
            </button>
        </div>

        <header className="flex justify-between items-center mb-10">
          <div>
            <h1 className="text-2xl font-bold text-white">Dashboard</h1>
            <p className="text-gray-500 text-sm mt-1">Real-time overview of your websites</p>
          </div>

          <div className="flex items-center gap-3">
            <button
              onClick={handleRedirectToSignup}
              className="bg-brand-blue hover:opacity-90 text-primary-100 px-4 py-2.5 rounded-xl font-bold flex items-center gap-2 transition-all shadow-lg shadow-brand-blue/20"
            >
              <Plus size={18} />
              <span className="hidden sm:inline">Add Monitor</span>
            </button>
          </div>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
          <StatsCard label="Overall Uptime" value={`${uptimePercentage}%`} color="text-green-400" />
          <StatsCard label="Monitors UP" value={String(upSites)} color="text-brand-blue" />
          <StatsCard label="Monitors DOWN" value={String(downSites)} color="text-red-400" />
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {website.map((site) => (
            <DemoMonitorCard key={site.id} data={site} />
          ))}
        </div>
      </main>
    </div>
  );
}



function DemoMonitorCard({ data }: { data: DemoWebsite }) {
  const isUp = data.status === "UP";

  return (
    <div className="group relative bg-card-header border border-border-main p-5 rounded-2xl hover:border-gray-500 transition-all duration-300">
      <div className="flex items-center gap-4 mb-4">
        <div className={`shrink-0 h-10 w-10 rounded-full flex items-center justify-center ${isUp ? "bg-green-500/20 text-green-400" : "bg-red-500/20 text-red-400"}`}>
          <Globe size={20} />
        </div>
        <div className="min-w-0 flex-1">
          <h3 className="font-bold text-white group-hover:text-brand-blue transition-colors truncate">
            {data.name}
          </h3>
          <p className="text-xs text-gray-500 block truncate">{data.url}</p>
        </div>
        <span className={`shrink-0 px-2 py-1 rounded-md text-xs font-bold ${isUp ? "bg-green-500/10 text-green-400 border border-green-500/20" : "bg-red-500/10 text-red-400 border border-red-500/20"}`}>
          {isUp ? "200 OK" : "500 ERR"}
        </span>
      </div>
      
      {/* REUSING YOUR REAL LINE GRAPH */}
      <div className="h-24 mt-4 w-full">
        <LineGraph staticData={data.graphData} />
      </div>

      <div className="flex justify-between items-center mt-4 pt-4 border-t border-border-main text-xs text-gray-400">
        <span>Last checked: <span className="text-white font-medium">{new Date(data.last_checked_at).toLocaleTimeString()}</span></span>
      </div>
    </div>
  );
}

interface SidebarItemProps {
  icon: React.ReactNode;
  label: string;
  active?: boolean;
  onClick?: () => void;
}

function SidebarItem({ icon, label, active = false, onClick }: SidebarItemProps) {
  return (
    <div
      onClick={onClick}
      className={`flex items-center gap-3 px-4 py-3 rounded-xl cursor-pointer transition-all ${
        active
          ? "bg-brand-blue text-primary-100 font-bold shadow-lg shadow-brand-blue/20"
          : "hover:bg-card-header text-gray-400 hover:text-white"
      }`}
    >
      {icon}
      <span className="text-sm">{label}</span>
    </div>
  );
}

interface StatsCardProps {
  label: string;
  value: string;
  color: string;
}

function StatsCard({ label, value, color }: StatsCardProps) {
  return (
    <div className="bg-card-header border border-border-main p-6 rounded-2xl">
      <h3 className="text-gray-400 text-sm font-medium mb-1">{label}</h3>
      <p className={`text-3xl font-bold ${color}`}>{value}</p>
    </div>
  );
}