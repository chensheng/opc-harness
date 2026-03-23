# SQLite 数据库功能演示

本脚本演示如何使用 INFRA-008 实现的数据库功能。

## 前置条件

1. 确保项目已启动：`npm run tauri:dev`
2. 打开浏览器控制台（F12）

## 演示步骤

### 1. 创建项目

```javascript
// 创建第一个项目
const projectId1 = await invoke('create_project', {
  name: 'AI 助手',
  description: '基于 AI 的智能助手应用'
});
console.log('✅ 创建项目 1:', projectId1);

// 创建第二个项目
const projectId2 = await invoke('create_project', {
  name: '数据分析平台',
  description: '可视化数据分析工具'
});
console.log('✅ 创建项目 2:', projectId2);
```

### 2. 获取所有项目

```javascript
const projects = await invoke('get_all_projects');
console.log('📋 所有项目:', projects);
console.table(projects.map(p => ({
  名称：p.name,
  状态：p.status,
  进度：p.progress + '%',
  创建时间：new Date(p.createdAt).toLocaleString()
})));
```

### 3. 获取单个项目

```javascript
const project = await invoke('get_project_by_id', { id: projectId1 });
console.log('📄 项目详情:', project);
```

### 4. 更新项目

```javascript
// 更新项目状态
project.status = 'design';
project.progress = 25;
await invoke('update_project', { project });
console.log('✅ 项目已更新');

// 验证更新
const updatedProject = await invoke('get_project_by_id', { id: projectId1 });
console.log('📄 更新后的项目:', updatedProject);
```

### 5. 保存 AI 配置

```javascript
// 保存 OpenAI 配置
await invoke('save_ai_config', {
  config: {
    provider: 'openai',
    model: 'gpt-4o',
    apiKey: 'sk-test-key-123'
  }
});
console.log('✅ OpenAI 配置已保存');

// 保存 Kimi 配置
await invoke('save_ai_config', {
  config: {
    provider: 'kimi',
    model: 'kimi-k1.5',
    apiKey: 'kimi-test-key-456'
  }
});
console.log('✅ Kimi 配置已保存');
```

### 6. 获取 AI 配置

```javascript
const allConfigs = await invoke('get_all_ai_configs');
console.log('🔧 所有 AI 配置:', allConfigs);

const openaiConfig = await invoke('get_ai_config', { provider: 'openai' });
console.log('🔧 OpenAI 配置详情:', openaiConfig);
```

### 7. 删除配置

```javascript
// 删除测试数据
await invoke('delete_ai_config', { provider: 'kimi' });
console.log('🗑️ Kimi 配置已删除');

await invoke('delete_project', { id: projectId2 });
console.log('🗑️ 项目 2 已删除');
```

### 8. 完整工作流示例

```javascript
async function demoWorkflow() {
  console.log('🚀 开始完整工作流演示...\n');
  
  // Step 1: 创建项目
  console.log('Step 1: 创建项目');
  const project = await invoke('create_project', {
    name: 'Vibe Coding 平台',
    description: 'AI 驱动的编码平台'
  });
  console.log('✅ 项目创建成功:', project, '\n');
  
  // Step 2: 配置 AI Provider
  console.log('Step 2: 配置 AI Provider');
  await invoke('save_ai_config', {
    config: {
      provider: 'openai',
      model: 'gpt-4o',
      apiKey: 'your-api-key-here'
    }
  });
  console.log('✅ AI 配置已保存\n');
  
  // Step 3: 读取并显示
  console.log('Step 3: 读取配置');
  const config = await invoke('get_ai_config', { provider: 'openai' });
  console.log('📖 AI 配置:', config, '\n');
  
  // Step 4: 清理测试数据
  console.log('Step 4: 清理测试数据');
  await invoke('delete_ai_config', { provider: 'openai' });
  await invoke('delete_project', { id: project });
  console.log('✅ 测试数据已清理\n');
  
  console.log('🎉 工作流演示完成！');
}

// 运行演示
await demoWorkflow();
```

## 预期输出

```
🚀 开始完整工作流演示...

Step 1: 创建项目
✅ 项目创建成功：xxx-xxx-xxx 

Step 2: 配置 AI Provider
✅ AI 配置已保存

Step 3: 读取配置
📖 AI 配置：{ provider: 'openai', model: 'gpt-4o', apiKey: 'your-api-key-here' } 

Step 4: 清理测试数据
✅ 测试数据已清理

🎉 工作流演示完成！
```

## 调试技巧

1. **查看数据库文件**:
   - Windows: `%APPDATA%\opc-harness\opc-harness.db`
   - 使用 DB Browser for SQLite 打开查看

2. **检查错误**:
   ```javascript
   try {
     const result = await invoke('some_command');
     console.log(result);
   } catch (error) {
     console.error('❌ 错误:', error);
   }
   ```

3. **验证数据格式**:
   ```javascript
   const projects = await invoke('get_all_projects');
   console.log('字段名格式:', Object.keys(projects[0]));
   // 应该输出 camelCase 字段：['id', 'name', 'createdAt', 'updatedAt', ...]
   ```

## 常见问题

**Q: invoke 未定义？**
A: 确保已导入：`import { invoke } from '@tauri-apps/api/core'`

**Q: 字段名不匹配？**
A: Rust 后端会自动转换为 camelCase，前端应使用 `createdAt` 而非 `created_at`

**Q: 数据库在哪里？**
A: Windows 上位于 `%APPDATA%\opc-harness\opc-harness.db`

## 下一步

- 在前端 Settings 页面集成 AI 配置管理
- 在 Dashboard 页面显示项目列表
- 在项目详情页实现编辑功能
