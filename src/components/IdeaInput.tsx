import { useState } from 'react';
import { Sparkles, Save, Download, ChevronRight } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface PRDResult {
  prd: string;
  idea: string;
  generatedAt: string;
}

export function IdeaInput() {
  const [idea, setIdea] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [prdResult, setPrdResult] = useState<PRDResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleGeneratePRD = async () => {
    if (!idea.trim()) return;

    setIsGenerating(true);
    setError(null);

    try {
      // TODO: 调用 Tauri command 生成 PRD
      // const result = await invoke('generate_prd', { idea });

      // 模拟生成
      await new Promise(resolve => setTimeout(resolve, 2000));
      const mockPRD = `# 产品需求文档 - ${idea.substring(0, 30)}...

## 1. 产品概述
这是一个基于${idea}的产品，旨在解决用户的核心痛点。

## 2. 目标用户
- 主要用户群体：需要${idea}的用户
- 次要用户群体：相关领域的从业者

## 3. 核心功能
### 3.1 功能一
描述第一个核心功能

### 3.2 功能二
描述第二个核心功能

## 4. 技术架构
- 前端：React + TypeScript
- 后端：Tauri + Rust
- 数据库：SQLite

## 5. 开发计划
- Phase 1: MVP (2-3 周)
- Phase 2: 功能完善 (4-6 周)
- Phase 3: 商业化 (8-10 周)

## 6. 风险评估
- 技术风险：中等
- 市场风险：低
- 竞争风险：中等`;

      setPrdResult({
        prd: mockPRD,
        idea,
        generatedAt: new Date().toISOString(),
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : '生成 PRD 失败');
    } finally {
      setIsGenerating(false);
    }
  };

  const handleSave = async () => {
    if (!prdResult) return;
    // TODO: 保存到数据库
    alert('PRD 已保存（功能待实现）');
  };

  const handleExport = () => {
    if (!prdResult) return;
    const blob = new Blob([prdResult.prd], { type: 'text/markdown' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `PRD-${new Date().toISOString().split('T')[0]}.md`;
    a.click();
    URL.revokeObjectURL(url);
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
      {prdResult && (
        <div className="bg-white rounded-2xl shadow-sm border border-slate-200 p-6 animate-fade-in">
          <div className="flex items-center justify-between mb-6">
            <h3 className="text-xl font-bold">生成的产品需求文档</h3>
            <div className="flex gap-3">
              <button
                onClick={handleSave}
                className="px-4 py-2 text-sm text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors flex items-center gap-2"
              >
                <Save className="w-4 h-4" />
                保存
              </button>
              <button
                onClick={handleExport}
                className="px-4 py-2 text-sm text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors flex items-center gap-2"
              >
                <Download className="w-4 h-4" />
                导出 Markdown
              </button>
            </div>
          </div>

          <div className="prose prose-slate max-w-none">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{prdResult.prd}</ReactMarkdown>
          </div>

          <div className="mt-6 pt-6 border-t border-slate-200">
            <p className="text-sm text-slate-500">
              生成时间：{new Date(prdResult.generatedAt).toLocaleString('zh-CN')}
            </p>
          </div>
        </div>
      )}
    </div>
  );
}
