import {
  LineChart,
  Line,
  XAxis,
  Tooltip,
  ResponsiveContainer,
  CartesianGrid,
} from "recharts";
import { format } from "date-fns";
import { useQuery } from "@tanstack/react-query";
import { fetchReliability } from "../../api/api";

const formatCustomDuration = (seconds: number) => {
  if (seconds === 0) return "0s";
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
};

export default function LineGraph({ websiteId }: { websiteId: string }) {
  const { data, isLoading } = useQuery({
    queryKey: ["reliability", websiteId],
    queryFn: () => fetchReliability(websiteId),
    refetchInterval: 60000,
    select: (data) => [...data].reverse(),
  });

  if (isLoading)
    return (
      <div className="h-full w-full flex items-end pb-2">
        <div className="w-full h-1/2 bg-gray-700/20 animate-pulse rounded"></div>
      </div>
    );

  if (!data || data.length === 0)
    return (
      <div className="text-xs text-gray-600 mt-8 text-center">
        No data available.
      </div>
    );

  return (
    <div className="w-full h-full" style={{ minHeight: "100px" }}>
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" stroke="#334155" opacity={0.2} vertical={false} />
          
          <XAxis dataKey="date" hide />

          <Tooltip
            cursor={{ stroke: "rgba(255,255,255,0.2)", strokeWidth: 2 }}
            contentStyle={{
              backgroundColor: "#1e293b",
              borderColor: "#334155",
              color: "#e2e8f0",
              fontSize: "12px",
              borderRadius: "8px",
              boxShadow: "0 4px 6px -1px rgba(0, 0, 0, 0.5)",
            }}
            labelFormatter={(label: any) => {
               if (!label) return "";
               return format(new Date(label), "MMM d, yyyy");
            }}
            formatter={(value: any, name: any) => [
              formatCustomDuration(Number(value)),
              name
            ]}
          />

          <Line
            type="monotone"
            dataKey="up_seconds"
            stroke="#22c55e" 
            strokeWidth={2}
            dot={true} 
            activeDot={{ r: 4, fill: "#22c55e" }}
            name="Uptime"
          />

          
          <Line
            type="monotone"
            dataKey="down_seconds"
            stroke="#ef4444" 
            strokeWidth={2}
            dot={true}
            activeDot={{ r: 4, fill: "#ef4444" }}
            name="Downtime"
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}