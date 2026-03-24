# 前端开发规范 (React + TypeScript)

> **适用范围**: `src/` 目录下所有前端代码  
> **最后更新**: 2026-03-22

## 🎯 模块职责

### 分层架构
```
src/
├── components/     # UI 组件层 - 仅负责渲染和用户交互
├── hooks/         # 自定义 Hooks - 封装可复用的 React 逻辑
├── stores/        # 状态管理层 - 全局状态管理 (Zustand)
├── types/         # 类型定义层 - TypeScript 接口和类型
└── lib/           # 工具函数层 - 纯函数，无业务依赖
```

### 数据流规则
```typescript
// ✅ 推荐：单向数据流
Component → Store → Tauri Commands → Rust Backend
     ↑                                       |
     └─────────── State Update ←─────────────┘

// ❌ 禁止：
- 组件直接调用 invoke() - 必须通过 hooks 或 stores 封装
- Store 直接操作 DOM - 必须通过组件
- 循环依赖：components → stores → components
```

## 🛠️ 开发规范

### 组件开发
```typescript
// ✅ 推荐：使用函数组件 + TypeScript
interface Props {
  title: string;
  on_submit?: (data: FormData) => void;
}

export function MyComponent({ title, on_submit }: Props) {
  return <div>{title}</div>;
}

// ❌ 避免：使用 class 组件
// ❌ 避免：使用 PropTypes 代替 TypeScript
```

### 状态管理
```typescript
// ✅ 推荐：使用 Zustand store
import { useAppStore } from '@/stores/appStore';

function Component() {
  const { projects, addProject } = useAppStore();
  return <div>{projects.map(p => <Project key={p.id} {...p} />)}</div>;
}

// ❌ 避免：使用全局变量
// ❌ 避免：在组件内直接调用 invoke()
```

### 类型定义
```typescript
// ✅ 推荐：集中管理类型
// src/types/index.ts
export interface Project {
  id: string;
  name: string;
  status: 'idea' | 'design' | 'coding' | 'completed';
}

// ❌ 避免：使用 any
// ❌ 避免：在多个文件重复定义相同类型
```

## 📁 文件组织

### 命名规范
- **组件文件**: PascalCase (e.g., `MyComponent.tsx`)
- **Hooks 文件**: camelCase + hook 后缀 (e.g., `useAIStream.ts`)
- **工具文件**: camelCase (e.g., `utils.ts`)
- **类型文件**: 集中在 `types/index.ts`

### 目录结构
```
components/
├── ui/              # shadcn/ui 基础组件 (不可修改)
├── common/          # 通用组件 (Layout, Header, Sidebar)
├── vibe-design/     # Vibe Design 业务组件
├── vibe-coding/     # Vibe Coding 业务组件
└── vibe-marketing/  # Vibe Marketing 业务组件
```

## 🔒 架构约束

### 依赖方向
```typescript
// ✅ 允许：
components → hooks → stores → types
components → lib/utils

// ❌ 禁止：
stores → components  // 状态层不可依赖 UI 层
hooks → components   // Hooks 不可依赖具体组件
lib → stores         // 工具函数不可依赖状态
```

### 导入规范
```typescript
// ✅ 推荐：使用路径别名
import { Button } from '@/components/ui/button';
import { useAppStore } from '@/stores/appStore';

// ❌ 避免：相对路径过深
import { Button } from '../../../components/ui/button';

// ⚠️ 强制：ESLint 检查 import 路径深度 <= 3
```

## 🚨 常见陷阱

### 陷阱 1: 在组件中直接调用 Tauri API
``typescript
// ❌ 错误
function MyComponent() {
  const handleSave = async () => {
    await invoke('save_project', { data });
  };
}

// ✅ 正确
// stores/projectStore.ts
export const useProjectStore = create((set) => ({
  saveProject: async (data) => {
    await invoke('save_project', { data });
  }
}));

// components/MyComponent.tsx
function MyComponent() {
  const { saveProject } = useProjectStore();
  const handleSave = () => saveProject(data);
}
```

### 陷阱 2: 过度使用 useEffect
``typescript
// ❌ 错误：不必要的 useEffect
const [count, setCount] = useState(0);
useEffect(() => {
  document.title = `Count: ${count}`;
}, [count]);

// ✅ 正确：直接在事件处理中更新
const handleClick = () => {
  setCount(c => c + 1);
  document.title = `Count: ${count + 1}`;
};
```

### 陷阱 3: 类型滥用 any
```typescript
// ❌ 错误
function processData(data: any) {
  return data.value;
}

// ✅ 正确
interface Data {
  value: string;
}
function processData(data: Data) {
  return data.value;
}
```

## 📖 参考资源

- [React 官方文档](https://react.dev/)
- [TypeScript 手册](https://www.typescriptlang.org/docs/)
- [Zustand 文档](https://zustand-demo.pmnd.rs/)
- [shadcn/ui 文档](https://ui.shadcn.com/)

---

**需要帮助？** 
查看根目录 [`AGENTS.md`](../AGENTS.md) 获取更多导航信息。
