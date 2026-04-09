# 用户故事管理功能 (User Story Management)

## 📋 概述

Vibe Coding 模块新增了用户故事管理功能，支持通过 AI 将 PRD 或产品需求自动拆分为符合 INVEST 原则的用户故事。

**✅ 已实现真实 AI 集成**：本功能现已对接真实的 AI 服务（OpenAI/Claude/Kimi/GLM等），不再使用 Mock 数据。

## ✨ 功能特性

### 1. AI 驱动的用户故事拆分
- **智能识别**：从 PRD 内容中自动识别功能点
- **标准格式**：生成符合 "As a... I want... So that..." 格式的用户故事
- **验收标准**：为每个故事自动生成可测试的验收标准
- **优先级评估**：基于业务价值和技术复杂度评估优先级（P0-P3）
- **故事点估算**：提供工作量估算（Story Points）
- **依赖分析**：自动识别故事间的依赖关系
- **模块划分**：智能划分功能模块

### 2. 用户故事管理
- **可视化展示**：清晰的故事卡片布局
- **状态跟踪**：支持 draft → refined → approved → in_development → completed 流程
- **依赖管理**：标识故事间的依赖关系
- **标签系统**：支持多维度分类和筛选
- **统计面板**：实时显示故事数量、优先级分布等指标

## 🔧 AI 集成说明

### 支持的 AI 提供商与模型推荐

#### ✅ 强烈推荐
- **OpenAI GPT-4 Turbo** (`gpt-4-turbo-preview`)
  - 最稳定,成功率最高
  - 适合复杂PRD分析
  
- **OpenAI GPT-3.5 Turbo** (`gpt-3.5-turbo`)
  - 性价比高,速度快
  - 适合简单到中等复杂度PRD

#### ⚠️ 谨慎使用
- **Kimi (Moonshot AI)**
  - ❌ **不要使用 `kimi-for-coding` 模型** - 该模型可能返回编码数据
  - ✅ 推荐使用 `moonshot-v1-8k` 或 `moonshot-v1-32k`
  - 适合长文档处理(支持128K上下文)
  
- **GLM (智谱 AI)**
  - 推荐使用 `glm-4` 或 `glm-3-turbo`
  - 中文理解能力较好

- **Anthropic Claude**
  - 推荐使用 `claude-3-sonnet-20240229`
  - 逻辑推理能力强

#### ❌ 不推荐
- Kimi for Coding (`kimi-for-coding`) - 可能返回损坏的Base64数据
- 任何实验性/测试版模型

### 默认配置
- **模型**: OpenAI GPT-4 Turbo (`gpt-4-turbo-preview`)
- **Temperature**: 0.7 (平衡创造性和准确性)
- **Max Tokens**: 4096 (支持长PRD解析)
- **响应格式**: Markdown 表格(主要) / JSON(后备)

### 输出格式

AI 会以 **Markdown 表格**格式输出用户故事,包含以下列:
- 序号、标题、角色、功能、价值
- 优先级、故事点、验收标准
- 模块、标签、依赖关系

系统会自动将表格转换为结构化的用户故事数据。

### API Key 配置方式

#### 方式 1: 在应用设置中配置 (推荐)
1. 打开应用的 **AI 配置页面**
2. 选择你喜欢的 AI 提供商 (OpenAI/Claude/Kimi/GLM)
3. 输入对应的 API Key
4. 选择要使用的模型
5. 点击"保存"并设置为"激活"

✅ **用户故事拆分功能会自动使用已配置的 AI 提供商和模型**,无需额外操作!

#### 方式 2: 环境变量 (备选)
``bash
# Windows PowerShell
$env:OPENAI_API_KEY="sk-your-api-key"

# Linux/Mac
export OPENAI_API_KEY="sk-your-api-key"
```

支持的环境变量:
- `OPENAI_API_KEY`
- `ANTHROPIC_API_KEY`
- `MOONSHOT_API_KEY` / `KIMI_API_KEY`
- `ZHIPU_API_KEY` / `GLM_API_KEY`

#### 方式 3: 前端直接传入 (开发调试用)
``tsx
<UserStoryManager 
  prdContent={prdContent}
  provider="openai"
  model="gpt-4-turbo-preview"
  apiKey="sk-your-api-key"
/>
```

## 🚀 快速开始

### 在前端组件中使用

``tsx
import { UserStoryManager } from '@/components/vibe-coding/UserStoryManager'

function MyComponent() {
  const handleStoriesGenerated = (stories) => {
    console.log('生成了', stories.length, '个用户故事')
    // 处理生成的故事...
  }

  return (
    <UserStoryManager 
      prdContent="# 任务管理系统\n我们需要一个..."
      apiKey="your-api-key"  // 可选，如果不传则使用环境变量
      onStoriesGenerated={handleStoriesGenerated}
    />
  )
}
```

### 在代码中使用 Hook

``tsx
import { useUserStoryDecomposition } from '@/hooks/useUserStoryDecomposition'

function MyComponent() {
  const { userStories, loading, error, decompose } = useUserStoryDecomposition()

  const handleDecompose = async () => {
    await decompose(
      '# 我的产品需求\n...',
      'optional-api-key'  // 可选
    )
    
    if (userStories.length > 0) {
      console.log('拆分成功！', userStories)
    }
  }

  return (
    <div>
      <button onClick={handleDecompose} disabled={loading}>
        {loading ? 'AI 拆分中...' : '开始拆分'}
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

``typescript
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

``rust
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

### 示例 1：基本用法（使用环境变量中的 API Key）

``tsx
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

### 示例 2：传入 API Key

``tsx
<UserStoryManager 
  prdContent={prdContent}
  apiKey="sk-your-openai-api-key"
  onStoriesGenerated={(stories) => {
    console.log(`成功生成 ${stories.length} 个用户故事`)
    // 保存到数据库或进行其他处理
  }}
/>
```

