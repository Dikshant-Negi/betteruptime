import { useState } from "react";
import { LayoutDashboard, Globe, Plus, LogOut } from "lucide-react";
import { useLocation, useNavigate } from "react-router-dom";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { fetchWebsite} from "../../api/api";
import type { Website } from "../../types/types";
import AddMonitor from "./AddMonitor";
import LineGraph from "./LineGraph";
import Logo from "../../assets/Logo";

export default function DashBoard() {
    const navigate = useNavigate();
    const location = useLocation();
    const queryClient = useQueryClient();

    const [isModalOpen, setIsModalOpen] = useState(false);

    const { data: website = [], isLoading, isError } = useQuery ({
        queryKey: ["website"],
        queryFn: fetchWebsite,
        retry: 1,
        refetchInterval: 60000,
        refetchIntervalInBackground: true,
    });

    const totalSites = website.length;
    const upSites = website.filter((w) => w.status === "UP").length;
    const downSites = website.filter((w) => w.status === "DOWN").length;

    const uptimePercentage = totalSites === 0 ? 100: Math.round((upSites / totalSites) * 100);

    const handleLogout = () => {
        localStorage.removeItem("token");
        navigate("/");
    }
    
    return (
        <div className="flex h-screen bg-primary-100 text-gray-300 font-sans">

            <aside className="w-64 border-r border-border-main bg-primary-100 flex flex-col justify-between p-4 hidden md:flex">
                <div>
                    <div className="mb-8 flex items-center gap-3">
                        <Logo />
                        <span className="text-xl font-bold tracking-tight">SkyWatch</span>
                    </div>

                    <nav className="space-y-2">
                        <SidebarItem icon={<LayoutDashboard size={20} />} label="Dashboard" active={location.pathname === "/dashboard"} onClick={() => navigate("/dashboard")} />
                    </nav>   
                </div>

                <button onClick={handleLogout} className="flex item-center gap-3 px-4 py-3 text-sm font-medium text-red-400 hover:bg-red-500/10 rounded-xl transition-colors">
                    <LogOut size={20} /> Logout
                </button>
            </aside>
                <main className="flex-1 overflow-y-auto bg-primary-100 p-8">

                    <header className="flex justify-between items-center mb-10">
                        <div>
                            <h1 className="text-2xl font-bold text-white">Dashboard</h1>
                            <p className="text-gray-500 text-sm mt-1">Real-time overview of your websites</p>
                        </div>

                        <div className="flex items-center gap-3">
                            <button 
                                onClick={handleLogout}
                                className="md:hidden p-2.5 text-red-400 bg-red-500/10 hover:bg-red-500/20 rounded-xl transition-colors"
                                title="Logout"
                            >
                                <LogOut size={20} />
                            </button>
                            <button 
                                onClick={() => setIsModalOpen(true)}
                                className="bg-brand-blue hover:opacity-90 text-primary-100 px-4 py-2.5 rounded-xl font-bold flex items-center gap-2 transition-all shadow-lg shadow-brand-blue/20"
                            >
                                <Plus size={18} />
                                <span className="hidden sm:inline">Add Monitor</span> 
                            </button>
                        </div>
                    </header>

                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
                        <StatsCard label = "Overall Uptime" value={`${uptimePercentage}%`} color="text-green-400" />
                        <StatsCard label="Monitors UP" value={String(upSites)} color="text-brand-blue" />
                        <StatsCard label="Monitors DOWN" value={String(downSites)} color="text-red-400" />
                        
                    </div>

                    {isLoading && (
                        <div className="text-center py-20 text-gray-500 animate-pulse">
                            Loading your Monitors...
                        </div>
                    )}

                    {isError && (
                        <div className="bg-red-500/10 border border-red-500/20 text-red-400 p-4 rounded-xl mb-6">
                            Faild to load websites!
                        </div>
                    )}

                    {!isLoading && !isError && website.length === 0 && (
                        <div className="text-center py-20 border border-dashed border-border-main rounded-2xl bg-card-header/30">
                            <Globe className="m-auto h-12 w-12 text-gray-600 mb-4" />
                            <h3 className="text-lg font-medium text-white">No monitor yet</h3>
                            <p className="text-gray-500 mb-6">Add your first website to start tracking uptime.</p>
                            <button onClick={() => setIsModalOpen(true)} className="text-brand-blue hover:underline">
                                Add one now &rarr;
                            </button>
                        </div>
                    )}

                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {website.map((site) => (
                            <MonitorCard key={site.id} data={site} />
                        ))}
                    </div>
                        {isModalOpen && (
                            <AddMonitor
                                onClose={() => setIsModalOpen(false)}
                                onSuccess={() => queryClient.invalidateQueries({ queryKey: ["website"] })}
                            />
                        )}
                </main>
        </div>
    );
}

function SidebarItem({ icon, label, active = false, onClick }: { icon: any; label: string; active?: boolean; onClick?: () => void; }) {
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

function StatsCard({ label, value, color }: { label: string; value: string; color: string }) {
  return (
    <div className="bg-card-header border border-border-main p-6 rounded-2xl">
      <h3 className="text-gray-400 text-sm font-medium mb-1">{label}</h3>
      <p className={`text-3xl font-bold ${color}`}>{value}</p>
    </div>
  );
}

function MonitorCard({ data }: { data: Website }) {
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
            
            <a 
                href={data.url} 
                target="_blank" 
                rel="noreferrer" 
                className="text-xs text-gray-500 hover:underline block truncate"
            >
                {data.url}
            </a>
        </div>
        
        <span className={`shrink-0 px-2 py-1 rounded-md text-xs font-bold ${isUp ? "bg-green-500/10 text-green-400 border border-green-500/20" : "bg-red-500/10 text-red-400 border border-red-500/20"}`}>
          {isUp ? "200 OK" : "500 ERR"}
        </span>
      </div>
      <div className="h-24 mt-4 w-full">
        <LineGraph websiteId={String(data.id)} />
      </div>
      <div className="flex justify-between items-center mt-4 pt-4 border-t border-border-main text-xs text-gray-400">
        <span>Last checked: <span className="text-white font-medium">{data.last_checked_at ? new Date(data.last_checked_at).toLocaleTimeString() : "Pending"}</span></span>
      </div>
    </div>
  );
}
