import { useState } from 'react';
import {
  Users,
  Save,
  Download,
  Copy,
  Check,
  ZoomIn,
  ZoomOut,
  Maximize2,
  Minimize2,
  Printer,
  Loader2,
  User,
  Briefcase,
  DollarSign,
  GraduationCap,
  Target,
  Lightbulb,
  MessageSquare,
  TrendingUp,
} from 'lucide-react';
import type { UserPersona } from '@/types';

interface UserPersonasProps {
  personas: UserPersona[];
  idea?: string;
  generatedAt?: string;
  onSave?: () => void;
  onExport?: () => void;
  isStreaming?: boolean;
  progress?: number;
}

export function UserPersonas({
  personas,
  idea,
  generatedAt,
  onSave,
  onExport,
  isStreaming = false,
  progress = 0,
}: UserPersonasProps) {
  const [zoom, setZoom] = useState(100);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [copied, setCopied] = useState(false);
  const [selectedPersonaIndex, setSelectedPersonaIndex] = useState(0);

  const handleCopy = async () => {
    const text = JSON.stringify(personas, null, 2);
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handlePrint = () => {
    const printWindow = window.open('', '_blank');
    if (printWindow) {
      const personaHtml = personas
        .map(
          (p, i) => `
        <div style="margin-bottom: 30px; page-break-inside: avoid;">
          <h2 style="color: #1e40af; border-bottom: 2px solid #3b82f6; padding-bottom: 10px;">
            ${p.name || `用户画像 ${i + 1}`}
          </h2>
          ${p.age ? `<p><strong>年龄:</strong> ${p.age}</p>` : ''}
          ${p.occupation ? `<p><strong>职业:</strong> ${p.occupation}</p>` : ''}
          ${p.city ? `<p><strong>城市:</strong> ${p.city}</p>` : ''}
          ${p.incomeLevel ? `<p><strong>收入水平:</strong> ${p.incomeLevel}</p>` : ''}
          ${p.coreNeeds ? `<p><strong>核心需求:</strong> ${p.coreNeeds}</p>` : ''}
          ${p.usageScenarios ? `<p><strong>使用场景:</strong> ${p.usageScenarios}</p>` : ''}
          ${p.quote ? `<blockquote style="border-left: 4px solid #3b82f6; padding-left: 16px; color: #6b7280; margin: 20px 0;">"${p.quote}"</blockquote>` : ''}
        </div>
      `
        )
        .join('');

      printWindow.document.write(`
        <html>
          <head>
            <title>用户画像</title>
            <style>
              body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 40px; }
              h1 { color: #1e40af; border-bottom: 2px solid #3b82f6; padding-bottom: 10px; }
              h2 { color: #1e3a8a; margin-top: 30px; }
              p { line-height: 1.6; }
              blockquote { border-left: 4px solid #3b82f6; padding-left: 16px; color: #6b7280; }
            </style>
          </head>
          <body>
            <h1>用户画像 (${personas.length}个)</h1>
            ${idea ? `<p style="color: #6b7280; margin-bottom: 30px;"><strong>产品创意:</strong> ${idea}</p>` : ''}
            ${personaHtml}
          </body>
        </html>
      `);
      printWindow.document.close();
      printWindow.print();
    }
  };

  const handleExport = () => {
    const blob = new Blob([JSON.stringify(personas, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `user-personas-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
    onExport?.();
  };

  const containerClass = isFullscreen
    ? 'fixed inset-0 z-50 bg-white p-8 overflow-auto'
    : 'bg-white rounded-2xl shadow-sm border border-slate-200 p-6 animate-fade-in';

  // 渲染单个画像卡片
  const renderPersonaCard = (persona: UserPersona | undefined, index: number) => {
    if (!persona) return null;

    const isSelected = index === selectedPersonaIndex;

    return (
      <div
        key={index}
        className={`rounded-xl border-2 transition-all duration-200 cursor-pointer ${
          isSelected
            ? 'border-blue-500 shadow-lg bg-blue-50'
            : 'border-slate-200 hover:border-blue-300 hover:shadow-md'
        }`}
        onClick={() => setSelectedPersonaIndex(index)}
      >
        {/* 卡片头部 */}
        <div className="bg-gradient-to-r from-blue-500 to-indigo-600 text-white p-4 rounded-t-xl">
          <div className="flex items-center gap-3">
            <div className="w-12 h-12 rounded-full bg-white/20 flex items-center justify-center">
              <User className="w-6 h-6" />
            </div>
            <div>
              <h3 className="text-xl font-bold">{persona.name || `用户画像 ${index + 1}`}</h3>
              <p className="text-sm text-blue-100">
                {persona.occupation || '职业未指定'}
                {persona.city && ` · ${persona.city}`}
              </p>
            </div>
          </div>
        </div>

        {/* 卡片内容 */}
        <div className="p-5 space-y-4">
          {/* 基本信息 */}
          <div className="grid grid-cols-2 gap-3">
            {persona.age && (
              <div className="flex items-center gap-2 text-sm">
                <User className="w-4 h-4 text-slate-400" />
                <span className="text-slate-600">年龄:</span>
                <span className="font-medium text-slate-900">{persona.age}</span>
              </div>
            )}
            {persona.incomeLevel && (
              <div className="flex items-center gap-2 text-sm">
                <DollarSign className="w-4 h-4 text-slate-400" />
                <span className="text-slate-600">收入:</span>
                <span className="font-medium text-slate-900">{persona.incomeLevel}</span>
              </div>
            )}
            {persona.education && (
              <div className="flex items-center gap-2 text-sm col-span-2">
                <GraduationCap className="w-4 h-4 text-slate-400" />
                <span className="text-slate-600">教育:</span>
                <span className="font-medium text-slate-900">{persona.education}</span>
              </div>
            )}
          </div>

          {/* 核心需求 */}
          {persona.coreNeeds && (
            <div className="bg-amber-50 border border-amber-200 rounded-lg p-3">
              <div className="flex items-start gap-2">
                <Target className="w-4 h-4 text-amber-600 mt-0.5" />
                <div>
                  <p className="text-xs font-semibold text-amber-800 mb-1">核心需求</p>
                  <p className="text-sm text-amber-900">{persona.coreNeeds}</p>
                </div>
              </div>
            </div>
          )}

          {/* 使用场景 */}
          {persona.usageScenarios && (
            <div className="bg-green-50 border border-green-200 rounded-lg p-3">
              <div className="flex items-start gap-2">
                <Lightbulb className="w-4 h-4 text-green-600 mt-0.5" />
                <div>
                  <p className="text-xs font-semibold text-green-800 mb-1">使用场景</p>
                  <p className="text-sm text-green-900">{persona.usageScenarios}</p>
                </div>
              </div>
            </div>
          )}

          {/* 期望功能 */}
          {persona.expectedFeatures && (
            <div className="bg-purple-50 border border-purple-200 rounded-lg p-3">
              <div className="flex items-start gap-2">
                <Briefcase className="w-4 h-4 text-purple-600 mt-0.5" />
                <div>
                  <p className="text-xs font-semibold text-purple-800 mb-1">期望功能</p>
                  <p className="text-sm text-purple-900">{persona.expectedFeatures}</p>
                </div>
              </div>
            </div>
          )}

          {/* 行为特征 */}
          {(persona.informationChannels || persona.decisionFactors) && (
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
              <div className="flex items-start gap-2">
                <TrendingUp className="w-4 h-4 text-blue-600 mt-0.5" />
                <div>
                  <p className="text-xs font-semibold text-blue-800 mb-1">行为特征</p>
                  {persona.informationChannels && (
                    <p className="text-sm text-blue-900 mb-1">
                      <span className="font-medium">信息渠道:</span> {persona.informationChannels}
                    </p>
                  )}
                  {persona.decisionFactors && (
                    <p className="text-sm text-blue-900">
                      <span className="font-medium">决策因素:</span> {persona.decisionFactors}
                    </p>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* 用户引言 */}
          {persona.quote && (
            <div className="border-l-4 border-blue-500 pl-4 py-2 bg-slate-50 rounded-r-lg">
              <MessageSquare className="w-4 h-4 text-blue-500 mb-2" />
              <p className="text-sm italic text-slate-700">"{persona.quote}"</p>
            </div>
          )}

          {/* 原始描述（如果其他字段都没有） */}
          {!persona.coreNeeds && !persona.usageScenarios && persona.description && (
            <div className="text-sm text-slate-600 leading-relaxed">{persona.description}</div>
          )}
        </div>
      </div>
    );
  };

  return (
    <div className={containerClass}>
      {/* 工具栏 */}
      <div className="flex items-center justify-between mb-6 pb-4 border-b border-slate-200">
        <div className="flex items-center gap-3">
          <Users className="w-6 h-6 text-blue-500" />
          <h3 className="text-xl font-bold text-slate-900">用户画像 ({personas.length}个)</h3>
          {idea && (
            <span className="text-sm text-slate-500 truncate max-w-md">
              · {idea.substring(0, 50)}
              {idea.length > 50 ? '...' : ''}
            </span>
          )}
          {/* 流式生成状态指示器 */}
          {isStreaming && (
            <span className="flex items-center gap-2 text-sm text-blue-600 ml-2">
              <Loader2 className="w-4 h-4 animate-spin" />
              生成中... {progress}%
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => setZoom(Math.max(75, zoom - 25))}
            className="p-2 text-slate-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
            title="缩小"
          >
            <ZoomOut className="w-4 h-4" />
          </button>
          <span className="text-sm text-slate-600 min-w-[60px] text-center">{zoom}%</span>
          <button
            onClick={() => setZoom(Math.min(150, zoom + 25))}
            className="p-2 text-slate-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
            title="放大"
          >
            <ZoomIn className="w-4 h-4" />
          </button>

          <div className="w-px h-6 bg-slate-200 mx-2" />

          <button
            onClick={handleCopy}
            className="p-2 text-slate-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
            title="复制 JSON"
          >
            {copied ? <Check className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
          </button>
          <button
            onClick={handlePrint}
            className="p-2 text-slate-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
            title="打印"
          >
            <Printer className="w-4 h-4" />
          </button>
          <button
            onClick={handleExport}
            className="p-2 text-slate-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
            title="导出 JSON"
          >
            <Download className="w-4 h-4" />
          </button>
          {onSave && (
            <button
              onClick={onSave}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
            >
              <Save className="w-4 h-4" />
              保存
            </button>
          )}
          <button
            onClick={() => setIsFullscreen(!isFullscreen)}
            className="p-2 text-slate-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
            title={isFullscreen ? '退出全屏' : '全屏'}
          >
            {isFullscreen ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
          </button>
        </div>
      </div>

      {/* 生成时间 */}
      {generatedAt && (
        <div className="mb-4 text-sm text-slate-500 text-right">生成时间：{generatedAt}</div>
      )}

      {/* 画像列表 */}
      <div
        className="space-y-4 transition-transform duration-200"
        style={{ transform: `scale(${zoom / 100})`, transformOrigin: 'top center' }}
      >
        {personas.length === 0 ? (
          <div className="text-center py-12">
            <Users className="w-16 h-16 text-slate-300 mx-auto mb-4" />
            <p className="text-slate-500">暂无用户画像数据</p>
          </div>
        ) : (
          <>
            {/* 画像选择器（移动端友好） */}
            <div className="flex gap-2 overflow-x-auto pb-2 mb-4">
              {personas.map((persona, index) => (
                <button
                  key={index}
                  onClick={() => setSelectedPersonaIndex(index)}
                  className={`px-4 py-2 rounded-lg whitespace-nowrap transition-colors ${
                    index === selectedPersonaIndex
                      ? 'bg-blue-500 text-white'
                      : 'bg-slate-100 text-slate-700 hover:bg-slate-200'
                  }`}
                >
                  {persona.name || `画像 ${index + 1}`}
                </button>
              ))}
            </div>

            {/* 当前选中的画像卡片 */}
            {selectedPersonaIndex >= 0 &&
              selectedPersonaIndex < personas.length &&
              renderPersonaCard(personas[selectedPersonaIndex], selectedPersonaIndex)}

            {/* 所有画像概览（小卡片） */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mt-6 pt-6 border-t border-slate-200">
              {personas.map((persona, index) => (
                <div
                  key={index}
                  className={`p-4 rounded-lg border transition-all cursor-pointer ${
                    index === selectedPersonaIndex
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-slate-200 hover:border-blue-300 hover:bg-slate-50'
                  }`}
                  onClick={() => setSelectedPersonaIndex(index)}
                >
                  <div className="flex items-center gap-2 mb-2">
                    <User className="w-4 h-4 text-blue-500" />
                    <h4 className="font-semibold text-slate-900">
                      {persona.name || `画像 ${index + 1}`}
                    </h4>
                  </div>
                  <p className="text-sm text-slate-600 line-clamp-2">
                    {persona.coreNeeds || persona.description || '暂无描述'}
                  </p>
                </div>
              ))}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
