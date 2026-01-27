
export interface AuthResponse {
  success: boolean;
  jwt: string;
}

export interface CreateWebsitePayload {
  name: string;
  url: string;
  check_interval: number;
}

export interface WebsiteStats {
  total_uptime_seconds: number;
  total_downtime_seconds: number;
  last_status_change: string;
}

export interface Website {
  id: number;
  user_id: number;
  name: string;
  url: string;
  check_interval: number;
  stats?: WebsiteStats;
  status: string;
  last_checked_at: number;
}

export interface Incident {
  id: number;
  website_id: number;
  start_time: string;
  end_time: string | null;
  error_reason: string;
}