import { useState } from 'react'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import type { PRD } from '@/types'

export function usePRDExport() {
  const [isExporting, setIsExporting] = useState(false)
  const [exportProgress, setExportProgress] = useState(0)
  const [showExportDialog, setShowExportDialog] = useState(false)
  const [exportStatus, setExportStatus] = useState<'success' | 'error'>('success')
  const [exportMessage, setExportMessage] = useState('')

  const handleExport = async (prd: PRD | null, editedMarkdown: string) => {
    if (!prd && !editedMarkdown) {
      alert('暂无可导出的内容，请先生成产品需求文档')
      return
    }

    setIsExporting(true)
    setExportProgress(0)
    setShowExportDialog(true)
    setExportStatus('success')

    try {
      // 步骤 1: 准备内容 (进度 10%)
      setExportProgress(10)
      await new Promise(resolve => setTimeout(resolve, 300))

      // 优先使用完整的 Markdown 内容，如果没有则从结构化数据生成
      const content =
        editedMarkdown ||
        prd?.markdownContent ||
        `# ${prd?.title}\n\n## 产品概述\n\n${prd?.overview}\n\n## 目标用户\n\n${prd?.targetUsers.map(u => `- ${u}`).join('\n')}\n\n## 核心功能\n\n${prd?.coreFeatures.map(f => `- ${f}`).join('\n')}\n\n## 技术栈\n\n${prd?.techStack.map(t => `- ${t}`).join('\n')}\n\n## 预估工作量\n\n${prd?.estimatedEffort}\n\n## 商业模式\n\n${prd?.businessModel || '待定'}\n\n## 定价策略\n\n${prd?.pricing || '待定'}`

      // 生成默认文件名
      const defaultFilename = `${prd?.title || editedMarkdown ? 'PRD' : '产品需求文档'}-PRD.md`

      console.log('[PRD Export] Starting export...')
      console.log('[PRD Export] Default filename:', defaultFilename)
      console.log('[PRD Export] Content length:', content.length)

      // 步骤 2: 打开保存对话框 (进度 30%)
      setExportProgress(30)
      await new Promise(resolve => setTimeout(resolve, 400))

      const filePath = await save({
        defaultPath: defaultFilename,
        filters: [
          {
            name: 'Markdown Files',
            extensions: ['md'],
          },
        ],
      })

      console.log('[PRD Export] Selected path:', filePath)

      if (filePath) {
        // 步骤 3: 写入文件 (进度 50% -> 90%)
        setExportProgress(50)
        await new Promise(resolve => setTimeout(resolve, 300))

        await writeTextFile(filePath, content)

        setExportProgress(90)
        await new Promise(resolve => setTimeout(resolve, 300))

        setExportProgress(100)
        setExportMessage(`文件已成功保存到：${filePath}`)
        console.log('[PRD Export] File saved successfully to:', filePath)

        // 延迟关闭对话框
        setTimeout(() => {
          setShowExportDialog(false)
          setIsExporting(false)
        }, 1500)
      } else {
        // 用户取消
        setExportMessage('已取消导出')
        setExportProgress(0)
        setTimeout(() => {
          setShowExportDialog(false)
          setIsExporting(false)
        }, 800)
      }
    } catch (error) {
      console.error('[PRD Export] Error during export:', error)
      setExportStatus('error')
      setExportMessage(`导出失败：${error instanceof Error ? error.message : error}`)
      setExportProgress(0)
      // 错误时不自动关闭对话框，让用户手动关闭
      setIsExporting(false)
    }
  }

  return {
    isExporting,
    exportProgress,
    showExportDialog,
    exportStatus,
    exportMessage,
    setShowExportDialog,
    handleExport,
  }
}
