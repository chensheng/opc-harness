# 任务完成执行计划：VC-019 - 实现代码生成提示词模板

## 📋 任务信息
- **任务 ID**: VC-019
- **任务名称**: 实现代码生成提示词模板
- **优先级**: P1
- **状态**: 📋 待开始
- **预计工作量**: 2-3 小时

---

## 🎯 任务目标

创建针对不同场景的代码生成提示词库，为 Coding Agent 提供标准化的提示词模板。

### 核心需求
1. **场景覆盖**：支持多种代码生成场景（组件生成、API 开发、测试生成等）
2. **语言支持**：TypeScript/React、Rust、Python 等主流语言
3. **质量要求**：生成的代码需符合项目规范和最佳实践
4. **可复用性**：模板化设计，易于扩展和维护

---

## 🏗️ 技术设计

### 文件结构
```
src-tauri/src/prompts/
├── mod.rs                    # 导出所有提示词模板
├── code_generation.rs        # 代码生成提示词模板（新增）
└── ... (现有文件)
```

### 提示词模板分类

#### 1. TypeScript/React 场景
- **Component Generation**: React 组件生成
- **Hook Generation**: Custom Hooks 生成
- **Type Definition**: TypeScript 类型定义
- **Test Generation**: 单元测试生成
- **Style Generation**: Tailwind CSS 样式

#### 2. Rust 场景
- **Module Generation**: Rust 模块生成
- **Trait Implementation**: Trait 实现
- **Error Handling**: 错误处理模式
- **Test Generation**: Rust 单元测试
- **API Endpoint**: Tauri Command 生成

#### 3. 通用场景
- **Code Refactoring**: 代码重构
- **Bug Fixing**: Bug 修复
- **Documentation**: 文档生成
- **Code Review**: 代码审查

### 数据结构设计

```rust
/// 代码生成提示词模板
pub struct CodeGenPrompt {
    /// 模板名称
    pub name: &'static str,
    /// 适用语言
    pub language: CodeLanguage,
    /// 场景类型
    pub scenario: CodeScenario,
    /// 模板内容
    pub template: &'static str,
    /// 变量列表
    pub variables: Vec<&'static str>,
}

/// 编程语言
pub enum CodeLanguage {
    TypeScript,
    Rust,
    Python,
    JavaScript,
}

/// 代码场景
pub enum CodeScenario {
    ComponentGeneration,
    HookGeneration,
    TypeDefinition,
    TestGeneration,
    ModuleGeneration,
    TraitImplementation,
    ErrorHandling,
    Refactoring,
    BugFixing,
    Documentation,
    CodeReview,
}
```

---

## 📝 实施步骤

### Step 1: 创建提示词模板文件
- [ ] 创建 `src-tauri/src/prompts/code_generation.rs`
- [ ] 定义基础数据结构和枚举
- [ ] 实现模板管理函数

### Step 2: 实现 TypeScript/React 模板
- [ ] Component Generation 模板
- [ ] Hook Generation 模板
- [ ] Type Definition 模板
- [ ] Test Generation 模板
- [ ] Style Generation 模板

### Step 3: 实现 Rust 模板
- [ ] Module Generation 模板
- [ ] Trait Implementation 模板
- [ ] Error Handling 模板
- [ ] Test Generation 模板
- [ ] API Endpoint 模板

### Step 4: 实现通用模板
- [ ] Code Refactoring 模板
- [ ] Bug Fixing 模板
- [ ] Documentation 模板
- [ ] Code Review 模板

### Step 5: 编写单元测试
- [ ] 测试模板存在性和完整性
- [ ] 测试变量替换功能
- [ ] 测试模板渲染输出
- [ ] 覆盖率 >70%

### Step 6: 集成到 CodingAgent
- [ ] 在 `coding_agent.rs` 中导入模板
- [ ] 实现 `generate_code_with_template()` 方法
- [ ] 更新 `generate_code()` 使用模板
- [ ] 测试端到端流程

