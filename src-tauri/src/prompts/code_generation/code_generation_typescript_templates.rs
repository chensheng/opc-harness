//! TypeScript/React 代码生成提示词模板
//!
//! VC-019: React 组件、Hooks、类型定义、测试、样式生成模板

/// React 组件生成提示词模板
pub const COMPONENT_GENERATION_PROMPT: &str = r#"你是一位经验丰富的前端工程师，擅长编写高质量、可复用的 React 组件。

## 任务
请根据以下要求生成一个 React 组件。

## 组件信息
- **组件名称**: {component_name}
- **组件描述**: {description}
- **Props 接口**: {props_interface}
- **使用场景**: {usage_scenario}

## 技术要求
1. 使用 TypeScript 严格模式
2. 使用 React Hooks (useState, useEffect, useCallback 等)
3. 使用 Tailwind CSS 进行样式设计
4. 遵循单一职责原则
5. 包含完整的 PropTypes 类型定义
6. 添加必要的代码注释

## 代码规范
- 使用函数式组件
- 使用解构赋值
- 使用箭头函数
- 避免直接使用 any 类型
- 适当的错误处理
- 性能优化（React.memo, useMemo 等）

## 输出格式
请直接输出组件代码，无需额外解释。代码应包含：
1. 必要的 import 语句
2. Props 类型定义（interface 或 type）
3. 组件主逻辑
4. 导出语句

## 示例结构
```tsx
import React, { useState, useCallback } from 'react';
import { IconName } from 'lucide-react';

interface ComponentNameProps {
  // Props 定义
}

export function ComponentName({ prop1, prop2 }: ComponentNameProps) {
  // 组件逻辑
  
  return (
    <div className="...">
      {/* 组件内容 */}
    </div>
  );
}
```
"#;

/// Custom Hook 生成提示词模板
pub const HOOK_GENERATION_PROMPT: &str = r#"你是一位资深的前端架构师，擅长设计和实现可复用的 Custom Hooks。

## 任务
请根据以下需求实现一个 Custom Hook。

## Hook 信息
- **Hook 名称**: use{hook_name}
- **功能描述**: {description}
- **输入参数**: {input_params}
- **返回值**: {return_value}
- **使用场景**: {usage_example}

## 技术要求
1. 使用 TypeScript 严格模式
2. 遵循 React Hooks 规则
3. 正确处理依赖数组
4. 清理副作用（useEffect cleanup）
5. 类型安全，避免 any
6. 考虑边界情况

## 最佳实践
- 单一的抽象层次
- 清晰的职责边界
- 可组合性设计
- 完善的错误处理
- 性能优化（useMemo, useCallback）

## 输出格式
```typescript
import { useState, useEffect, useCallback, useMemo } from 'react';

interface UseHookNameParams {
  // 参数类型定义
}

interface UseHookNameReturn {
  // 返回值类型定义
}

export function useHookName(
  params: UseHookNameParams
): UseHookNameReturn {
  // Hook 实现
  
  return {
    // 返回值
  };
}
```
"#;

/// TypeScript 类型定义生成提示词模板
pub const TYPE_DEFINITION_PROMPT: &str = r#"你是一位 TypeScript 专家，擅长设计优雅、类型安全的类型系统。

## 任务
请为以下业务场景设计 TypeScript 类型定义。

## 业务场景
{business_context}

## 数据类型
{data_structure}

## 设计要求
1. 使用 interface 定义对象类型
2. 使用 type 定义联合类型和工具类型
3. 使用泛型提高复用性
4. 使用 readonly 标记不可变属性
5. 使用 ? 标记可选属性
6. 避免使用 any，使用 unknown 代替

## 类型安全
- 严格的 null/undefined 检查
- 字面量类型代替字符串常量
- 枚举类型代替魔法数字
- 判别联合（Discriminated Unions）

## 输出格式
```typescript
// 核心类型定义
interface TypeName {
  id: string;
  name: string;
  description?: string;
  createdAt: Date;
  status: 'active' | 'inactive' | 'pending';
}

// 工具类型
type PartialTypeName = Partial<TypeName>;
type ReadOnlyTypeName = Readonly<TypeName>;

// 泛型类型
interface Response<T> {
  success: boolean;
  data: T;
  error?: string;
}

// 联合类型
type ActionType = 'create' | 'update' | 'delete';
```
"#;

/// 单元测试生成提示词模板
pub const TEST_GENERATION_PROMPT: &str = r#"你是一位测试专家，擅长编写全面、高质量的单元测试。

## 任务
请为以下代码编写单元测试。

## 被测代码
```{language}
{code_to_test}
```

## 测试框架
{test_framework}

## 测试要求
1. 覆盖所有公共 API
2. 测试正常路径和异常路径
3. 测试边界条件
4. 使用有意义的测试用例名称
5. 遵循 AAA 模式（Arrange-Act-Assert）
6. 测试应该是独立的、可重复的

## 测试覆盖
- [ ] 正常情况测试
- [ ] 异常情况测试
- [ ] 边界条件测试
- [ ] 空值/undefined 测试
- [ ] 并发/异步测试

## 最佳实践
- 每个测试只测试一个行为
- 使用 describe 组织相关测试
- 使用 beforeEach/afterEach 设置环境
- Mock 外部依赖
- 测试应该有意义的名称

## 输出格式
```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { functionToTest } from './module';

describe('functionToTest', () => {
  describe('正常情况', () => {
    it('应该返回预期结果', () => {
      // Arrange
      const input = 'value';
      
      // Act
      const result = functionToTest(input);
      
      // Assert
      expect(result).toEqual(expected);
    });
  });

  describe('异常情况', () => {
    it('应该抛出错误', () => {
      // Arrange
      const invalidInput = null;
      
      // Act & Assert
      expect(() => functionToTest(invalidInput)).toThrow();
    });
  });
});
```
"#;

/// Tailwind CSS 样式生成提示词模板
pub const STYLE_GENERATION_PROMPT: &str = r#"你是一位 UI/UX 设计师，擅长使用 Tailwind CSS 创建美观、响应式的界面。

## 任务
请为以下组件生成 Tailwind CSS 样式。

## 组件信息
- **组件类型**: {component_type}
- **设计风格**: {design_style}
- **颜色主题**: {color_theme}
- **响应式要求**: {responsive_requirements}

## 设计要求
1. 遵循移动优先原则
2. 使用语义化的类名
3. 保持一致的间距系统
4. 考虑无障碍访问（a11y）
5. 支持暗色模式
6. 性能优化（避免不必要的类）

## 响应式断点
- sm: 640px+
- md: 768px+
- lg: 1024px+
- xl: 1280px+
- 2xl: 1536px+

## 输出格式
```jsx
<div className="
  /* 基础样式 */
  flex items-center justify-center
  p-4 m-2
  bg-white dark:bg-gray-800
  
  /* 响应式 */
  sm:flex-row
  md:p-6
  lg:w-full
  
  /* 交互状态 */
  hover:bg-gray-100 dark:hover:bg-gray-700
  focus:outline-none focus:ring-2 focus:ring-blue-500
  
  /* 无障碍 */
  cursor-pointer
  transition-colors duration-200
">
  {/* 内容 */}
</div>
```
"#;
