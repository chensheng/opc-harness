# Harness Engineering 标准开发流程与规范

**版本**: v3.0  
**最后更新**: 2026-03-24  
**维护者**: OPC-HARNESS Team  

---

## 📋 核心开发流程（7 阶段）

### 1️⃣ 任务选择
- **来源**: MVP 规划文档 (`docs/exec-plans/active/MVP版本规划.md`)
- **优先级**: P0 > P1 > P2
- **原则**: 选择关键路径上独立性强、可验证的任务

### 2️⃣ 开发实施

#### 架构约束
- ✅ 开发前查阅 `architecture-rules.md`
- ✅ 遵守前后端分层及依赖规范
- ❌ 严禁循环依赖

#### 后端 (Rust)
```rust
// 必须包含：完整类型定义、错误处理、日志记录
pub struct MyStruct {
    pub field: String,
}

impl MyStruct {
    pub fn new(value: &str) -> Result<Self, String> {
        // 错误处理
    }
}
```

**要点**:
- 遇到模块错误检查 `mod` 声明及 `Cargo.toml` 依赖
- 使用 snake_case 命名（序列化后保持）

#### 前端 (TypeScript/React)
```typescript
// 必须包含：类型安全、Hooks 封装、路径别名
import { useAgent } from '@/hooks/useAgent';
import type { AgentConfig } from '@/types/agent';
```

**要点**:
- Hooks 封装 IPC 调用
- 使用 `@/` 路径别名
- 严格 TypeScript 模式 (`strict: true`)

### 3️⃣ 单元测试

#### Rust 后端测试
```bash
# 开发调试时使用（查看详细错误）
cd src-tauri
cargo test --bin opc-harness -- --nocapture

# 提交前验证：无需手动运行！harness:check 会自动执行
```

**要求**:
- ✅ 所有功能代码必须包含单元测试
- ✅ 新实现的功能测试覆盖率 ≥70%
- ✅ 测试类型：结构体创建、方法逻辑、序列化、错误处理

**示例**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_struct_creation() {
        let instance = MyStruct::new("test").unwrap();
        assert_eq!(instance.field, "test");
    }
}
```

#### TypeScript 前端测试
```bash
# 开发调试时使用（查看详细错误）
npm run test:unit -- --reporter=verbose

# 提交前验证：无需手动运行！harness:check 会自动执行
```

**要求**:
- ✅ Hooks 测试覆盖所有自定义 Hooks
- ✅ Store 测试覆盖状态管理逻辑
- ✅ 工具函数测试
- ✅ 组件渲染测试
- ✅ 新实现的功能测试覆盖率 ≥70%

**环境处理**:
- 若出现 `ECONNREFUSED`（VectorDB 端口 1420），标记为环境问题
- Harness 检查会自动识别，仅扣 5 分（警告级别）
- 解决方案：启动 VectorDB / 使用 Mock / 跳过集成测试

### 4️⃣ 端到端测试 (E2E)
- **时机**: 单元测试通过后
- **范围**: 覆盖核心用户流程
- **命令**: `npm run test:e2e`

### 5️⃣ 质量验证 ⭐

运行完整的 Harness Engineering 健康检查：

```bash
npm run harness:check
```

**目标**: Health Score 100/100

#### 检查项（8 步）

| # | 检查项 | 失败扣分 | 说明 |
|---|--------|---------|------|
| 1 | TypeScript 类型检查 | -20 | 编译时验证 |
| 2 | ESLint 代码质量 | -15 | 代码风格 |
| 3 | Prettier 格式 | -10 | 格式化规范 |
| 4 | Rust 编译检查 | -25 | 编译验证 |
| 5 | **Rust 单元测试** | -20 | 自动运行 `cargo test` |
| 6 | **TypeScript 单元测试** | -20 | 自动运行 `npm run test:unit` |
| 7 | 依赖完整性 | -5 | 文件检查 |
| 8 | 目录结构 | -5 | 结构验证 |

**可选检查**（使用 `-All` 参数）:
- 文档一致性检查 (-5 分)
- 死代码检测 (-5 分)

#### TypeScript 测试智能识别

**真实测试失败**（扣 20 分，阻塞发布）:
- 断言失败
- 逻辑错误
- 组件渲染失败

**环境问题**（扣 5 分，警告）:
- `ECONNREFUSED ::1:1420` / `127.0.0.1:1420`
- VectorDB 未启动
- 非代码缺陷

**执行异常**（扣 10 分）:
- 超时（>30 秒）
- npm/node_modules 不可用

#### 评分标准

```
90-100 分：[EXCELLENT] 优秀，可发布
70-89 分： [GOOD] 良好，需改进
<70 分：   [NEEDS FIX] 需要立即修复
```

### 6️⃣ 文档更新

#### 必须完成的文档
1. **MVP 规划更新**: 标记任务状态为已完成
2. **任务完成报告**: 创建详细报告（模板见下方）

#### 任务完成报告模板
```markdown
# {任务 ID} 任务完成报告 - {任务名称}

## 📋 任务概述
- 任务 ID: {ID}
- 优先级: P{0/1/2}
- 状态: ✅ 已完成

## 🎯 验收标准
- [ ] 标准 1
- [ ] 标准 2

## 💻 技术实现
{核心代码片段}

## 🧪 测试结果
- Rust 测试：X/X 通过
- TS 测试：X/X 通过
- Health Score: 100/100