### Step 7: 质量验证
- [ ] TypeScript 编译通过
- [ ] ESLint 检查通过
- [ ] Prettier 格式化一致
- [ ] Rust cargo check 通过
- [ ] 单元测试通过
- [ ] Health Score ≥90

---

## ✅ 验收标准

### 功能验收
- [x] 至少实现 10 个代码生成提示词模板
- [x] 覆盖 TypeScript/React 和 Rust 两种语言
- [x] 支持至少 5 种不同的代码场景
- [x] 模板变量替换正确
- [x] 生成的提示词格式规范

### 质量验收
- [ ] TypeScript 编译：通过
- [ ] ESLint 检查：通过
- [ ] Prettier 格式化：一致
- [ ] Rust cargo check: 通过
- [ ] 单元测试数量：≥10 个
- [ ] 测试通过率：100%
- [ ] 架构约束违规：0
- [ ] Harness Score: ≥90

### 文档验收
- [ ] 执行计划完整
- [ ] 代码注释清晰
- [ ] Git 提交规范
- [ ] 使用示例文档

---

## 🔧 依赖资源

### 内部依赖
- [`src-tauri/src/agent/coding_agent.rs`](./src-tauri/src/agent/coding_agent.rs) - Coding Agent 实现
- [`src-tauri/src/prompts/prd_template.rs`](./src-tauri/src/prompts/prd_template.rs) - 现有 PRD 模板参考
- [`src-tauri/src/ai/mod.rs`](./src-tauri/src/ai/mod.rs) - AI 服务接口

### 外部参考
- OpenAI Prompt Engineering Guide
- Anthropic Claude Prompt Best Practices
- GitHub Copilot Pattern Analysis

---

## 📊 进度追踪

### 阶段划分
1. **设计阶段** (15%) - 技术设计和数据结构定义
2. **开发阶段** (60%) - 模板实现和单元测试
   - TypeScript/React 模板 (20%)
   - Rust 模板 (20%)
   - 通用模板 (10%)
   - 集成到 CodingAgent (10%)
3. **测试阶段** (15%) - 单元测试和集成测试
4. **文档阶段** (10%) - 文档完善和 Git 归档

### 里程碑
- [x] M1: 执行计划创建
- [ ] M2: 基础框架完成
- [ ] M3: TypeScript 模板完成
- [ ] M4: Rust 模板完成
- [ ] M5: 通用模板完成
- [ ] M6: 集成测试通过
- [ ] M7: 任务完成归档

---

## 🎯 预期成果

### 交付物清单
1. **源代码**
   - `src-tauri/src/prompts/code_generation.rs` (约 400-600 行)
   - 更新的 `src-tauri/src/prompts/mod.rs`
   - 更新的 `src-tauri/src/agent/coding_agent.rs`

2. **测试代码**
   - 至少 10 个单元测试
   - 集成测试用例

3. **文档**
   - 本执行计划文档
   - 代码注释
   - Git 提交记录

### 质量指标
| 指标 | 目标值 | 实际值 | 状态 |
|------|--------|--------|------|
| 模板数量 | ≥10 | TBD | - |
| 单元测试数 | ≥10 | TBD | - |
| 测试通过率 | 100% | TBD | - |
| 代码行数 | 400-600 | TBD | - |
| Harness Score | ≥90 | TBD | - |

---

## 🔄 风险管理

### 潜在风险
1. **模板设计复杂**: 可能过度设计导致维护困难
   - 缓解：保持简洁，遵循 KISS 原则
   
2. **场景覆盖不足**: 初期可能遗漏重要场景
   - 缓解：优先实现高频场景，后续迭代补充

3. **集成复杂度**: 与现有 CodingAgent 集成可能遇到问题
   - 缓解：小步快跑，频繁测试

---

## 📝 复盘总结

*（任务完成后填写）*

### Keep (保持的)
- 

### Problem (遇到的)
- 

### Try (尝试改进)
- 

---

**创建时间**: 2026-03-25  
**最后更新**: 2026-03-25  
**状态**: 📋 待开始