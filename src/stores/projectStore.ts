import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

export interface Project {
  id: string;
  name: string;
  description: string;
  status: 'draft' | 'designing' | 'coding' | 'marketing' | 'completed';
  createdAt: number;
  updatedAt: number;
  path?: string;
}

interface ProjectState {
  // 项目列表
  projects: Project[];
  
  // 当前选中的项目
  currentProject: Project | null;
  
  // 项目操作
  addProject: (project: Omit<Project, 'id' | 'createdAt' | 'updatedAt'>) => void;
  updateProject: (id: string, updates: Partial<Project>) => void;
  deleteProject: (id: string) => void;
  setCurrentProject: (project: Project | null) => void;
  
  // 查询
  getProjectById: (id: string) => Project | undefined;
  getProjectsByStatus: (status: Project['status']) => Project[];
}

export const useProjectStore = create<ProjectState>()(
  devtools(
    immer((set, get) => ({
      // 初始状态
      projects: [],
      currentProject: null,

      addProject: (project) =>
        set((state) => {
          const now = Date.now();
          const newProject: Project = {
            ...project,
            id: crypto.randomUUID(),
            createdAt: now,
            updatedAt: now,
          };
          state.projects.push(newProject);
        }),

      updateProject: (id, updates) =>
        set((state) => {
          const project = state.projects.find((p) => p.id === id);
          if (project) {
            Object.assign(project, updates, { updatedAt: Date.now() });
          }
          if (state.currentProject?.id === id) {
            Object.assign(state.currentProject, updates, { updatedAt: Date.now() });
          }
        }),

      deleteProject: (id) =>
        set((state) => {
          state.projects = state.projects.filter((p) => p.id !== id);
          if (state.currentProject?.id === id) {
            state.currentProject = null;
          }
        }),

      setCurrentProject: (project) =>
        set((state) => {
          state.currentProject = project;
        }),

      getProjectById: (id) => {
        return get().projects.find((p) => p.id === id);
      },

      getProjectsByStatus: (status) => {
        return get().projects.filter((p) => p.status === status);
      },
    })),
    { name: 'ProjectStore' }
  )
);
