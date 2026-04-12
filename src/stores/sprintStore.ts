import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import type { Sprint } from '@/types'

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

    loadProjectSprints: async _projectId => {
      // TODO: 实现从后端加载Sprint的逻辑
      // 目前使用空数组作为占位
      try {
        set({ isLoading: true })
        // const response = await invoke<{ success: boolean; sprints: any[] }>('get_sprints', {
        //   request: { project_id: projectId },
        // })
        // if (response.success) {
        //   set(state => {
        //     state.sprintsByProject[projectId] = response.sprints
        //   })
        // }
      } catch (error) {
        console.error('[SprintStore] Failed to load sprints:', error)
      } finally {
        set({ isLoading: false })
      }
    },

    setProjectSprints: async (projectId, sprints) => {
      try {
        set({ isLoading: true })
        // TODO: 实现保存到后端的逻辑
        // await invoke<{ success: boolean; count: number }>('save_sprints', {
        //   request: {
        //     project_id: projectId,
        //     sprints: sprints,
        //   },
        // })

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
      const currentSprints = get().getProjectSprints(projectId)
      const updatedSprints = currentSprints.filter(s => s.id !== sprintId)
      await get().setProjectSprints(projectId, updatedSprints)
    },

    clearProjectSprints: projectId => {
      set(state => {
        delete state.sprintsByProject[projectId]
      })
    },
  }))
)
