export interface SessionDto {
  name: string;
  path: string;
  size: string;
  age_days: number;
  content_preview: string | null;
}

export interface ProjectDto {
  name: string;
  path: string;
  session_count: number;
  total_size: string;
  sessions: SessionDto[];
}

export interface StatisticsDto {
  total_projects: number;
  total_sessions: number;
  total_size: string;
}
