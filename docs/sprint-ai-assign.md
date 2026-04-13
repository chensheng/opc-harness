# Sprint AI智能分配用户故事功能

## 功能概述

在Sprint计划管理中新增了AI智能分配功能，能够通过AI分析未分配的用户故事，智能推荐适合分配到指定Sprint的故事列表。

**✨ 新增特性（2026-04-13）**：实时显示AI分析过程，让用户清晰了解AI的思考逻辑和推理步骤。

## 使用步骤

1. **前置条件**
   - 确保已在设置中配置AI服务（OpenAI/Claude/Kimi等）
   - 确保有未分配的用户故事（sprintId为空的故事）
   - 确保已创建至少一个Sprint计划

2. **操作流程**
   ```
   Sprint计划TAB 
   → 找到目标Sprint
   → 点击Sparkles图标按钮（AI智能分配）
   → 在弹出的对话框中点击"开始AI分析"
   → 🆕 实时查看AI的分析和思考过程
   → 查看AI推荐结果和推荐理由
   → 选择接受全部或部分推荐
   → 点击"确认分配"完成操作
   ```

3. **界面说明**
   - **左侧区域**：
     - Sprint基本信息卡片（名称、目标、时间范围、容量情况）
     - 操作按钮（开始AI分析）
     - 🆕 **AI分析过程实时显示区**：流式展示AI的思考过程，包含分析步骤、优先级评估、容量计算等
     - 已选故事统计和容量警告
   - **右侧区域**：显示未分配的用户故事列表
   - **AI推荐标识**：被AI推荐的故事会有绿色高亮显示，并展示推荐理由和置信度
   - **容量提示**：实时显示选中故事的总点数，超出容量时给出警告

## 技术实现

### 核心组件

- **AIAssignStoriesDialog.tsx**: AI智能分配对话框组件
  - 位置：`src/components/vibe-coding/AIAssignStoriesDialog.tsx`
  - 功能：提供AI分析和推荐的完整交互界面
  - 🆕 **新增状态**：`aiThinkingProcess` - 存储AI实时思考过程
  - 🆕 **新增函数**：`formatThinkingProcess()` - 格式化AI输出，增强可读性

### 实时显示实现

