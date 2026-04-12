import { useEffect, useState, useRef } from 'react'
import { useParams, useNavigate, useLocation } from 'react-router-dom'
import { Download, Edit, ArrowRight, Code } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { ScrollToTop } from '@/components/ui/ScrollToTop'
import { useProjectStore, useAppStore } from '@/stores'
import type { PRD } from '@/types'
import { ExportDialog } from './PRDDisplayExportDialog'
import { SaveDialog } from './PRDDisplaySaveDialog'
import { PRDDEditor } from './PRDDisplayEditor'
import { PRDDisplayStreamingView } from './PRDDisplayStreamingView'
import { FullDocTab } from './PRDDisplayTabs'
import { usePRDExport } from './usePRDExport'
import { usePRDSave } from './usePRDSave'
import { usePRDGeneration } from './usePRDGeneration'
import { parseMarkdownToPRD } from './PRDDisplayUtils'
import { ProjectListFloatingButton } from './ProjectListFloatingButton'

export function PRDDisplay() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const location = useLocation()
  const {
    getProjectById,
    setProjectPRD,
    updateProjectStatus,
    updateProjectProgress,
    syncProjectToDatabase,
  } = useProjectStore()
  const { setLoading } = useAppStore()

  // 解析 URL 参数
  const urlParams = new URLSearchParams(location.search)

  const [prd, setPrd] = useState<PRD | null>(null)
  const [isEditing, setIsEditing] = useState(false)
  const [editedMarkdown, setEditedMarkdown] = useState<string>('')
  const [previewMode, setPreviewMode] = useState<'edit' | 'preview' | 'split'>('split')

  // 防止重复生成 PRD 的标志
  const hasStartedGenerationRef = useRef(false)

  const project = projectId ? getProjectById(projectId) : undefined

  // 使用 PRD 生成 Hook
  const {
    streamingPRD,
    markdownContent,
    isStreaming,
    isComplete: _isComplete,
    error,
    generatePRD,
    handleStopGeneration,
  } = usePRDGeneration({
    projectId,
    projectIdea: project?.idea || project?.description || '',
    setProjectPRD,
    updateProjectStatus,
    updateProjectProgress,
    syncProjectToDatabase,
    setLoading,
    urlParams, // 传递 URL 参数
  })

  // 使用导出 Hook
  const {
    isExporting,
    exportProgress,
    showExportDialog,
    exportStatus,
    exportMessage,
    setShowExportDialog,
    handleExport,
  } = usePRDExport()

  // 使用保存 Hook
  const {
    isSaving,
    saveProgress,
    showSaveDialog,
    saveStatus,
    saveMessage,
    setShowSaveDialog,
    handleSaveEdit,
  } = usePRDSave({
    projectId,
    setProjectPRD,
    syncProjectToDatabase,
    setLoading,
    setIsEditing,
  })

  useEffect(() => {
    if (project && !hasStartedGenerationRef.current) {
      if (project.prd) {
        // 如果已有结构化的 PRD 对象，直接使用
        setPrd(project.prd)
      } else if (project.prdMarkdown) {
        // 如果只有原始 Markdown，解析为结构化 PRD
        try {
          const parsedPRD = parseMarkdownToPRD(project.prdMarkdown)
          // 添加完整的 Markdown 内容
          parsedPRD.markdownContent = project.prdMarkdown
          setPrd(parsedPRD)
        } catch (error) {
          console.error('[PRDDisplay] Failed to parse PRD from markdown:', error)
          hasStartedGenerationRef.current = true
          generatePRD()
        }
      } else {
        // Generate PRD if not exists
        hasStartedGenerationRef.current = true
        generatePRD()
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  // 当流式 PRD 更新时，同步到本地状态
  useEffect(() => {
    if (streamingPRD && isStreaming) {
      setPrd(streamingPRD)
    }
  }, [streamingPRD, isStreaming])

  const handleCancelEdit = () => {
    setIsEditing(false)
    setEditedMarkdown('')
  }

  const startEditing = () => {
    if (prd) {
      // 优先使用保存的完整 Markdown 内容，如果没有则从结构化数据转换
      const markdownContent = prd.markdownContent || ''
      setEditedMarkdown(markdownContent)
      setIsEditing(true)
      setPreviewMode('split') // 默认分屏模式
    }
  }

  // 为 AI 优化助手创建简化版保存函数（不显示对话框）
  const handleSaveToDatabase = async (content: string) => {
    if (!content || !projectId) return

    try {
      // 解析 Markdown 内容
      const updatedPrd = parseMarkdownToPRD(content)
      updatedPrd.markdownContent = content

      // 更新本地状态
      setProjectPRD(projectId, updatedPrd)
      setPrd(updatedPrd)

      // 同步到数据库
      await syncProjectToDatabase(projectId)
    } catch (error) {
      console.error('[AI Optimization Save] Failed to save:', error)
      throw error
    }
  }

  if (!project) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">项目不存在</p>
        <Button onClick={() => navigate('/')} className="mt-4">
          返回首页
        </Button>
      </div>
    )
  }

  // 流式生成中的 UI
  if (isStreaming) {
    return (
      <PRDDisplayStreamingView
        markdownContent={markdownContent}
        error={error}
        onStopGeneration={handleStopGeneration}
        onRetry={generatePRD}
      />
    )
  }

  // 如果有错误但没有 PRD，显示错误信息
  if (error && !prd) {
    return (
      <div className="max-w-4xl mx-auto space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold">PRD 生成失败</h1>
            <p className="text-muted-foreground mt-1">无法生成产品需求文档</p>
          </div>
        </div>

        <Card className="border-destructive">
          <CardContent className="py-6">
            <div className="space-y-4">
              <p className="text-destructive font-medium">{error}</p>
              <div className="flex gap-2">
                <Button onClick={generatePRD}>重试</Button>
                <Button variant="outline" onClick={() => navigate('/')}>
                  返回首页
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    )
  }

  // 没有 PRD 时的初始状态
  if (!prd) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
          <p className="mt-4 text-muted-foreground">正在生成 PRD...</p>
        </div>
      </div>
    )
  }

  // PRD 已生成的正常显示
  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">{project.name}</h1>
          <p className="text-muted-foreground">产品需求文档</p>
        </div>
        <div className="flex gap-2">
          {!isEditing && (
            <>
              <Button variant="default" onClick={() => navigate(`/coding/${projectId}`)}>
                <Code className="w-4 h-4 mr-2" />
                开始开发
                <ArrowRight className="w-4 h-4 ml-2" />
              </Button>
              <Button variant="outline" onClick={() => handleExport(prd, editedMarkdown)}>
                <Download className="w-4 h-4 mr-2" />
                导出
              </Button>
              <Button variant="outline" onClick={startEditing}>
                <Edit className="w-4 h-4 mr-2" />
                编辑
              </Button>
            </>
          )}
          {isEditing && (
            <>
              <Button variant="outline" onClick={handleCancelEdit}>
                取消
              </Button>
              <Button onClick={() => handleSaveEdit(editedMarkdown, setPrd)}>保存</Button>
            </>
          )}
        </div>
      </div>

      {/* 编辑模式 - Markdown 编辑器 */}
      {isEditing && editedMarkdown ? (
        <PRDDEditor
          projectId={projectId || ''}
          editedMarkdown={editedMarkdown}
          onMarkdownChange={setEditedMarkdown}
          previewMode={previewMode}
          onPreviewModeChange={setPreviewMode}
          onCancel={handleCancelEdit}
          onSave={() => handleSaveEdit(editedMarkdown, setPrd)}
          onSaveToDatabase={handleSaveToDatabase}
        />
      ) : (
        <FullDocTab prd={prd} />
      )}

      {/* 保存进度对话框 */}
      <SaveDialog
        open={showSaveDialog}
        onOpenChange={setShowSaveDialog}
        isSaving={isSaving}
        saveProgress={saveProgress}
        saveStatus={saveStatus}
        saveMessage={saveMessage}
        onSave={() => handleSaveEdit(editedMarkdown, setPrd)}
      />

      {/* 导出进度对话框 */}
      <ExportDialog
        open={showExportDialog}
        onOpenChange={setShowExportDialog}
        isExporting={isExporting}
        exportProgress={exportProgress}
        exportStatus={exportStatus}
        exportMessage={exportMessage}
        onRetry={() => handleExport(prd, editedMarkdown)}
      />

      {/* 回到顶部按钮 */}
      <ScrollToTop />

      {/* 项目列表悬浮按钮 */}
      <ProjectListFloatingButton />
    </div>
  )
}
