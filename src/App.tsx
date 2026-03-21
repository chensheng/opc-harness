import { useState } from 'react';
import { Sparkles, Code2, Rocket, Settings, Github, Twitter } from 'lucide-react';

function App() {
  const [activeTab, setActiveTab] = useState<'design' | 'coding' | 'marketing'>('design');

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900">
      {/* Header */}
      <header className="bg-white border-b border-slate-200 px-6 py-4">
        <div className="flex items-center justify-between max-w-7xl mx-auto">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-xl">
              OPC
            </div>
            <div>
              <h1 className="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                OPC-HARNESS
              </h1>
              <p className="text-xs text-slate-500">AI驱动的一人公司操作系统</p>
            </div>
          </div>
          
          <div className="flex items-center gap-4">
            <button className="p-2 text-slate-500 hover:text-slate-700 hover:bg-slate-100 rounded-lg transition-colors">
              <Settings className="w-5 h-5" />
            </button>
            <a 
              href="https://github.com/opc-harness/opc-harness" 
              target="_blank"
              rel="noopener noreferrer"
              className="p-2 text-slate-500 hover:text-slate-700 hover:bg-slate-100 rounded-lg transition-colors"
            >
              <Github className="w-5 h-5" />
            </a>
          </div>
        </div>
      </header>

      {/* Main Navigation */}
      <nav className="bg-white border-b border-slate-200 px-6">
        <div className="max-w-7xl mx-auto flex gap-1">
          {[
            { id: 'design', label: 'Vibe Design', icon: Sparkles, desc: '产品构思' },
            { id: 'coding', label: 'Vibe Coding', icon: Code2, desc: '快速构建' },
            { id: 'marketing', label: 'Vibe Marketing', icon: Rocket, desc: '增长运营' },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as typeof activeTab)}
              className={`flex items-center gap-2 px-6 py-4 border-b-2 transition-colors ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-slate-600 hover:text-slate-900 hover:border-slate-300'
              }`}
            >
              <tab.icon className="w-5 h-5" />
              <div className="text-left">
                <div className="font-medium">{tab.label}</div>
                <div className="text-xs opacity-70">{tab.desc}</div>
              </div>
            </button>
          ))}
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto p-6">
        {activeTab === 'design' && <VibeDesignPanel />}
        {activeTab === 'coding' && <VibeCodingPanel />}
        {activeTab === 'marketing' && <VibeMarketingPanel />}
      </main>
    </div>
  );
}

// Vibe Design Panel
function VibeDesignPanel() {
  const [idea, setIdea] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);

  const handleGenerate = async () => {
    if (!idea.trim()) return;
    setIsGenerating(true);
    // TODO: Call Tauri command to generate PRD
    setTimeout(() => setIsGenerating(false), 2000);
  };

  return (
    <div className="space-y-6 animate-fade-in">
      <div className="bg-white rounded-2xl shadow-sm border border-slate-200 p-8">
        <h2 className="text-2xl font-bold mb-2">💡 输入你的产品想法</h2>
        <p className="text-slate-600 mb-6">
          描述你的产品创意，AI将帮你完善产品构思，生成PRD、用户画像和竞品分析
        </p>
        
        <textarea
          value={idea}
          onChange={(e) => setIdea(e.target.value)}
          placeholder="我想做一个帮助独立开发者管理项目进度的工具，类似Trello但是更简单，专门为单人项目设计..."
          className="w-full h-40 p-4 border border-slate-300 rounded-xl resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
        
        <div className="flex items-center justify-between mt-4">
          <div className="text-sm text-slate-500">
            支持自然语言描述，越详细越好
          </div>
          <button
            onClick={handleGenerate}
            disabled={!idea.trim() || isGenerating}
            className="flex items-center gap-2 px-6 py-3 bg-blue-600 text-white rounded-xl font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <Sparkles className="w-5 h-5" />
            {isGenerating ? 'AI思考中...' : '开始分析'}
          </button>
        </div>
      </div>

      {/* Quick Start Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {[
          { title: 'SaaS工具', desc: '订阅制软件服务', icon: '🚀' },
          { title: '个人博客', desc: '内容创作平台', icon: '✍️' },
          { title: '电商网站', desc: '在线销售产品', icon: '🛒' },
        ].map((template) => (
          <button
            key={template.title}
            onClick={() => setIdea(`我想做一个${template.desc}...`)}
            className="p-6 bg-white rounded-xl border border-slate-200 hover:border-blue-300 hover:shadow-md transition-all text-left"
          >
            <div className="text-3xl mb-3">{template.icon}</div>
            <h3 className="font-semibold text-slate-900">{template.title}</h3>
            <p className="text-sm text-slate-500 mt-1">{template.desc}</p>
          </button>
        ))}
      </div>
    </div>
  );
}

// Vibe Coding Panel
function VibeCodingPanel() {
  return (
    <div className="bg-white rounded-2xl shadow-sm border border-slate-200 p-8 animate-fade-in">
      <div className="text-center py-12">
        <Code2 className="w-16 h-16 text-slate-300 mx-auto mb-4" />
        <h2 className="text-2xl font-bold mb-2">Vibe Coding</h2>
        <p className="text-slate-600 max-w-lg mx-auto mb-6">
          通过AI编码工具（Kimi CLI、Claude Code、Codex）快速构建你的产品原型。
          先在 Vibe Design 中完成产品构思，然后进入编码阶段。
        </p>
        <button className="px-6 py-3 bg-slate-100 text-slate-700 rounded-xl font-medium hover:bg-slate-200 transition-colors">
          请先完成 Vibe Design
        </button>
      </div>
    </div>
  );
}

// Vibe Marketing Panel
function VibeMarketingPanel() {
  return (
    <div className="bg-white rounded-2xl shadow-sm border border-slate-200 p-8 animate-fade-in">
      <div className="text-center py-12">
        <Rocket className="w-16 h-16 text-slate-300 mx-auto mb-4" />
        <h2 className="text-2xl font-bold mb-2">Vibe Marketing</h2>
        <p className="text-slate-600 max-w-lg mx-auto mb-6">
          AI辅助的增长运营工具，帮你生成发布策略、营销文案，助力产品增长。
        </p>
        <div className="flex items-center justify-center gap-4">
          <a 
            href="#"
            onClick={(e) => { e.preventDefault(); alert('敬请期待'); }}
            className="flex items-center gap-2 px-4 py-2 text-slate-500 hover:text-blue-600 transition-colors"
          >
            <Twitter className="w-5 h-5" />
            Twitter
          </a>
        </div>
      </div>
    </div>
  );
}

export default App;
