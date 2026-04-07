import { useState } from 'react'
import { parseMarkdownToPRD } from './PRDDisplayUtils'
import type { PRD } from '@/types'

interface UsePRDSaveParams {
  projectId: string | undefined
  setProjectPRD: (projectId: string, prd: PRD) => void
  getProjectById: (projectId: string) => any
  syncProjectToDatabase: (projectId: string) => Promise<void>
  setLoading: (loading: boolean, message?: string) => void
  setIsEditing: (editing: boolean) => void
}

export function usePRDSave({
  projectId,
  setProjectPRD,
  getProjectById,
  syncProjectToDatabase,
  setLoading,
  setIsEditing,
}: UsePRDSaveParams) {
  const [isSaving, setIsSaving] = useState(false)
  const [saveProgress, setSaveProgress] = useState(0)
  const [showSaveDialog, setShowSaveDialog] = useState(false)
  const [saveStatus, setSaveStatus] = useState<'success' | 'error'>('success')
  const [saveMessage, setSaveMessage] = useState('')

  const handleSaveEdit = async (editedMarkdown: string, setPrd: (prd: PRD) => void) => {
    if (!editedMarkdown || !projectId) return
    
    console.log('[PRD Save] Starting save process...')
    console.log('[PRD Save] Edited markdown length:', editedMarkdown.length)
    
    // 显示保存对话框并开始进度
    setIsSaving(true)
    setSaveProgress(0)
    setShowSaveDialog(true)
    setSaveStatus('success')
    setSaveMessage('正在保存产品需求文档...')
    
    try {
      // 步骤 1: 解析 Markdown 内容 (进度 20%)
      setSaveProgress(20)
      await new Promise(resolve => setTimeout(resolve, 200))
      
      const updatedPrd = parseMarkdownToPRD(editedMarkdown)
      
      // 添加完整的 Markdown 内容到 PRD 对象
      updatedPrd.markdownContent = editedMarkdown
      
      console.log('[PRD Save] Parsed PRD object:', updatedPrd)
      
      // 步骤 2: 更新本地状态 (进度 40%)
      setSaveProgress(40)
      await new Promise(resolve => setTimeout(resolve, 200))
      
      setProjectPRD(projectId, updatedPrd)
      setPrd(updatedPrd)
      
      // 步骤 3: 验证保存是否成功 (进度 60%)
      setSaveProgress(60)
      await new Promise(resolve => setTimeout(resolve, 200))
      
      const savedProject = getProjectById(projectId)
      console.log('[PRD Save] Saved project PRD:', savedProject?.prd)
      
      // 步骤 4: 同步到数据库 (进度 80%)
      setSaveProgress(80)
      setSaveMessage('正在同步到数据库...')
      await syncProjectToDatabase(projectId)
      console.log('[PRD Save] Successfully synced to database')
      
      // 步骤 5: 完成 (进度 100%)
      setSaveProgress(100)
      setSaveStatus('success')
      setSaveMessage('✅ 产品需求文档已成功保存！')
      
      // 延迟关闭对话框
      setTimeout(() => {
        setShowSaveDialog(false)
        setIsSaving(false)
        setIsEditing(false)
        setLoading(false)
      }, 1500)
      
    } catch (error) {
      console.error('[PRD Save] Failed to save:', error)
      setSaveStatus('error')
      setSaveMessage(`❌ 保存失败：${error instanceof Error ? error.message : '未知错误'}`)
      setSaveProgress(0)
      setIsSaving(false)
      // 错误时不自动关闭对话框，让用户手动关闭
    }
  }

  return {
    isSaving,
    saveProgress,
    showSaveDialog,
    saveStatus,
    saveMessage,
    setShowSaveDialog,
    handleSaveEdit,
  }
}
