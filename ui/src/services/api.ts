import { invoke } from '@tauri-apps/api/core';
import type { ProjectDto, SessionDto, StatisticsDto } from '../types';

export const api = {
  async getStatistics(): Promise<StatisticsDto> {
    return await invoke('get_statistics');
  },

  async scanProjects(): Promise<ProjectDto[]> {
    return await invoke('scan_projects');
  },

  async getProjectSessions(projectPath: string): Promise<SessionDto[]> {
    return await invoke('get_project_sessions', { projectPath });
  },

  async deleteSessions(sessionPaths: string[]): Promise<number> {
    return await invoke('delete_sessions', { sessionPaths });
  },

  async deleteProject(projectPath: string): Promise<void> {
    return await invoke('delete_project', { projectPath });
  },

  async filterSessionsByAge(days: number): Promise<SessionDto[]> {
    return await invoke('filter_sessions_by_age', { days });
  },

  async deleteOldSessions(days: number): Promise<number> {
    return await invoke('delete_old_sessions', { days });
  },

  async findClaudeDirectory(): Promise<string | null> {
    return await invoke('find_claude_directory');
  },

  async getSessionContent(sessionPath: string): Promise<string> {
    return await invoke('get_session_content', { sessionPath });
  },
};
