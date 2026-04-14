import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { invoke } from '@tauri-apps/api/core'
import type { UserStory } from '@/types'

// 后端返回的 snake_case 格式的用户故事类型
interface BackendUserStory {
  id: string
  story_number: string
  title: string
  role: string
  feature: string
  benefit: string
  description: string
  acceptance_criteria: string[]
  priority: 'P0' | 'P1' | 'P2' | 'P3'
  status: 'draft' | 'refined' | 'approved' | 'in_development' | 'completed'
  story_points?: number
  dependencies: string[]
  feature_module?: string
  sprint_id?: string
  labels: string[]
  created_at: string
  updated_at: string
}

interface UserStoryState {
  // 按项目ID存储用户故事
  storiesByProject: Record<string, UserStory[]>
  isLoading: boolean
}

interface UserStoryActions {
  // 从数据库加载项目的用户故事
  loadProjectStories: (projectId: string) => Promise<void>

  // 设置项目的用户故事(同时保存到数据库)
  setProjectStories: (projectId: string, stories: UserStory[]) => Promise<void>

  // 获取项目的用户故事
  getProjectStories: (projectId: string) => UserStory[]

  // 获取指定Sprint下的用户故事
  getSprintStories: (projectId: string, sprintId: string) => UserStory[]

  // 添加单个用户故事
  addStory: (projectId: string, story: UserStory) => Promise<void>

  // 更新单个用户故事
  updateStory: (projectId: string, storyId: string, updates: Partial<UserStory>) => Promise<void>

  // 删除单个用户故事
  deleteStory: (projectId: string, storyId: string) => Promise<void>

  // 清空项目的用户故事
  clearProjectStories: (projectId: string) => void
}

export const useUserStoryStore = create<UserStoryState & UserStoryActions>()(
  immer((set, get) => ({
    storiesByProject: {},
    isLoading: false,

    loadProjectStories: async projectId => {
      try {
        set({ isLoading: true })

        const response = await invoke<{ success: boolean; user_stories: BackendUserStory[] }>(
          'get_user_stories',
          {
            request: { project_id: projectId },
          }
        )

        if (response.success) {
          // 将 Rust 后端的 snake_case UserStory 转换为前端的 camelCase 格式
          const frontendStories = response.user_stories.map(story => ({
            id: story.id,
            storyNumber: story.story_number,
            title: story.title,
            role: story.role,
            feature: story.feature,
            benefit: story.benefit,
            description: story.description,
            acceptanceCriteria: story.acceptance_criteria,
            priority: story.priority,
            status: story.status,
            storyPoints: story.story_points,
            dependencies: story.dependencies,
            featureModule: story.feature_module,
            sprintId: story.sprint_id || undefined, // 将 null 转换为 undefined
            labels: story.labels,
            createdAt: story.created_at,
            updatedAt: story.updated_at,
          })) as UserStory[]

          set(state => {
            state.storiesByProject[projectId] = frontendStories
          })
        }
      } catch (error) {
        console.error('[UserStoryStore] Failed to load stories:', error)
      } finally {
        set({ isLoading: false })
      }
    },

    setProjectStories: async (projectId, stories) => {
      try {
        set({ isLoading: true })

        // 将前端的 camelCase UserStory 转换为 Rust 后端期望的 snake_case 格式
        const rustStories = stories.map(story => ({
          id: story.id,
          story_number: story.storyNumber,
          title: story.title,
          role: story.role,
          feature: story.feature,
          benefit: story.benefit,
          description: story.description,
          acceptance_criteria: story.acceptanceCriteria,
          priority: story.priority,
          status: story.status,
          story_points: story.storyPoints,
          dependencies: story.dependencies,
          feature_module: story.featureModule,
          sprint_id: story.sprintId,
          labels: story.labels,
          created_at: story.createdAt,
          updated_at: story.updatedAt,
        }))

        await invoke<{ success: boolean; count: number }>('save_user_stories', {
          request: {
            project_id: projectId,
            user_stories: rustStories,
          },
        })

        // 更新本地状态
        set(state => {
          state.storiesByProject[projectId] = stories
        })
      } catch (error) {
        console.error('[UserStoryStore] Failed to save stories:', error)
        throw error
      } finally {
        set({ isLoading: false })
      }
    },

    getProjectStories: projectId => {
      return get().storiesByProject[projectId] || []
    },

    getSprintStories: (projectId, sprintId) => {
      const stories = get().getProjectStories(projectId)
      return stories.filter(story => story.sprintId === sprintId)
    },

    addStory: async (projectId, story) => {
      const currentStories = get().getProjectStories(projectId)
      const updatedStories = [...currentStories, story]
      await get().setProjectStories(projectId, updatedStories)
    },

    updateStory: async (projectId, storyId, updates) => {
      const currentStories = get().getProjectStories(projectId)
      const updatedStories = currentStories.map(s =>
        s.id === storyId ? { ...s, ...updates, updatedAt: new Date().toISOString() } : s
      )

      await get().setProjectStories(projectId, updatedStories)
    },

    deleteStory: async (projectId, storyId) => {
      const currentStories = get().getProjectStories(projectId)
      const updatedStories = currentStories.filter(s => s.id !== storyId)
      await get().setProjectStories(projectId, updatedStories)
    },

    clearProjectStories: projectId => {
      set(state => {
        delete state.storiesByProject[projectId]
      })
    },
  }))
)
