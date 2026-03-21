import { useState } from 'react';
import {
  FileText,
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
} from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface PRDDisplayProps {
  prd: string;
  idea?: string;
  generatedAt?: string;
  onSave?: () => void;
  onExport?: () => void;
  isStreaming?: boolean;
  progress?: number;
}

export function PRDDisplay({
  prd,
  idea,
  generatedAt,
  onSave,
  onExport,
  isStreaming = false,
  progress = 0,
}: PRDDisplayProps) {
  const [zoom, setZoom] = useState(100);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(prd);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handlePrint = () => {
    const printWindow = window.open('', '_blank');
    if (printWindow) {
      printWindow.document.write(`
        <html>
          <head>
            <title>产品需求文档</title>
            <style>
              body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 40px; }
              h1 { color: #1e40af; border-bottom: 2px solid #3b82f6; padding-bottom: 10px; }
              h2 { color: #1e3a8a; margin-top: 30px; }
              h3 { color: #1e40af; }
              code { background: #f3f4f6; padding: 2px 6px; border-radius: 4px; }
              pre { background: #f9fafb; padding: 16px; border-radius: 8px; overflow-x: auto; }
              blockquote { border-left: 4px solid #3b82f6; padding-left: 16px; color: #6b7280; }
              table { border-collapse: collapse; width: 100%; margin: 20px 0; }
              th, td { border: 1px solid #d1d5db; padding: 12px; text-align: left; }
              th { background: #f3f4f6; font-weight: 600; }
            </style>
          </head>
          <body>
            ${prd.replace(/\n/g, '<br>').replace(/#/g, (_, __, str) => {
              const level = str.trim().indexOf('#');
              return `<h${level}>`.repeat(level === 0 ? 1 : level);
            })}
          </body>
        </html>
      `);
      printWindow.document.close();
      printWindow.print();
    }
  };

  const containerClass = isFullscreen
    ? 'fixed inset-0 z-50 bg-white p-8 overflow-auto'
    : 'bg-white rounded-2xl shadow-sm border border-slate-200 p-6 animate-fade-in';

  return (
    <div className={containerClass}>
      {/* 工具栏 */}
      <div className="flex items-center justify-between mb-6 pb-4 border-b border-slate-200">
        <div className="flex items-center gap-3">
          <FileText className="w-6 h-6 text-blue-500" />
          <h3 className="text-xl font-bold text-slate-900">产品需求文档</h3>
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
          {/* 复制按钮 */}
          <button
            onClick={handleCopy}
            className="p-2 text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors"
            title="复制内容"
          >
            {copied ? <Check className="w-4 h-4 text-green-500" /> : <Copy className="w-4 h-4" />}
          </button>

          {/* 打印按钮 */}
          <button
            onClick={handlePrint}
            className="p-2 text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors"
            title="打印文档"
          >
            <Printer className="w-4 h-4" />
          </button>

          {/* 保存按钮 */}
          {onSave && (
            <button
              onClick={onSave}
              className="p-2 text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors"
              title="保存到数据库"
            >
              <Save className="w-4 h-4" />
            </button>
          )}

          {/* 导出按钮 */}
          {onExport && (
            <button
              onClick={onExport}
              className="p-2 text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors"
              title="导出 Markdown 文件"
            >
              <Download className="w-4 h-4" />
            </button>
          )}

          {/* 缩放控制 */}
          <div className="flex items-center gap-1 ml-2 pl-2 border-l border-slate-200">
            <button
              onClick={() => setZoom(z => Math.max(z - 10, 50))}
              className="p-2 text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors"
              disabled={zoom <= 50}
            >
              <ZoomOut className="w-4 h-4" />
            </button>
            <span className="text-sm text-slate-600 w-12 text-center">{zoom}%</span>
            <button
              onClick={() => setZoom(z => Math.min(z + 10, 150))}
              className="p-2 text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors"
              disabled={zoom >= 150}
            >
              <ZoomIn className="w-4 h-4" />
            </button>
          </div>

          {/* 全屏切换 */}
          <button
            onClick={() => setIsFullscreen(!isFullscreen)}
            className="p-2 text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition-colors ml-2"
            title={isFullscreen ? '退出全屏' : '全屏查看'}
          >
            {isFullscreen ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
          </button>
        </div>
      </div>

      {/* PRD 内容 */}
      <div
        className="prose prose-slate max-w-none transition-all duration-200 relative"
        style={{ transform: `scale(${zoom / 100})`, transformOrigin: 'top left' }}
      >
        <ReactMarkdown remarkPlugins={[remarkGfm]}>{prd}</ReactMarkdown>

        {/* 流式生成时的光标效果 */}
        {isStreaming && (
          <span className="inline-block w-2 h-5 bg-blue-500 animate-pulse ml-1"></span>
        )}
      </div>

      {/* 进度条 */}
      {isStreaming && progress > 0 && (
        <div className="mt-6">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm text-slate-600">生成进度</span>
            <span className="text-sm font-medium text-blue-600">{progress}%</span>
          </div>
          <div className="w-full bg-slate-200 rounded-full h-2 overflow-hidden">
            <div
              className="bg-gradient-to-r from-blue-500 to-purple-600 h-full transition-all duration-300 ease-out"
              style={{ width: `${progress}%` }}
            />
          </div>
        </div>
      )}

      {/* 元信息 */}
      {generatedAt && (
        <div className="mt-8 pt-6 border-t border-slate-200 flex items-center justify-between text-sm text-slate-500">
          <span>生成时间：{new Date(generatedAt).toLocaleString('zh-CN')}</span>
          <span>{prd.length.toLocaleString()} 字符</span>
        </div>
      )}

      {/* 全屏模式下的关闭按钮 */}
      {isFullscreen && (
        <button
          onClick={() => setIsFullscreen(false)}
          className="fixed top-4 right-4 p-2 bg-white shadow-lg rounded-full hover:bg-slate-100 transition-colors z-50"
        >
          <Minimize2 className="w-5 h-5" />
        </button>
      )}
    </div>
  );
}
