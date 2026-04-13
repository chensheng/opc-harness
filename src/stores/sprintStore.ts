import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { invoke } from '@tauri-apps/api/core'
import type { Sprint } from '@/types'

// 后端返回的 camelCase 格式的 Sprint 类型
interface BackendSprint {
  id: string
  projectId: string
  name: string
  goal: string
  startDate: string
  endDate: string
  status: string
  totalStoryPoints?: number
  completedStoryPoints?: number
  createdAt: string
  updatedAt: string
}

interface SprintState {
  // 按项目ID存储Sprint计划
  sprintsByProject: Record<string, Sprint[]>
  isLoading: boolean
}

interface SprintActions {
  // 加载项目的Sprint计划
  loadProjectSprints: (projectId: string) => Promise<void>

  // 设置项目的Sprint计划
  setProjectSprints: (projectId: string, sprints: Sprint[]) => Promise<void>

  // 获取项目的Sprint计划
  getProjectSprints: (projectId: string) => Sprint[]

  // 添加单个Sprint
  addSprint: (projectId: string, sprint: Sprint) => Promise<void>

  // 更新单个Sprint
  updateSprint: (projectId: string, sprintId: string, updates: Partial<Sprint>) => Promise<void>

  // 删除单个Sprint
  deleteSprint: (projectId: string, sprintId: string) => Promise<void>

  // 清空项目的Sprint计划
  clearProjectSprints: (projectId: string) => void
}

export const useSprintStore = create<SprintState & SprintActions>()(
  immer((set, get) => ({
    sprintsByProject: {},
    isLoading: false,

    loadProjectSprints: async projectId => {
      try {
        set({ isLoading: true })
        const rustSprints = await invoke<BackendSprint[]>('get_sprints_by_project', {
          request: { project_id: projectId },
        })

        // 将 Rust 后端的 camelCase Sprint 转换为前端的 camelCase 格式
        const frontendSprints = rustSprints.map(sprint => ({
          id: sprint.id,
          name: sprint.name,
          goal: sprint.goal,
          startDate: sprint.startDate,
          endDate: sprint.endDate,
          status: sprint.status as Sprint['status'],
          totalStoryPoints: sprint.totalStoryPoints,
          completedStoryPoints: sprint.completedStoryPoints,
          createdAt: sprint.createdAt,
          updatedAt: sprint.updatedAt,
        }))

        set(state => {
          state.sprintsByProject[projectId] = frontendSprints
        })
      } catch (error) {
        console.error('[SprintStore] Failed to load sprints:', error)
      } finally {
        set({ isLoading: false })
      }
    },

    setProjectSprints: async (projectId, sprints) => {
      try {
        set({ isLoading: true })

        // 将前端的 camelCase Sprint 转换为 Rust 后端期望的格式
        const rustSprints = sprints.map(sprint => ({
          id: sprint.id,
          projectId: projectId,
          name: sprint.name,
          goal: sprint.goal,
          startDate: sprint.startDate,
          endDate: sprint.endDate,
          status: sprint.status,
          totalStoryPoints: sprint.totalStoryPoints || 0,
          completedStoryPoints: sprint.completedStoryPoints || 0,
          createdAt: sprint.createdAt,
          updatedAt: sprint.updatedAt,
        }))

        // 保存到后端数据库
        await invoke<number>('save_sprints', {
          request: {
            project_id: projectId,
            sprints: rustSprints,
          },
        })

        // 更新本地状态
        set(state => {
          state.sprintsByProject[projectId] = sprints
        })
      } catch (error) {
        console.error('[SprintStore] Failed to save sprints:', error)
        throw error
      } finally {
        set({ isLoading: false })
      }
    },

    getProjectSprints: projectId => {
      return get().sprintsByProject[projectId] || []
    },

    addSprint: async (projectId, sprint) => {
      const currentSprints = get().getProjectSprints(projectId)
      const updatedSprints = [...currentSprints, sprint]
      await get().setProjectSprints(projectId, updatedSprints)
    },

    updateSprint: async (projectId, sprintId, updates) => {
      const currentSprints = get().getProjectSprints(projectId)
      const updatedSprints = currentSprints.map(s =>
        s.id === sprintId ? { ...s, ...updates, updatedAt: new Date().toISOString() } : s
      )
      await get().setProjectSprints(projectId, updatedSprints)
    },

    deleteSprint: async (projectId, sprintId) => {
      try {
        set({ isLoading: true })
        // 从后端数据库删除
        await invoke<void>('delete_sprint', {
          request: {
            sprint_id: sprintId,
          },
        })

        // 更新本地状态
        const currentSprints = get().getProjectSprints(projectId)
        const updatedSprints = currentSprints.filter(s => s.id !== sprintId)
        set(state => {
          state.sprintsByProject[projectId] = updatedSprints
        })
      } catch (error) {
        console.error('[SprintStore] Failed to delete sprint:', error)
        throw error
      } finally {
        set({ isLoading: false })
      }
    },

    clearProjectSprints: projectId => {
      set(state => {
        delete state.sprintsByProject[projectId]
      })
    },
  }))
)
