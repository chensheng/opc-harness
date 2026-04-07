import { useEffect, useState, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { Download, Edit } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useProjectStore, useAppStore } from '@/stores'
import type { PRD } from '@/types'
import { convertPRDToMarkdown } from './PRDDisplayUtils'
import { ExportDialog } from './PRDDisplayExportDialog'
import { SaveDialog } from './PRDDisplaySaveDialog'
import { PRDDEditor } from './PRDDisplayEditor'
import { PRDDisplayStreamingView } from './PRDDisplayStreamingView'
import {
  FullDocTab,
  OverviewTab,
  FeaturesTab,
  TechTab,
  BusinessTab,
} from './PRDDisplayTabs'
import { usePRDExport } from './usePRDExport'
import { usePRDSave } from './usePRDSave'
import { usePRDGeneration } from './usePRDGeneration'

export function PRDDisplay() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectPRD, updateProjectStatus, updateProjectProgress, syncProjectToDatabase } =
    useProjectStore()
  const { setLoading } = useAppStore()

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
    isComplete,
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
    getProjectById,
    syncProjectToDatabase,
    setLoading,
    setIsEditing,
  })

  useEffect(() => {
    if (project && !hasStartedGenerationRef.current) {
      if (project.prd) {
        setPrd(project.prd)
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
      const markdownContent = prd.markdownContent || convertPRDToMarkdown(prd)
      setEditedMarkdown(markdownContent)
      setIsEditing(true)
      setPreviewMode('split') // 默认分屏模式
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
          <h1 className="text-2xl font-bold">📋 产品需求文档</h1>
          <p className="text-muted-foreground">{project.name}</p>
        </div>
        <div className="flex gap-2">
          {!isEditing && (
            <>
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
              <Button onClick={() => handleSaveEdit(editedMarkdown, setPrd)}>
                保存
              </Button>
            </>
          )}
        </div>
      </div>

      {/* 编辑模式 - Markdown 编辑器 */}
      {isEditing && editedMarkdown ? (
        <PRDDEditor
          editedMarkdown={editedMarkdown}
          onMarkdownChange={setEditedMarkdown}
          previewMode={previewMode}
          onPreviewModeChange={setPreviewMode}
          onCancel={handleCancelEdit}
          onSave={() => handleSaveEdit(editedMarkdown, setPrd)}
        />
      ) : (
        <Tabs defaultValue="full" className="w-full">
          <TabsList className="grid w-full grid-cols-5">
            <TabsTrigger value="full">完整文档</TabsTrigger>
            <TabsTrigger value="overview">概述</TabsTrigger>
            <TabsTrigger value="features">功能</TabsTrigger>
            <TabsTrigger value="tech">技术</TabsTrigger>
            <TabsTrigger value="business">商业</TabsTrigger>
          </TabsList>

          {/* 完整文档视图 */}
          <TabsContent value="full">
            <FullDocTab prd={prd} />
          </TabsContent>

          {/* 概述视图 */}
          <TabsContent value="overview">
            <OverviewTab prd={prd} />
          </TabsContent>

          {/* 功能视图 */}
          <TabsContent value="features">
            <FeaturesTab prd={prd} />
          </TabsContent>

          {/* 技术视图 */}
          <TabsContent value="tech">
            <TechTab prd={prd} />
          </TabsContent>

          {/* 商业视图 */}
          <TabsContent value="business">
            <BusinessTab prd={prd} />
          </TabsContent>
        </Tabs>
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
    </div>
  )
}
