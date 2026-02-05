import { motion } from "framer-motion";
import { Activity, Shield, Zap, Globe } from "lucide-react";

const stats = [
  { value: "99.99%", label: "Uptime" },
  { value: "12ms", label: "Avg Response" },
  { value: "24/7", label: "Monitoring" },
  { value: "150+", label: "Global Nodes" },
];

const features = [
  {
    icon: <Activity size={28} />,
    title: "Real-Time Monitoring",
    desc: "Instant alerts and live telemetry for all your services."
  },
  {
    icon: <Shield size={28} />,
    title: "Enterprise Security",
    desc: "End-to-end encryption and secure global infrastructure."
  },
  {
    icon: <Zap size={28} />,
    title: "Lightning Fast",
    desc: "Low latency global network for instant status checks."
  },
  {
    icon: <Globe size={28} />,
    title: "Worldwide Coverage",
    desc: "Monitor from 150+ regions across the globe."
  },
];

export default function App() {
  return (
    <div className="bg-[#0b0f19] h-svh  text-white  font-sans overflow-x-hidden">

      {/* Background Glow */}
      <div className="absolute inset-0 -z-10 bg-[radial-gradient(circle_at_30%_20%,rgba(59,130,246,0.15),transparent)]" />

      {/* Navbar */}
      <nav className="flex justify-between items-center px-6 md:px-12 py-6">
        <h1 className="text-xl font-bold tracking-wide">SkyWatch</h1>
        <button className="px-5 py-2 bg-white text-black rounded-full font-medium hover:scale-105 transition">
          Get Started
        </button>
      </nav>

      {/* HERO */}
      <main className="text-center px-6 pt-16 pb-24 animate-float">
        <motion.h1
          initial={{ opacity: 0, y: 40 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
          className="text-4xl md:text-6xl font-bold leading-tight bg-linear-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent animate-gradient"
        >
          Your Infrastructure. <br /> Always Online.
        </motion.h1>

        <p className="mt-6 text-gray-400 max-w-xl mx-auto">
          Powerful uptime monitoring with instant alerts, global checks, and real-time performance insights.
        </p>

        <div className="mt-8 flex justify-center gap-4">
          <button className="px-6 py-3 bg-blue-500 rounded-lg font-medium hover:bg-blue-600 transition">
            Start Monitoring
          </button>
          <button className="px-6 py-3 border border-white/20 rounded-lg hover:bg-white/5 transition">
            Live Demo
          </button>
        </div>

        <div className="mt-6 text-emerald-400 text-sm animate-pulse">
          ● All systems operational
        </div>
      </main>

      {/* STATS */}
      <section className="grid grid-cols-2 md:grid-cols-4 gap-6 px-6 md:px-16 pb-24">
        {stats.map((stat, i) => (
          <motion.div
            key={i}
            initial={{ opacity: 0, y: 30 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ delay: i * 0.1 }}
            className="p-6 rounded-xl bg-white/5 border border-white/10 backdrop-blur-xl text-center hover:scale-105 transition"
          >
            <h2 className="text-2xl font-bold text-blue-400">{stat.value}</h2>
            <p className="text-gray-400 text-sm">{stat.label}</p>
          </motion.div>
        ))}
      </section>

      {/* FEATURES */}
      <section className="px-6 md:px-16 pb-32">
        <h2 className="text-3xl font-bold text-center mb-12">Built for Reliability</h2>

        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
          {features.map((f, i) => (
            <motion.div
              key={i}
              whileHover={{ y: -10 }}
              className="group p-8 rounded-2xl border border-white/10 bg-white/5 backdrop-blur-xl hover:border-blue-400/40 transition"
            >
              <div className="text-blue-400 mb-4">{f.icon}</div>
              <h3 className="font-semibold text-lg">{f.title}</h3>
              <p className="text-gray-400 mt-2 text-sm">{f.desc}</p>
            </motion.div>
          ))}
        </div>
      </section>

      {/* FINAL CTA */}
      <section className="relative text-center px-6 pb-32">
        <div className="absolute -inset-20 bg-linear-to-r from-blue-500/20 via-purple-500/20 to-pink-500/20 blur-3xl -z-10" />
        <h2 className="text-4xl font-bold">Ready to stay online?</h2>
        <p className="text-gray-400 mt-4">Start monitoring your infrastructure in under 60 seconds.</p>
        <button className="mt-8 px-8 py-4 bg-white text-black font-semibold rounded-xl hover:scale-105 transition">
          Create Free Account
        </button>
      </section>

      {/* FOOTER */}
      <footer className="text-center text-gray-500 pb-10 text-sm">
        © {new Date().getFullYear()} SkyWatch. All rights reserved.
      </footer>
    </div>
  );
}
