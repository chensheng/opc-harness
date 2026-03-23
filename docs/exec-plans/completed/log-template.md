# 执行日志模板

## 日志格式规范

### 前端日志 (TypeScript)

```typescript
// 日志级别和格式
console.log('[INFO] 一般信息');
console.warn('[WARN] 警告信息');
console.error('[ERROR] 错误信息');
console.debug('[DEBUG] 调试信息');

// 结构化日志
console.group('[上下文] 操作名称');
console.log('[PARAMS] 参数名:', 值);
console.log('[STATE] 状态名:', 状态值);

try {
  // 业务逻辑
  console.log('[ACTION] 执行操作...');
  const result = await operation();
  
  console.log('[RESULT] 操作成功:', result);
  console.groupEnd();
} catch (error) {
  console.error('[ERROR] 操作失败:', error);
  console.groupEnd();
  throw error;
}
```

### 后端日志 (Rust)

```rust
use log::{debug, info, warn, error};

// 日志级别
debug!("调试信息");
info!("一般信息");
warn!("警告信息");
error!("错误信息");

// 结构化日志
info!(target: "project_management", "创建项目"; 
      "name" => &project_name,
      "user_id" => user_id);

match operation() {
    Ok(result) => {
        info!("操作成功：result_count={}", result.len());
        Ok(result)
    }
    Err(e) => {
        error!("操作失败：{}", e);
        Err(format!("操作失败：{}", e))
    }
}
```

## 日志记录场景

### 1. Tauri 命令调用

**前端**:
```typescript
async function handleProjectCreation(params: CreateProjectParams) {
  console.group('[Tauri Command] create_project');
  console.log('[REQUEST]', params);
  
  try {
    const result = await invoke('create_project', params);
    console.log('[RESPONSE]', result);
    console.log('[SUCCESS] 项目创建成功，ID:', result.id);
    console.groupEnd();
    return result;
  } catch (error) {
    console.error('[FAILURE] 项目创建失败');
    console.error('[ERROR DETAILS]', error);
    console.groupEnd();
    throw error;
  }
}
```

**后端**:
```rust
#[tauri::command]
pub async fn create_project(
    name: String,
    description: String,
) -> Result<Project, String> {
    info!(target: "commands", "收到项目创建请求"; 
          "name" => &name,
          "description_length" => description.len());
    
    // 验证参数
    if name.trim().is_empty() {
        warn!("项目名称为空");
        return Err("项目名称不能为空".to_string());
    }
    
    // 创建项目
    match ProjectService::create(&name, &description).await {
        Ok(project) => {
            info!(target: "commands", "项目创建成功"; 
                  "project_id" => project.id);
            Ok(project)
        }
        Err(e) => {
            error!(target: "commands", "项目创建失败：{}", e);
            Err(format!("创建失败：{}", e))
        }
    }
}
```

### 2. AI API 调用

```typescript
async function callAI(prompt: string, provider: string) {
  const startTime = performance.now();
  console.group(`[AI] ${provider}`);
  console.log('[PROMPT]', prompt.substring(0, 100) + '...');
  console.log('[CONFIG]', { provider, model: config.model });
  
  try {
    const response = await fetchAI(prompt);
    const endTime = performance.now();
    const duration = endTime - startTime;
    
    console.log('[RESPONSE]', response.content.substring(0, 100) + '...');
    console.log(`[PERFORMANCE] 耗时：${duration.toFixed(2)}ms`);
    console.groupEnd();
    
    return response;
  } catch (error) {
    const endTime = performance.now();
    console.error('[FAILURE] AI 调用失败');
    console.error('[ERROR]', error);
    console.log(`[PERFORMANCE] 失败耗时：${(endTime - startTime).toFixed(2)}ms`);
    console.groupEnd();
    throw error;
  }
}
```

### 3. 数据库操作

```rust
async fn query_projects(
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Project>> {
    debug!(target: "db", "查询项目列表";
           "status" => status.unwrap_or("all"),
           "limit" => limit,
           "offset" => offset);
    
    let start = Instant::now();
    
    let query = "SELECT * FROM projects WHERE (? IS NULL OR status = ?) LIMIT ? OFFSET ?";
    let projects = sqlx::query_as::<_, Project>(query)
        .bind(status)
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(&pool)
        .await?;
    
    let duration = start.elapsed();
    info!(target: "db", "项目查询完成";
          "count" => projects.len(),
          "duration_ms" => format!("{:?}", duration));
    
    Ok(projects)
}
```

### 4. 错误处理

```typescript
class ErrorHandler {
  static handle(error: unknown, context: string) {
    console.group(`[ERROR HANDLER] ${context}`);
    
    if (error instanceof Error) {
      console.error('[TYPE]', error.constructor.name);
      console.error('[MESSAGE]', error.message);
      console.error('[STACK]', error.stack);
      
      // 添加额外上下文
      console.error('[CONTEXT]', {
        timestamp: new Date().toISOString(),
        userAgent: navigator.userAgent,
        url: window.location.href,
      });
    } else {
      console.error('[UNKNOWN ERROR]', error);
    }
    
    console.groupEnd();
    
    // 上报错误监控（生产环境）
    if (process.env.NODE_ENV === 'production') {
      this.reportToMonitoring(error, context);
    }
  }
}
```

## 日志级别使用指南

| 级别 | 用途 | 示例 |
|------|------|------|
| DEBUG | 详细调试信息，默认关闭 | 函数入参、中间状态 |
| INFO | 一般运行信息 | 操作开始/结束、用户行为 |
| WARN | 警告但不影响运行 | 使用了废弃 API、配置缺失 |
| ERROR | 严重错误需要处理 | 操作失败、数据异常 |

## 日志最佳实践

### ✅ 推荐

1. **结构化**: 使用一致的标签格式 `[TAG] message`
2. **上下文**: 提供足够的上下文信息定位问题
3. **性能**: 记录关键操作耗时
4. **脱敏**: 敏感信息（密码、token）要脱敏
5. **分组**: 使用 `console.group()` 组织相关日志

### ❌ 避免

1. **过度日志**: 不要在循环中大量打印
2. **模糊信息**: "出错了" 这样的信息没用
3. **生产环境**: 生产环境避免输出 DEBUG 日志
4. **敏感数据**: 不要打印完整密码、API Key

## 日志分析工具

### 开发环境
- Chrome DevTools Console
- Rust env_logger / tracing

### 生产环境（未来）
- Sentry（前端错误追踪）
- ELK Stack（日志聚合分析）

---

**模板版本**: 1.0.0  
**最后更新**: 2026-03-22
