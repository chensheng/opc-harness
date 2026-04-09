# 用户故事管理功能 (User Story Management)

## 📋 概述

Vibe Coding 模块新增了用户故事管理功能，支持通过 AI 将 PRD 或产品需求自动拆分为符合 INVEST 原则的用户故事。

## ✨ 功能特性

### 1. AI 驱动的用户故事拆分
- **智能识别**：从 PRD 内容中自动识别功能点
- **标准格式**：生成符合 "As a... I want... So that..." 格式的用户故事
- **验收标准**：为每个故事自动生成可测试的验收标准
- **优先级评估**：基于业务价值和技术复杂度评估优先级（P0-P3）
- **故事点估算**：提供工作量估算（Story Points）

### 2. 用户故事管理
- **可视化展示**：清晰的故事卡片布局
- **状态跟踪**：支持 draft → refined → approved → in_development → completed 流程
- **依赖管理**：标识故事间的依赖关系
- **标签系统**：支持多维度分类和筛选
- **统计面板**：实时显示故事数量、优先级分布等指标

## 🚀 快速开始

### 在前端组件中使用

```tsx
import { UserStoryManager } from '@/components/vibe-coding/UserStoryManager'

function MyComponent() {
  const handleStoriesGenerated = (stories) => {
    console.log('生成了', stories.length, '个用户故事')
    // 处理生成的故事...
  }

  return (
    <UserStoryManager 
      prdContent="# 任务管理系统\n我们需要一个..."
      apiKey="your-api-key"  // 可选
      onStoriesGenerated={handleStoriesGenerated}
    />
  )
}
```

### 在代码中使用 Hook

```tsx
import { useUserStoryDecomposition } from '@/hooks/useUserStoryDecomposition'

function MyComponent() {
  const { userStories, loading, error, decompose } = useUserStoryDecomposition()

  const handleDecompose = async () => {
    await decompose(
      '# 我的产品需求\n...',
      'optional-api-key'
    )
    
    if (userStories.length > 0) {
      console.log('拆分成功！', userStories)
    }
  }

  return (
    <div>
      <button onClick={handleDecompose} disabled={loading}>
        {loading ? '拆分中...' : '开始拆分'}
      </button>
      
      {error && <div className="error">{error}</div>}
      
      {userStories.map(story => (
        <div key={story.id}>
          <h3>{story.title}</h3>
          <p>As a {story.role}, I want {story.feature}</p>
        </div>
      ))}
    </div>
  )
}
```

## 📊 数据结构

### UserStory 类型定义

```typescript
interface UserStory {
  id: string                    // 唯一标识符
  storyNumber: string           // 故事编号 (US-001)
  title: string                 // 故事标题
  role: string                  // 角色 (As a ...)
  feature: string               // 功能 (I want ...)
  benefit: string               // 价值 (So that ...)
  description: string           // 详细描述
  acceptanceCriteria: string[]  // 验收标准
  priority: 'P0' | 'P1' | 'P2' | 'P3'  // 优先级
  status: 'draft' | 'refined' | 'approved' | 'in_development' | 'completed'
  storyPoints?: number          // 故事点估算
  dependencies?: string[]       // 依赖的故事 ID
  featureModule?: string        // 功能模块
  labels: string[]              // 标签
  createdAt: string             // 创建时间
  updatedAt: string             // 更新时间
}
```

## 🔧 后端实现

### Tauri Command

```rust
#[tauri::command]
pub async fn decompose_user_stories(
    request: DecomposeUserStoriesRequest,
) -> Result<DecomposeUserStoriesResponse, String>
```

**请求参数：**
- `prd_content`: PRD 内容或功能描述
- `api_key`: 可选的 AI API Key

**响应数据：**
- `success`: 是否成功
- `user_stories`: 拆分出的用户故事列表
- `error_message`: 错误消息（如果失败）

### 当前实现状态

✅ **已完成：**
- 前端 UI 组件（`UserStoryManager.tsx`）
- React Hook（`useUserStoryDecomposition.ts`）
- 类型定义（TypeScript & Rust）
- Tauri Command 接口
- Mock 数据生成器
- 单元测试（前端 + Rust）

🔄 **待实现：**
- 真实的 AI 集成（调用 OpenAI/Claude/Kimi 等）
- 用户故事的持久化存储
- 故事编辑和审核工作流
- 与 Issue 系统的关联映射
- 批量导入/导出功能

## 📝 使用示例

### 示例 1：基本用法

```tsx
<UserStoryManager 
  prdContent={`
    # 在线商城系统
    
    我们需要开发一个在线商城，包含以下核心功能：
    1. 用户注册和登录
    2. 商品浏览和搜索
    3. 购物车管理
    4. 订单处理
    5. 支付集成
  `}
/>
```

### 示例 2：带回调的高级用法

```tsx
const [stories, setStories] = useState<UserStory[]>([])

<UserStoryManager 
  prdContent={prdContent}
  apiKey={apiKey}
  onStoriesGenerated={(newStories) => {
    setStories(newStories)
    // 保存到数据库
    saveToDatabase(newStories)
    // 触发后续流程
    startTaskDecomposition(newStories)
  }}
/>
```

## 🎯 INVEST 原则

生成的用户故事遵循敏捷开发的 INVEST 原则：

- **I**ndependent（独立的）：尽量减少故事间的依赖
- **N**egotiable（可协商的）：细节可以在讨论中调整
- **V**aluable（有价值的）：对用户或业务有明确价值
- **E**stimable（可估算的）：可以评估工作量
- **S**mall（小的）：足够小，可在一个迭代内完成
- **T**estable（可测试的）：有明确的验收标准

## 🧪 测试

### 运行前端测试

```bash
npm run test:unit -- UserStoryManager
```

### 运行 Rust 测试

```bash
cd src-tauri
cargo test tests_user_story_decomposition
```

## 📚 相关文件

- **前端组件**: `src/components/vibe-coding/UserStoryManager.tsx`
- **React Hook**: `src/hooks/useUserStoryDecomposition.ts`
- **类型定义**: `src/types/index.ts` (UserStory 相关类型)
- **Rust Command**: `src-tauri/src/commands/quality.rs` (decompose_user_stories)
- **测试文件**: 
  - `src/components/vibe-coding/UserStoryManager.test.tsx`
  - `src-tauri/src/commands/quality.rs` (tests_user_story_decomposition)

## 🔮 未来规划

1. **AI 集成优化**
   - 支持多种 AI 提供商（OpenAI, Claude, Kimi, GLM）
   - 提示词工程优化，提高拆分质量
   - 支持自定义拆分规则

2. **协作功能**
   - 多人评审和评论
   - 故事版本历史
   - 变更追踪

3. **项目管理集成**
   - 一键转换为 Issues
   - 与 Milestone 关联
   - Sprint 规划支持

4. **数据分析**
   - 故事完成率统计
   - 速度追踪（Velocity）
   - 瓶颈识别

## 🤝 贡献指南

欢迎贡献代码、报告问题或提出改进建议！

---

**最后更新**: 2026-04-09  
**维护者**: OPC-Harness Team
