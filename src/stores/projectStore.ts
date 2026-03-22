import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { immer } from 'zustand/middleware/immer'
import type { Project, PRD, UserPersona, CompetitorAnalysis } from '@/types'

interface ProjectState {
  projects: Project[]
  currentProjectId: string | null
}

interface ProjectActions {
  createProject: (name: string, description: string, idea?: string) => Project
  updateProject: (id: string, updates: Partial<Project>) => void
  deleteProject: (id: string) => void
  setCurrentProject: (id: string | null) => void
  getCurrentProject: () => Project | undefined
  getProjectById: (id: string) => Project | undefined
  setProjectPRD: (id: string, prd: PRD) => void
  setProjectPersonas: (id: string, personas: UserPersona[]) => void
  setProjectCompetitorAnalysis: (id: string, analysis: CompetitorAnalysis) => void
  updateProjectProgress: (id: string, progress: number) => void
  updateProjectStatus: (id: string, status: Project['status']) => void
}

export const useProjectStore = create<ProjectState & ProjectActions>()(
  immer(
    persist(
      (set, get) => ({
        projects: [],
        currentProjectId: null,

        createProject: (name, description, idea) => {
          const project: Project = {
            id: crypto.randomUUID(),
            name,
            description,
            status: 'idea',
            progress: 0,
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            idea,
          }
          set(state => {
            state.projects.push(project)
            state.currentProjectId = project.id
          })
          return project
        },

        updateProject: (id, updates) =>
          set(state => {
            const project = state.projects.find(p => p.id === id)
            if (project) {
              Object.assign(project, updates, { updatedAt: new Date().toISOString() })
            }
          }),

        deleteProject: id =>
          set(state => {
            state.projects = state.projects.filter(p => p.id !== id)
            if (state.currentProjectId === id) {
              state.currentProjectId = null
            }
          }),

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

        setProjectPRD: (id, prd) =>
          set(state => {
            const project = state.projects.find(p => p.id === id)
            if (project) {
              project.prd = prd
              project.updatedAt = new Date().toISOString()
            }
          }),

        setProjectPersonas: (id, personas) =>
          set(state => {
            const project = state.projects.find(p => p.id === id)
            if (project) {
              project.userPersonas = personas
              project.updatedAt = new Date().toISOString()
            }
          }),

        setProjectCompetitorAnalysis: (id, analysis) =>
          set(state => {
            const project = state.projects.find(p => p.id === id)
            if (project) {
              project.competitorAnalysis = analysis
              project.updatedAt = new Date().toISOString()
            }
          }),

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
      }),
      {
        name: 'opc-harness-projects',
      }
    )
  )
)
