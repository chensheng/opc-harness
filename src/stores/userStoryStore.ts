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
        console.log('\n' + '='.repeat(80))
        console.log('📥 [UserStoryStore] loadProjectStories CALLED')
        console.log('   Timestamp:', new Date().toISOString())
        console.log(`   Project ID: ${projectId}`)
        console.log('='.repeat(80))

        set({ isLoading: true })

        console.log('   Invoking Rust backend get_user_stories command...')
        const response = await invoke<{ success: boolean; user_stories: BackendUserStory[] }>(
          'get_user_stories',
          {
            request: { project_id: projectId },
          }
        )

        console.log('\n📥 [UserStoryStore] Received response from Rust backend')
        console.log('   Response success:', response.success)
        console.log('   Response user_stories count:', response.user_stories?.length || 0)
        if (response.user_stories && response.user_stories.length > 0) {
          console.log('   First loaded story:', {
            id: response.user_stories[0].id,
            storyNumber: response.user_stories[0].story_number,
            title: response.user_stories[0].title,
          })
        }

        if (response.success) {
          console.log('   Converting stories from snake_case to camelCase...')
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
            labels: story.labels,
            createdAt: story.created_at,
            updatedAt: story.updated_at,
          })) as UserStory[]

          console.log('   Updating local store state...')
          set(state => {
            state.storiesByProject[projectId] = frontendStories
          })
          console.log('   Local state updated')
          console.log(
            `✅ [UserStoryStore] Loaded ${frontendStories.length} stories from database for project ${projectId}`
          )
          if (frontendStories.length > 0) {
            console.log(
              `   First story: ${frontendStories[0].storyNumber} - ${frontendStories[0].title}`
            )
          }
        } else {
          console.warn('⚠️ [UserStoryStore] Load response not successful')
        }
        console.log('='.repeat(80) + '\n')
      } catch (error) {
        console.error('\n❌ [UserStoryStore] loadProjectStories FAILED')
        console.error('   Error:', error)
        console.error('   Error message:', error instanceof Error ? error.message : String(error))
        console.error('='.repeat(80) + '\n')
      } finally {
        set({ isLoading: false })
      }
    },

    setProjectStories: async (projectId, stories) => {
      try {
        console.log('\n' + '='.repeat(80))
        console.log('💾 [UserStoryStore] setProjectStories CALLED')
        console.log('   Timestamp:', new Date().toISOString())
        console.log(`   Project ID: ${projectId}`)
        console.log(`   Stories count: ${stories.length}`)
        if (stories.length > 0) {
          console.log('   First story to save:', {
            id: stories[0].id,
            storyNumber: stories[0].storyNumber,
            title: stories[0].title,
          })
        }
        console.log('='.repeat(80))

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
          labels: story.labels,
          created_at: story.createdAt,
          updated_at: story.updatedAt,
        }))

        console.log('   Converted stories to snake_case format for Rust backend')
        console.log('   Invoking Rust backend save_user_stories command...')
        const response = await invoke<{ success: boolean; count: number }>('save_user_stories', {
          request: {
            project_id: projectId,
            user_stories: rustStories,
          },
        })

        console.log('\n📥 [UserStoryStore] Received response from Rust backend')
        console.log('   Response:', response)

        if (response.success) {
          console.log(
            `✅ [UserStoryStore] Backend reported success: saved ${response.count} stories`
          )
        } else {
          console.error(`❌ [UserStoryStore] Backend reported failure`)
        }

        // 更新本地状态
        console.log('   Updating local store state...')
        set(state => {
          state.storiesByProject[projectId] = stories
        })
        console.log('   Local state updated')
        console.log('='.repeat(80) + '\n')
      } catch (error) {
        console.error('\n❌ [UserStoryStore] setProjectStories FAILED')
        console.error('   Error:', error)
        console.error('   Error message:', error instanceof Error ? error.message : String(error))
        console.error('='.repeat(80) + '\n')
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
