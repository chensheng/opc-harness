import { useState } from 'react';
import { Sparkles, ChevronRight } from 'lucide-react';
import { PRDDisplay } from './PRDDisplay';
import { usePRDStream } from '../hooks/usePRDStream';
import { savePRD as savePRDApi, exportPRDToMarkdown as exportPRDApi } from '@/api';

export function IdeaInput() {
  const [idea, setIdea] = useState('');
  const [error, setError] = useState<string | null>(null);
  // 使用 PRD 流式生成 hook
  const {
    isGenerating,
    prd,
    progress,
    generatePRD: streamGeneratePRD,
    reset: resetStream,
  } = usePRDStream();

  const handleGeneratePRD = async () => {
    if (!idea.trim()) return;

    setError(null);
    resetStream();

    try {
      // 使用流式生成
      await streamGeneratePRD(idea);

      // 生成完成后设置状态
    } catch (err) {
      setError(err instanceof Error ? err.message : '生成 PRD 失败');
    }
  };

  const handleSave = async () => {
    if (!prd || !idea.trim()) return;

    try {
      // TODO: 需要创建项目后才能保存，这里先使用一个临时项目 ID
      // VD-021: 实际应该先创建项目，然后关联 PRD
      const tempProjectId = 'temp-project-' + new Date().toISOString().split('T')[0];

      await savePRDApi(tempProjectId, prd);

      alert('✅ PRD 已成功保存到数据库和本地文件！');
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '保存 PRD 失败';
      console.error('Failed to save PRD:', err);
      alert(`❌ 保存失败：${errorMessage}`);
    }
  };

  const handleExport = async () => {
    if (!prd || !idea.trim()) return;

    try {
      // 生成文件名：使用创意的前 20 个字符作为文件名的一部分
      const ideaPreview = idea
        .trim()
        .slice(0, 20)
        .replace(/[^\w\u4e00-\u9fa5]/g, '-');
      const filename = `PRD-${ideaPreview}-${new Date().toISOString().split('T')[0]}.md`;

      // 调用后端 API 导出文件
      const filePath = await exportPRDApi('temp-project', prd, filename);

      alert(`✅ PRD 已成功导出到：\n${filePath}`);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '导出 PRD 失败';
      console.error('Failed to export PRD:', err);
      alert(`❌ 导出失败：${errorMessage}`);
    }
  };

  return (
    <div className="space-y-6">
      {/* 想法输入区域 */}
      <div className="bg-white rounded-2xl shadow-sm border border-slate-200 p-6">
        <h2 className="text-2xl font-bold mb-4 flex items-center gap-2">
          <Sparkles className="w-6 h-6 text-blue-500" />
          输入产品想法
        </h2>

        <textarea
          value={idea}
          onChange={e => setIdea(e.target.value)}
          placeholder="用一两句话描述你的产品想法，例如：&#10;我想做一个帮助独立开发者快速生成产品需求文档的 AI 工具..."
          className="w-full h-32 px-4 py-3 border border-slate-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
        />

        <div className="mt-4 flex items-center justify-between">
          <p className="text-sm text-slate-500">AI 将根据你的想法生成完整的产品需求文档</p>
          <button
            onClick={handleGeneratePRD}
            disabled={!idea.trim() || isGenerating}
            className={`px-6 py-3 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-xl font-medium transition-all ${
              !idea.trim() || isGenerating
                ? 'opacity-50 cursor-not-allowed'
                : 'hover:shadow-lg hover:scale-105'
            }`}
          >
            {isGenerating ? (
              <span className="flex items-center gap-2">
                <Sparkles className="w-5 h-5 animate-spin" />
                生成中...
              </span>
            ) : (
              <span className="flex items-center gap-2">
                生成 PRD
                <ChevronRight className="w-5 h-5" />
              </span>
            )}
          </button>
        </div>

        {error && (
          <div className="mt-4 p-4 bg-red-50 border border-red-200 rounded-xl text-red-700">
            {error}
          </div>
        )}
      </div>

      {/* PRD 展示区域 */}
      {/* 展示 PRD 内容 */}
      {(isGenerating || prd) && (
        <PRDDisplay
          prd={prd}
          idea={idea}
          generatedAt={new Date().toISOString()}
          onSave={handleSave}
          onExport={handleExport}
          isStreaming={isGenerating}
          progress={progress}
        />
      )}
    </div>
  );
}
