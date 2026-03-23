import { render, RenderOptions } from '@testing-library/react'
import { ReactElement, ReactNode } from 'react'
import { BrowserRouter } from 'react-router-dom'

// 创建 Providers 组件包装器
function AllProviders({ children }: { children: ReactNode }) {
  return (
    <BrowserRouter>
      {/* 未来可以添加其他 providers，如 ThemeProvider、QueryClientProvider 等 */}
      {children}
    </BrowserRouter>
  )
}

// 自定义 render 函数
const customRender = (ui: ReactElement, options?: Omit<RenderOptions, 'wrapper'>) =>
  render(ui, { wrapper: AllProviders, ...options })

// 重新导出 testing-library 的所有内容
export * from '@testing-library/react'
export { customRender as render }
