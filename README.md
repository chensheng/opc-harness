# OPC-HARNESS

> AI驱动的一人公司操作系统

OPC-HARNESS 是一个为独立创造者整合产品构思(Vibe Design)、快速构建(Vibe Coding)、增长运营(Vibe Marketing)三大模块的AI驱动桌面应用，实现从想法到产品的全流程闭环。

## ✨ 核心功能

### 🎨 Vibe Design - 产品构思
- 自然语言输入产品想法
- AI生成产品需求文档(PRD)
- 自动生成用户画像
- 竞品分析和差异化建议

### 💻 Vibe Coding - 快速构建
- 集成多种AI编码工具CLI (Kimi/Claude/Codex)
- 代码编辑器和文件管理
- 实时预览功能
- 一键部署到Vercel/Netlify

### 📈 Vibe Marketing - 增长运营
- AI生成发布策略
- 多平台营销文案生成
- 发布时间线规划
- 推广渠道建议

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Windows: WebView2 Runtime + Visual Studio Build Tools

### 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/opc-harness.git
cd opc-harness

# 安装前端依赖
npm install

# 安装Tauri CLI
cargo install tauri-cli
```

### 开发

```bash
# 启动开发服务器
npm run tauri:dev
```

### 构建

```bash
# 构建生产版本
npm run tauri:build
```

## 🏗️ 技术架构

### 前端
- **框架**: React 18 + TypeScript 5
- **样式**: Tailwind CSS 3 + shadcn/ui
- **状态管理**: Zustand + Immer
- **路由**: React Router 6

### 后端 (Tauri v2)
- **语言**: Rust
- **数据库**: SQLite (rusqlite)
- **密钥存储**: OS Keychain (keyring-rs)
- **HTTP客户端**: reqwest

### AI集成
- **支持厂商**: OpenAI, Anthropic Claude, 月之暗面Kimi, 智谱GLM
- **协议**: REST API + SSE (Server-Sent Events)

## 📁 项目结构

```
opc-harness/
├── src/                      # 前端代码
│   ├── components/           # React组件
│   │   ├── ui/              # shadcn/ui组件
│   │   ├── vibe-design/     # Vibe Design模块
│   │   ├── vibe-coding/     # Vibe Coding模块
│   │   ├── vibe-marketing/  # Vibe Marketing模块
│   │   └── common/          # 通用组件
│   ├── stores/              # Zustand状态管理
│   ├── types/               # TypeScript类型定义
│   └── lib/                 # 工具函数
├── src-tauri/               # Tauri后端代码
│   └── src/
│       ├── commands/        # Tauri命令
│       ├── ai/              # AI Provider适配器
│       ├── cli/             # CLI工具集成
│       ├── db/              # 数据库模块
│       └── services/        # 业务逻辑
└── package.json
```

## 🛣️ 路线图

### MVP (v1.0)
- [x] 项目基础架构
- [x] AI厂商配置管理
- [x] Vibe Design核心功能
- [x] Vibe Coding基础功能
- [x] Vibe Marketing基础功能

### v1.1
- [ ] 更多AI厂商支持
- [ ] 模板市场
- [ ] 团队协作功能

### v2.0
- [ ] 本地模型支持 (Ollama)
- [ ] 插件系统
- [ ] 移动端应用

## 🤝 贡献

欢迎提交Issue和Pull Request！

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📄 许可证

[MIT](LICENSE)

---

<p align="center">
  Made with ❤️ for indie creators
</p>