### 示例 3：使用 Hook 自定义界面

``tsx
import { useUserStoryDecomposition } from '@/hooks/useUserStoryDecomposition'

function CustomStoryDecomposer() {
  const { userStories, loading, error, decompose } = useUserStoryDecomposition()
  const [input, setInput] = useState('')

  const handleDecompose = async () => {
    try {
      await decompose(input)
    } catch (err) {
      console.error('拆分失败:', err)
    }
  }

  return (
    <div className="space-y-4">
      <textarea
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="输入 PRD 内容..."
        className="w-full h-64 p-4 border rounded"
      />
      
      <button
        onClick={handleDecompose}
        disabled={loading || !input.trim()}
        className="px-6 py-2 bg-blue-500 text-white rounded disabled:bg-gray-300"
      >
        {loading ? 'AI 拆分中...' : '开始拆分'}
      </button>
      
      {error && (
        <div className="p-4 bg-red-50 text-red-700 rounded">
          {error}
        </div>
      )}
      
      {userStories.length > 0 && (
        <div className="space-y-4">
          <h3 className="text-xl font-bold">
            生成了 {userStories.length} 个用户故事
          </h3>
          {userStories.map(story => (
            <div key={story.id} className="p-4 border rounded">
              <div className="flex items-center gap-2 mb-2">
                <span className="font-bold">{story.storyNumber}</span>
                <span className={`px-2 py-1 rounded text-xs ${
                  story.priority === 'P0' ? 'bg-red-500 text-white' :
                  story.priority === 'P1' ? 'bg-orange-500 text-white' :
                  'bg-gray-200'
                }`}>
                  {story.priority}
                </span>
              </div>
              <h4 className="font-semibold mb-2">{story.title}</h4>
              <p className="text-sm text-gray-600 mb-2">
                As a <strong>{story.role}</strong>, 
                I want <strong>{story.feature}</strong>, 
                so that <strong>{story.benefit}</strong>
              </p>
              {story.acceptanceCriteria.length > 0 && (
                <div className="mt-2">
                  <p className="text-xs font-semibold text-gray-500">验收标准：</p>
                  <ul className="list-disc list-inside text-sm">
                    {story.acceptanceCriteria.map((criteria, idx) => (
                      <li key={idx}>{criteria}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
```

## ⚙️ 配置选项

### 切换 AI 提供商

修改 `src-tauri/src/commands/quality.rs` 中的 `decompose_with_ai` 函数：

``rust
// 默认使用 OpenAI
let provider = AIProvider::new(AIProviderType::OpenAI, api_key);

// 切换到 Kimi
let provider = AIProvider::new(AIProviderType::Kimi, api_key);

// 切换到 Claude
let provider = AIProvider::new(AIProviderType::Anthropic, api_key);

// 切换到 GLM
let provider = AIProvider::new(AIProviderType::GLM, api_key);
```

### 调整模型参数

``rust
let chat_request = ChatRequest {
    model: "gpt-3.5-turbo".to_string(),  // 更经济的模型
    messages: vec![...],
    temperature: Some(0.5),  // 更低温度，更确定性
    max_tokens: Some(2048),  // 减少 token 使用
    stream: false,
};
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

### 运行单元测试

``bash
cd src-tauri
cargo test tests_user_story_decomposition --bin opc-harness
```

### 运行前端测试

``bash
npm run test:unit -- UserStoryManager
```

## 🐛 故障排查

### 问题 1：提示 "未提供 API Key"

**解决方案**：
1. 设置环境变量：`export OPENAI_API_KEY="sk-xxx"`
2. 或在组件中传入：`apiKey="sk-xxx"`
3. 或在应用的 AI 配置页面设置

### 问题 2：AI 调用超时

**解决方案**：
1. 检查网络连接
2. 验证 API Key 是否有效
3. 尝试切换到其他 AI 提供商
4. 减少 PRD 内容长度

### 问题 3：解析失败

**解决方案**：
1. 查看控制台日志了解详细错误
2. 检查 AI 响应格式是否正确
3. 尝试降低 temperature 参数
4. 简化 PRD 内容结构

### 问题 4：生成的故事质量不高

**解决方案**：
1. 优化 PRD 内容，使其更结构化
2. 在"用户要求"中添加更多指导信息
3. 调整 temperature 参数（更高更有创造性，更低更准确）
4. 考虑切换到更强大的模型（如 GPT-4）

## 📊 性能优化建议

1. **缓存机制**：对相同的 PRD 内容缓存结果，避免重复调用
2. **增量更新**：只重新拆分修改的部分
3. **后台处理**：对于大型 PRD，考虑使用 Web Worker 或后台任务
4. **流式响应**：未来可以支持流式显示生成的故事

## 📚 相关文件

- **前端组件**: `src/components/vibe-coding/UserStoryManager.tsx`
- **React Hook**: `src/hooks/useUserStoryDecomposition.ts`
- **类型定义**: `src/types/index.ts` (UserStory 相关类型)
- **Rust Command**: `src-tauri/src/commands/quality.rs` (decompose_user_stories)
- **测试文件**: 
  - `src/components/vibe-coding/UserStoryManager.test.tsx`
  - `src-tauri/src/commands/quality.rs` (tests_user_story_decomposition)

## 🔮 未来规划

- [ ] 支持流式输出，实时显示生成的故事
- [ ] 添加故事编辑和审核工作流
- [ ] 与 Issue 追踪系统集成
- [ ] 支持批量导入/导出
- [ ] 添加故事复杂度可视化
- [ ] 支持多语言 PRD 解析