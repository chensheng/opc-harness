import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { invoke } from '@tauri-apps/api/core'
import type { Project, PRD, UserPersona, CompetitorAnalysis } from '@/types'

interface ProjectState {
  projects: Project[]
  currentProjectId: string | null
  isLoading: boolean
}

interface ProjectActions {
  // 从数据库加载所有项目
  loadProjectsFromDatabase: () => Promise<void>
  
  // 创建项目并保存到数据库
  createProject: (name: string, description: string, idea?: string) => Promise<Project>
  
  updateProject: (id: string, updates: Partial<Project>) => void
  deleteProject: (id: string) => Promise<void>
  setCurrentProject: (id: string | null) => void
  getCurrentProject: () => Project | undefined
  getProjectById: (id: string) => Project | undefined
  
  setProjectPRD: (id: string, prd: PRD) => Promise<void>
  setProjectPersonas: (id: string, personas: UserPersona[]) => Promise<void>
  setProjectCompetitorAnalysis: (id: string, analysis: CompetitorAnalysis) => Promise<void>
  
  updateProjectProgress: (id: string, progress: number) => void
  updateProjectStatus: (id: string, status: Project['status']) => void
  
  // 手动同步到数据库（保留作为备用）
  syncProjectToDatabase: (id: string) => Promise<void>
}

export const useProjectStore = create<ProjectState & ProjectActions>()(
  immer((set, get) => ({
    projects: [],
    currentProjectId: null,
    isLoading: false,

    // 从数据库加载所有项目
    loadProjectsFromDatabase: async () => {
      try {
        set({ isLoading: true })
        console.log('[ProjectStore] Loading projects from database...')
        
        const dbProjects = await invoke<any[]>('get_all_projects')
        
        // 转换数据库格式为前端格式
        const projects: Project[] = dbProjects.map(dbProj => ({
          id: dbProj.id,
          name: dbProj.name,
          description: dbProj.description || '',
          status: (dbProj.status as Project['status']) || 'idea',
          progress: dbProj.progress || 0,
          createdAt: dbProj.created_at || new Date().toISOString(),
          updatedAt: dbProj.updated_at || new Date().toISOString(),
          idea: dbProj.idea || undefined,
          prd: dbProj.prd ? JSON.parse(dbProj.prd) : undefined,
          userPersonas: dbProj.user_personas ? JSON.parse(dbProj.user_personas) : undefined,
          competitorAnalysis: dbProj.competitor_analysis ? JSON.parse(dbProj.competitor_analysis) : undefined,
        }))
        
        set(state => {
          state.projects = projects
        })
        
        console.log(`[ProjectStore] Loaded ${projects.length} projects from database`)
      } catch (error) {
        console.error('[ProjectStore] Failed to load projects from database:', error)
      } finally {
        set({ isLoading: false })
      }
    },

    // 创建项目并保存到数据库
    createProject: async (name, description, idea) => {
      // 异步保存到数据库，获取后端生成的 ID
      try {
        const projectId = await invoke<string>('create_project', {
          name,
          description,
        })
        
        // 创建项目对象，使用后端返回的 ID
        const project: Project = {
          id: projectId,
          name,
          description,
          status: 'idea',
          progress: 0,
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          idea,
        }
        
        // 添加到本地状态
        set(state => {
          state.projects.push(project)
          state.currentProjectId = project.id
        })
        
        console.log('[ProjectStore] Project created and saved to database:', projectId)
        return project
      } catch (error) {
        console.error('[ProjectStore] Failed to save project to database:', error)
        throw error
      }
    },

    updateProject: (id, updates) =>
      set(state => {
        const project = state.projects.find(p => p.id === id)
        if (project) {
          Object.assign(project, updates, { updatedAt: new Date().toISOString() })
        }
      }),

    deleteProject: async (id) => {
      set(state => {
        state.projects = state.projects.filter(p => p.id !== id)
        if (state.currentProjectId === id) {
          state.currentProjectId = null
        }
      })
      
      // 异步删除数据库记录
      try {
        await invoke('delete_project', { id })
        console.log('[ProjectStore] Project deleted from database:', id)
      } catch (error) {
        console.error('[ProjectStore] Failed to delete project from database:', error)
      }
    },

    setCurrentProject: id =>
      set(state => {
        state.currentProjectId = id
      }),

    getCurrentProject: () => {
      const { projects, currentProjectId } = get()
      return projects.find(p => p.id === currentProjectId)
    },

    getProjectById: id => {
      return get().projects.find(p => p.id === id)
    },

    setProjectPRD: async (id, prd) => {
      set(state => {
        const project = state.projects.find(p => p.id === id)
        if (project) {
          project.prd = prd
          project.updatedAt = new Date().toISOString()
        }
      })
      
      // 异步保存到数据库
      try {
        await get().syncProjectToDatabase(id)
      } catch (error) {
        console.error('[ProjectStore] Failed to save PRD to database:', error)
      }
    },

    setProjectPersonas: async (id, personas) => {
      set(state => {
        const project = state.projects.find(p => p.id === id)
        if (project) {
          project.userPersonas = personas
          project.updatedAt = new Date().toISOString()
        }
      })
      
      // 异步保存到数据库
      try {
        await get().syncProjectToDatabase(id)
      } catch (error) {
        console.error('[ProjectStore] Failed to save personas to database:', error)
      }
    },

    setProjectCompetitorAnalysis: async (id, analysis) => {
      set(state => {
        const project = state.projects.find(p => p.id === id)
        if (project) {
          project.competitorAnalysis = analysis
          project.updatedAt = new Date().toISOString()
        }
      })
      
      // 异步保存到数据库
      try {
        await get().syncProjectToDatabase(id)
      } catch (error) {
        console.error('[ProjectStore] Failed to save competitor analysis to database:', error)
      }
    },

    updateProjectProgress: (id, progress) =>
      set(state => {
        const project = state.projects.find(p => p.id === id)
        if (project) {
          project.progress = Math.min(100, Math.max(0, progress))
          project.updatedAt = new Date().toISOString()
        }
      }),

    updateProjectStatus: (id, status) =>
      set(state => {
        const project = state.projects.find(p => p.id === id)
        if (project) {
          project.status = status
          project.updatedAt = new Date().toISOString()
        }
      }),

    // 同步项目到数据库
    syncProjectToDatabase: async (id) => {
      const project = get().projects.find(p => p.id === id)
      if (!project) {
        console.warn('[ProjectStore] Project not found for sync:', id)
        return
      }

      try {
        console.log('[ProjectStore] Syncing project to database:', id)
        
        // 将复杂对象序列化为 JSON 字符串，并使用 camelCase 格式与后端交互
        const projectForDb = {
          id: project.id,
          name: project.name,
          description: project.description,
          status: project.status,
          progress: project.progress,
          createdAt: project.createdAt,
          updatedAt: project.updatedAt,
          idea: project.idea || null,
          prd: project.prd ? JSON.stringify(project.prd) : null,
          userPersonas: project.userPersonas ? JSON.stringify(project.userPersonas) : null,
          competitorAnalysis: project.competitorAnalysis ? JSON.stringify(project.competitorAnalysis) : null,
        }

        await invoke('update_project', { project: projectForDb })
        console.log('[ProjectStore] Project synced successfully')
      } catch (error) {
        console.error('[ProjectStore] Failed to sync project to database:', error)
        throw error
      }
    },
  }))
)