## ✅ 合规性声明
- ✅ Harness Engineering 7 阶段流程遵循
- ✅ 代码规范遵循
- ✅ 测试覆盖率达标
```

#### 归档规范
- ✅ 已完成报告移至 `docs/exec-plans/completed/`
- ❌ `active/` 仅保留进行中任务
- ✅ 移动后验证文件存在

### 7️⃣ 完成交付

```bash
# 1. 确认 Health Score ≥ 70
npm run harness:check

# 2. Git 提交
git add .
git commit -m "{任务 ID}: {描述}"

# 3. 打标签（可选）
git tag -a v{版本号} -m "{描述}"
```

---

## 🔧 常用命令速查

### 开发调试
```bash
# Rust 测试（快速定位错误）
cd src-tauri && cargo test --bin opc-harness -- --nocapture

# TS 测试（详细输出）
npm run test:unit -- --reporter=verbose

# 格式化代码
npm run format

# 修复 ESLint 问题
npm run lint:fix
```

### 提交前验证
```bash
# 唯一需要的命令（包含所有检查）
npm run harness:check

# 详细输出模式
npm run harness:check -- -Verbose
```

### CI/CD 集成
```yaml
# GitHub Actions 示例
- name: Harness Engineering Health Check
  run: npm run harness:check
  
- name: Upload Test Results
  uses: actions/upload-artifact@v3
  with:
    name: test-results
    path: test-results/
```

---

## 🎯 最佳实践

### ✅ 推荐做法
1. **测试驱动**: 编写功能前先写测试
2. **小步快跑**: 频繁运行 `harness:check`（而非手动运行多个测试）
3. **一次通过**: 提交前只运行一次 `harness:check`
4. **文档同步**: 代码完成后立即更新文档

### ❌ 避免的做法
1. ❌ 重复测试：`npm run test:unit && cargo test && npm run harness:check`
2. ❌ 跳过测试：直接提交而不运行质量检查
3. ❌ 注释漂移：代码修改后不更新文档
4. ❌ 硬编码：API Key 等敏感信息写入代码

---

## 🐛 常见问题处理

### Rust 相关问题

**Q1: 编译成功但测试失败？**
```bash
# 查看详细错误
cargo test --bin opc-harness -- --nocapture

# 检查序列化格式（snake_case vs camelCase）
assert!(json.contains("\"user_name\"")); // ✅ 正确
assert!(json.contains("\"userName\""));  // ❌ 错误
```

**Q2: 模块未找到？**
```rust
// 检查 main.rs 或 mod.rs 中的模块声明
pub mod my_module;

// 检查 Cargo.toml 依赖
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

### TypeScript 相关问题

**Q1: ECONNREFUSED 错误？**
```typescript
// 方案 1: 启动 VectorDB 服务
// 方案 2: 使用 Mock 数据
const mockData = { /* ... */ };

// 方案 3: 跳过特定测试
it.skip('requires database', () => { /* ... */ });
```

**Q2: ESLint 报错未使用变量？**
```typescript
// 移除未使用的导入
import { unused } from 'module'; // ❌ 删除

// Catch 块未用参数命名为 _error
try { /* ... */ } catch (_error) { /* ... */ }
```

### Harness 脚本问题

**Q1: PowerShell 执行策略错误？**
```powershell
# 使用 Bypass 参数
powershell -ExecutionPolicy Bypass -File ./scripts/harness-check.ps1
```

**Q2: 命令分隔符错误（Windows）？**
```bash
# 使用分号分隔
cd src-tauri; cargo test
```

---

## 📊 质量指标

### 测试覆盖率要求

| 模块 | 最低覆盖率 | 目标覆盖率 |
|------|-----------|-----------|
| Rust 后端 | 70% | 85%+ |
| TS 前端 | 70% | 85%+ |
| 核心 Hooks | 80% | 95%+ |
| Store | 80% | 95%+ |

### Health Score 权重分布

```
TypeScript 类型错误：-20 分
ESLint 错误：         -15 分
Prettier 错误：       -10 分
Rust 编译错误：       -25 分
Rust 测试失败：       -20 分
TS 测试失败（真实）：  -20 分
TS 测试失败（环境）：   -5 分
其他警告：            -5~10 分
```

---

## 🚀 持续改进

### 已实施的优化
- ✅ 自动化测试验证（Rust + TS）
- ✅ 智能错误识别（区分环境和代码问题）
- ✅ 超时保护机制（30 秒）
- ✅ 消除重复测试环节

### 下一步计划
1. 增加测试覆盖率门槛检查（<70% 扣分）
2. 集成 E2E 测试到 Harness 检查
3. 性能基准测试
4. Mock 基础设施统一层

---

## 📚 相关文档

- **MVP 规划**: `docs/exec-plans/active/MVP版本规划.md`
- **架构规则**: `docs/architecture-rules.md`
- **任务报告**: `docs/exec-plans/completed/task-completion-*.md`
- **改进报告**: `docs/exec-plans/completed/harness-engineering-*.md`

---

**文档历史**:
- v3.0 (2026-03-24): 整合精简版，合并所有 Harness Engineering 相关文档
- v2.0 (2026-03-24): 补充 Rust 和 TypeScript 单元测试
- v1.0 (早期版本): 基础流程定义

**变更说明**:
- 移除了重复的测试环节，明确 `harness:check` 为唯一提交前验证
- 简化了文档结构，将所有重要信息整合到单一文档
- 添加了快速参考和常见问题处理章节
