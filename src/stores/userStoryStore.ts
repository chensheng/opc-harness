import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { invoke } from '@tauri-apps/api/core'
import type { UserStory } from '@/types'

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
        console.log(`[UserStoryStore] 📥 Loading stories for project ${projectId}...`)

        const response = await invoke<{ success: boolean; userStories: UserStory[] }>(
          'get_user_stories',
          {
            request: { projectId },
          }
        )

        if (response.success) {
          set(state => {
            state.storiesByProject[projectId] = response.userStories
          })
          console.log(
            `✅ [UserStoryStore] Loaded ${response.userStories.length} stories from database for project ${projectId}`
          )
          if (response.userStories.length > 0) {
            console.log(
              `   First story: ${response.userStories[0].storyNumber} - ${response.userStories[0].title}`
            )
          }
        } else {
          console.warn('⚠️ [UserStoryStore] Load response not successful')
        }
      } catch (error) {
        console.error('❌ [UserStoryStore] Failed to load stories:', error)
      } finally {
        set({ isLoading: false })
      }
    },

    setProjectStories: async (projectId, stories) => {
      try {
        set({ isLoading: true })
        console.log(
          `💾 [UserStoryStore] Saving ${stories.length} stories for project ${projectId}...`
        )
        if (stories.length > 0) {
          console.log(`   First story: ${stories[0].storyNumber} - ${stories[0].title}`)
        }

        // 保存到数据库
        const response = await invoke<{ success: boolean; count: number }>('save_user_stories', {
          request: {
            projectId,
            userStories: stories,
          },
        })

        if (response.success) {
          console.log(`✅ [UserStoryStore] Saved ${response.count} stories to database`)
        } else {
          console.error(`❌ [UserStoryStore] Save failed`)
        }

        // 更新本地状态
        set(state => {
          state.storiesByProject[projectId] = stories
        })

        console.log(`🔄 [UserStoryStore] Updated local store with ${stories.length} stories`)
      } catch (error) {
        console.error('❌ [UserStoryStore] Failed to save stories:', error)
        throw error
      } finally {
        set({ isLoading: false })
      }
    },

    getProjectStories: projectId => {
      return get().storiesByProject[projectId] || []
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
