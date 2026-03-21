import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  CheckCircle2,
  XCircle,
  Download,
  Terminal,
  ExternalLink,
  AlertCircle,
  RefreshCw,
} from 'lucide-react';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { Badge } from './ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';

// 工具信息接口
interface ToolInfo {
  name: string;
  installed: boolean;
  version: string | null;
  path: string | null;
}

// 带类别的工具信息
interface ToolInfoWithCategory extends ToolInfo {
  category: string;
  is_required: boolean;
  description: string;
}

// 工具安装链接
const TOOL_INSTALL_LINKS: Record<string, { url: string; command?: string; description: string }> = {
  'Node.js': {
    url: 'https://nodejs.org/',
    command: 'npm install -g node',
    description: 'JavaScript 运行时环境',
  },
  npm: {
    url: 'https://nodejs.org/',
    description: '随 Node.js 一起安装',
  },
  pnpm: {
    url: 'https://pnpm.io/installation',
    command: 'npm install -g pnpm',
    description: '高性能包管理器',
  },
  Yarn: {
    url: 'https://yarnpkg.com/getting-started/install',
    command: 'npm install -g yarn',
    description: 'Facebook 包管理器',
  },
  Git: {
    url: 'https://git-scm.com/downloads',
    description: '分布式版本控制系统',
  },
  Python: {
    url: 'https://www.python.org/downloads/',
    description: 'Python 编程语言',
  },
  Rust: {
    url: 'https://rustup.rs/',
    command: 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh',
    description: 'Rust 编程语言',
  },
  Docker: {
    url: 'https://docs.docker.com/get-docker/',
    description: '容器化平台',
  },
  'VS Code': {
    url: 'https://code.visualstudio.com/download',
    description: 'Visual Studio Code 编辑器',
  },
  Cursor: {
    url: 'https://cursor.sh/',
    description: 'AI 驱动的代码编辑器',
  },
  'Kimi CLI': {
    url: 'https://kimi.moonshot.cn/',
    command: 'npm install -g @moonshot/kimi-cli',
    description: 'Kimi AI 编程助手',
  },
  'Claude Code': {
    url: 'https://claude.ai/',
    command: 'npm install -g @anthropic/claude-code',
    description: 'Claude AI 编程助手',
  },
  'Codex CLI': {
    url: 'https://platform.openai.com/codex',
    command: 'npm install -g @openai/codex',
    description: 'OpenAI Codex CLI',
  },
};