```typescript
// 1. 添加状态管理
const [aiThinkingProcess, setAiThinkingProcess] = useState<string>('')

// 2. 在流式接收时实时更新
const unlistenChunk = await listen<{ content: string }>('ai-stream-chunk', event => {
  accumulatedContent += event.payload.content
  // 去除JSON代码块标记，提取纯文本
  const displayContent = accumulatedContent
    .replace(/```json\n?/g, '')
    .replace(/```\n?/g, '')
    .trim()
  setAiThinkingProcess(displayContent)
})

// 3. 格式化显示
const formatThinkingProcess = (text: string) => {
  return text
    .replace(/^(分析|考虑|评估|推荐|总结).*/gm, match => `\n🔍 ${match}`)
    .replace(/(P[0-3]|优先级)/g, '⚡ $1')
    .replace(/(\d+)\s*点/g, '📊 $1 点')
    .replace(/(Sprint|容量|目标)/g, '🎯 $1')
}

// 4. UI渲染
{isAnalyzing && aiThinkingProcess && (
  <Card className="flex-shrink-0 border-primary/30 bg-primary/5">
    <CardHeader>
      <CardTitle>AI分析过程</CardTitle>
    </CardHeader>
    <CardContent>
      <ScrollArea className="h-[200px]">
        <div className="font-mono">
          {formatThinkingProcess(aiThinkingProcess)}
          <span className="animate-pulse" /> {/* 光标效果 */}
        </div>
      </ScrollArea>
    </CardContent>
  </Card>
)}
```

### 集成点

- **SprintManager.tsx**: Sprint管理主组件
  - 添加了`handleOpenAIAssignDialog`函数：打开AI分配对话框
  - 添加了`handleExecuteAIAssign`函数：执行实际的分配操作
  - 在操作列添加了Sparkles图标按钮

### AI Prompt设计

系统会自动构建详细的Prompt发送给AI，包含：
- Sprint基本信息（名称、目标、时间范围、容量）
- 所有未分配的用户故事列表（标题、描述、优先级、故事点等）
- 分析要求（考虑优先级、容量、依赖关系、业务价值等因素）
- 返回格式要求（JSON格式，包含storyId、reason、confidence）

### 数据流

```
// 1. 用户触发
handleOpenAIAssignDialog(sprint)

// 2. AI分析（流式输出）
invoke('stream_chat', {
  request: {
    provider: activeConfig.provider,
    model: activeConfig.model,
    api_key: activeConfig.apiKey,
    messages: [...],
    temperature: 0.7,
    max_tokens: 4000,
  },
})

// 3. 实时接收并显示思考过程
listen('ai-stream-chunk', event => {
  setAiThinkingProcess(formatDisplayContent(event.payload.content))
})

// 4. 解析AI返回的JSON
const result = JSON.parse(jsonString)
setRecommendations(result.recommendations)

// 5. 执行分配
handleExecuteAIAssign(sprintId, storyIds)
→ 批量更新 user_stories.sprint_id
→ 刷新Sprint和用户故事列表
```

## 实时显示特性

### 视觉效果
- 🎨 **主题色边框**：使用primary颜色的半透明边框，突出显示
- ✨ **脉冲动画**：标题图标和光标都有pulse动画，表示正在处理
- 📝 **等宽字体**：使用font-mono确保文本对齐，类似终端输出
- 🔄 **自动滚动**：ScrollArea自动滚动到最新内容
- 💫 **闪烁光标**：末尾的光标模拟打字机效果

### 内容格式化
- 🔍 **分析步骤标记**：自动识别"分析"、"考虑"等关键词，添加放大镜图标
- ⚡ **优先级高亮**：P0-P3优先级标记闪电图标
- 📊 **故事点标记**：数字+点的组合添加图表图标
- 🎯 **Sprint关键词**：Sprint、容量、目标等词添加靶心图标

### 用户体验提升
1. **透明度**：用户可以清楚看到AI是如何分析的，增加信任感
2. **教育性**：通过观察AI的思考过程，学习Sprint规划的最佳实践
3. **即时反馈**：不需要等待分析完成，实时看到进展
4. **可中断性**：如果AI的分析方向不对，可以提前停止

## 错误处理与显示

### 🆕 增强的错误显示（2026-04-13）

#### UI优化
- **醒目的错误卡片**：使用destructive颜色的边框和背景
- **独立的错误区域**：带有标题"AI分析出错"和AlertCircle图标
- **滚动支持**：长错误信息可以滚动查看，最大高度150px
- **操作按钮**：
  - "关闭"按钮：清除错误信息
  - "重新分析"按钮：一键重试，带加载状态

#### 智能错误分类
系统会自动识别并格式化常见错误类型：

```
// CodeFree CLI 错误（新增）
❌ CodeFree CLI 未配置认证信息

请在以下位置之一配置认证：
1. 配置文件：C:\Users\37844\.codefree-cli\settings.json
2. 环境变量：设置 CodeFree-oauth

// 认证错误
❌ API密钥无效，请检查配置
❌ 认证失败，请检查API密钥是否正确

// 配额错误
⚠️ API配额已用尽，请检查账户余额
⚠️ API调用频率超限，请稍后重试
⚠️ 账户余额不足，请充值

// 模型错误
❌ 模型不存在，请检查模型名称
❌ 无效的模型名称

// 网络错误
🌐 网络连接失败，请检查网络设置
⏱️ 请求超时，请检查网络连接后重试
🌐 连接被拒绝，请检查网络或代理设置

// HTTP状态码错误
❌ HTTP 401: 认证失败，请检查API密钥
❌ HTTP 429: 请求频率超限
❌ HTTP 500: 服务器内部错误

// 默认错误
❌ AI分析失败
[原始错误信息]
```

#### 错误处理流程
```
// 1. 捕获流式错误事件
listen('ai-stream-error', event => {
  const errorMessage = formatErrorMessage(event.payload.error)
  setError(errorMessage)
  setIsAnalyzing(false)
})

// 2. 捕获invoke异常
catch (err) {
  const errorMessage = formatErrorMessage(err.message)
  setError(errorMessage)
  setIsAnalyzing(false)
}

// 3. 格式化错误信息
const formatErrorMessage = (error: string): string => {
  // 匹配已知错误类型
  // 提取HTTP状态码
  // 返回友好的错误提示
}

// 4. UI显示
{error && (
  <Card className="border-destructive bg-destructive/5">
    <CardHeader>AI分析出错</CardHeader>
    <CardContent>
      <ScrollArea>{error}</ScrollArea>
      <Button onClick={() => setError(null)}>关闭</Button>
      <Button onClick={handleAIAnalysis}>重新分析</Button>
    </CardContent>
  </Card>
)}
```

#### 用户体验优势
1. **清晰的错误分类**：使用emoji图标区分错误类型（❌错误、⚠️警告、🌐网络、⏱️超时）
2. ** actionable提示**：每个错误都包含具体的解决建议
3. **保留原始信息**：在友好提示下方显示原始错误，便于调试
4. **快速重试**：一键重新分析，无需关闭对话框
5. **详细日志**：控制台输出完整错误信息，方便开发调试

### 示例场景

#### 场景1：API密钥错误
```
用户看到：
❌ API密钥无效，请检查配置

原始错误：Invalid API key provided

操作：
1. 点击"关闭"清除错误
2. 前往设置页面检查API密钥
3. 重新打开对话框再次尝试
```

#### 场景2：网络超时
```
用户看到：
⏱️ 请求超时，请检查网络连接后重试

原始错误：Request timeout after 30000ms

操作：
1. 检查网络连接
2. 点击"重新分析"按钮重试
```

#### 场景3：配额不足
```
用户看到：
⚠️ API配额已用尽，请检查账户余额

原始错误：You exceeded your current quota

操作：
1. 登录AI服务提供商网站
2. 检查账户余额和配额使用情况
3. 充值或等待配额重置
```

#### 场景4：JSON解析失败
```
用户看到：
❌ AI分析失败

原始错误：Unexpected token in JSON at position 123

操作：
1. 点击"重新分析"重试
2. 如果持续失败，可能是AI返回格式问题
3. 查看控制台日志获取详细信息
```

#### 场景5：CodeFree CLI认证错误（新增）
```
用户看到：
❌ CodeFree CLI 未配置认证信息

请在以下位置之一配置认证：
1. 配置文件：C:\Users\37844\.codefree-cli\settings.json
2. 环境变量：设置 CodeFree-oauth

详细错误：CodeFree CLI 错误：Please set an Auth method...

操作：
1. 打开文件资源管理器，导航到 C:\Users\37844\.codefree-cli\
2. 编辑 settings.json 文件，添加认证配置
3. 或者设置系统环境变量 CodeFree-oauth
4. 完成后点击"重新分析"按钮重试
```

## 注意事项

1. **API消耗**：AI分析会消耗API Token，请合理使用
2. **推荐质量**：推荐结果基于多个因素综合评估，建议仔细查看推荐理由
3. **容量限制**：系统会提示选中的故事点是否超出Sprint剩余容量
4. **错误处理**：如果AI返回格式不正确或网络超时，会显示友好错误提示
5. **数据安全**：分配操作会立即生效，建议先预览推荐结果再确认
6. **🆕 性能优化**：实时显示不会影响AI分析速度，采用异步更新机制

## 示例场景

### 场景1：新Sprint规划
```
背景：创建了一个新的Sprint，目标是实现用户认证功能
操作：
1. 点击Sparkles按钮
2. 🆕 实时看到AI分析："🔍 分析Sprint目标：用户认证功能"
3. 🆕 继续看到："⚡ 发现3个P0优先级的认证相关故事"
4. 🆕 看到容量评估："📊 预计需要21点，🎯 Sprint剩余容量25点"
5. AI推荐与认证相关的高优先级故事
6. 查看推荐理由（如："这个故事实现了登录功能，与Sprint目标高度匹配，优先级P0"）
7. 确认分配
```

### 场景2：容量优化
```
背景：Sprint剩余容量有限，需要精选故事
操作：
1. 🆕 实时观察AI的容量计算过程
2. AI根据剩余容量智能筛选
3. 优先推荐高价值、低故事点的故事
4. 考虑故事间的依赖关系
5. 确保总故事点不超过容量限制
```

### 场景3：学习AI决策逻辑
```
背景：想了解AI是如何做Sprint规划的
操作：
1. 启动AI分析
2. 🆕 仔细观察AI的思考过程：
   - 如何评估优先级
   - 如何权衡业务价值
   - 如何处理依赖关系
   - 如何优化容量分配
3. 学习敏捷开发的最佳实践
```

## 未来优化方向

1. **历史记录**：保存AI推荐历史，便于追溯和对比
2. **手动调整权重**：允许用户调整AI分析的权重因子（优先级、故事点、依赖等）
3. **多轮对话**：支持用户对推荐结果提出质疑，AI重新分析
4. **批量分析**：一次性为多个Sprint进行AI分配
5. **学习优化**：根据用户最终选择优化推荐算法
6. **🆕 思考过程导出**：允许用户导出AI的分析过程作为文档
7. **🆕 自定义格式化规则**：允许用户自定义关键词高亮规则
8. **🆕 分析速度控制**：提供慢速/正常/快速模式，方便阅读
