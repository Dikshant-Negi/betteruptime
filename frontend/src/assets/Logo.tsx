
interface LogoProps {
  className?: string;
}
export default function Logo({ 
  className = "h-8 w-8",
}: LogoProps) {
  return (
    <div className="flex items-center gap-2 text-white rounded-full bg-card-header">
      <svg
        className={className}
        viewBox="0 0 100 100"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <defs>
          <linearGradient id="neonGradient" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor="#22d3ee" stopOpacity="1" />
            <stop offset="100%" stopColor="#5568f7" stopOpacity="1" />
          </linearGradient>
          <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
            <feGaussianBlur stdDeviation="2.5" result="coloredBlur" />
            <feMerge>
              <feMergeNode in="coloredBlur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>

        <g
          stroke="url(#neonGradient)"
          strokeWidth="4"
          strokeLinecap="round"
          strokeLinejoin="round"
          fill="none"
          filter="url(#glow)"
        >
          {/* The Shield Shape */}
          <path d="M 20 30 V 50 C 20 75 40 85 50 85 C 60 85 80 75 80 50 V 30" />

          {/* The Pulse Line */}
          <path d="M 20 50 H 35 L 45 35 L 55 65 L 65 50 H 80" />

          {/* The Nodes */}
          <circle cx="20" cy="30" r="3" fill="#0f172a" />
          <circle cx="80" cy="30" r="3" fill="#0f172a" />
          <circle cx="50" cy="85" r="3" fill="#0f172a" />
          <circle cx="20" cy="50" r="2" fill="#22d3ee" stroke="none" />
          <circle cx="80" cy="50" r="2" fill="#a855f7" stroke="none" />
        </g>
      </svg>
    </div>
  );
}