export function ToolInstallerGuide() {
  const [tools, setTools] = useState<ToolInfoWithCategory[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // 检测工具
  const detectTools = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<ToolInfoWithCategory[]>('detect_tools_detailed');
      setTools(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : '检测工具失败');
    } finally {
      setLoading(false);
    }
  };

  // 初始检测
  useEffect(() => {
    detectTools();
  }, []);

  // 按类别分组
  const toolsByCategory = tools.reduce(
    (acc, tool) => {
      const category = tool.category;
      if (!acc[category]) {
        acc[category] = [];
      }
      acc[category].push(tool);
      return acc;
    },
    {} as Record<string, ToolInfoWithCategory[]>
  );

  // 统计
  const installedCount = tools.filter(t => t.installed).length;
  const requiredCount = tools.filter(t => t.is_required).length;
  const requiredInstalledCount = tools.filter(t => t.is_required && t.installed).length;
  const allRequiredInstalled = requiredCount === requiredInstalledCount;

  // 类别中文名
  const categoryNames: Record<string, string> = {
    Runtime: '运行时环境',
    VersionControl: '版本控制',
    PackageManager: '包管理器',
    AICoding: 'AI 编码工具',
    Editor: '编辑器',
    BuildTool: '构建工具',
  };

  if (loading) {
    return (
      <Card className="w-full">
        <CardContent className="flex items-center justify-center py-12">
          <RefreshCw className="w-8 h-8 animate-spin text-blue-500" />
          <span className="ml-3 text-slate-600">正在检测开发工具...</span>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card className="w-full border-red-200">
        <CardContent className="flex flex-col items-center justify-center py-12">
          <AlertCircle className="w-12 h-12 text-red-500 mb-4" />
          <p className="text-red-600 mb-4">{error}</p>
          <Button onClick={detectTools} variant="outline">
            <RefreshCw className="w-4 h-4 mr-2" />
            重试
          </Button>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className="w-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <Terminal className="w-5 h-5" />
              开发工具检测
            </CardTitle>
            <CardDescription>检测本地开发环境，确保所有必需工具已安装</CardDescription>
          </div>
          <Button onClick={detectTools} variant="outline" size="sm">
            <RefreshCw className="w-4 h-4 mr-2" />
            重新检测
          </Button>
        </div>

        {/* 统计信息 */}
        <div className="flex items-center gap-4 mt-4">
          <div className="flex items-center gap-2">
            <div
              className={`w-2 h-2 rounded-full ${allRequiredInstalled ? 'bg-green-500' : 'bg-yellow-500'}`}
            />
            <span className="text-sm text-slate-600">
              必需工具: {requiredInstalledCount}/{requiredCount}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-full bg-blue-500" />
            <span className="text-sm text-slate-600">
              已安装: {installedCount}/{tools.length}
            </span>
          </div>
        </div>

        {/* 警告提示 */}
        {!allRequiredInstalled && (
          <div className="flex items-start gap-2 p-3 bg-yellow-50 border border-yellow-200 rounded-lg mt-4">
            <AlertCircle className="w-5 h-5 text-yellow-600 mt-0.5" />
            <div className="text-sm text-yellow-800">
              <p className="font-medium">缺少必需工具</p>
              <p>请安装所有标记为"必需"的工具，以确保正常使用所有功能。</p>
            </div>
          </div>
        )}
      </CardHeader>

      <CardContent>
        <Tabs defaultValue="Runtime" className="w-full">
          <TabsList className="grid grid-cols-3 lg:grid-cols-6 mb-4">
            {Object.keys(toolsByCategory).map(category => (
              <TabsTrigger key={category} value={category} className="text-xs">
                {categoryNames[category] || category}
              </TabsTrigger>
            ))}
          </TabsList>

          {Object.entries(toolsByCategory).map(([category, categoryTools]) => (
            <TabsContent key={category} value={category} className="space-y-3">
              {categoryTools.map(tool => (
                <ToolItem key={tool.name} tool={tool} />
              ))}
            </TabsContent>
          ))}
        </Tabs>
      </CardContent>
    </Card>
  );
}

// 单个工具项组件
function ToolItem({ tool }: { tool: ToolInfoWithCategory }) {
  const installInfo = TOOL_INSTALL_LINKS[tool.name];

  return (
    <div
      className={`flex items-center justify-between p-4 rounded-lg border ${
        tool.installed ? 'bg-green-50 border-green-200' : 'bg-slate-50 border-slate-200'
      }`}
    >
      <div className="flex items-center gap-3">
        {tool.installed ? (
          <CheckCircle2 className="w-5 h-5 text-green-600" />
        ) : (
          <XCircle className="w-5 h-5 text-slate-400" />
        )}
        <div>
          <div className="flex items-center gap-2">
            <span className="font-medium">{tool.name}</span>
            {tool.is_required && (
              <Badge variant="secondary" className="text-xs">
                必需
              </Badge>
            )}
          </div>
          <p className="text-sm text-slate-500">{tool.description}</p>
          {tool.installed && tool.version && (
            <p className="text-xs text-green-600 mt-1">版本: {tool.version}</p>
          )}
        </div>
      </div>

      {!tool.installed && installInfo && (
        <div className="flex items-center gap-2">
          {installInfo.command && (
            <code className="hidden lg:block text-xs bg-slate-100 px-2 py-1 rounded">
              {installInfo.command}
            </code>
          )}
          <a
            href={installInfo.url}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center"
          >
            <Button size="sm" variant="outline">
              <Download className="w-4 h-4 mr-1" />
              安装
              <ExternalLink className="w-3 h-3 ml-1" />
            </Button>
          </a>
        </div>
      )}
    </div>
  );
}

export default ToolInstallerGuide;
