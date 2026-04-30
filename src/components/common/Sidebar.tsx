import { NavLink } from 'react-router-dom'
import { Home, Lightbulb, Code, TrendingUp, Settings, Cpu, FolderOpen, Network } from 'lucide-react'
import { cn } from '@/lib/utils'
import { useAppStore } from '@/stores'

const navItems = [
  { path: '/', icon: Home, label: '首页' },
  { path: '/idea', icon: Lightbulb, label: 'Vibe Design' },
  { path: '/coding', icon: Code, label: 'Vibe Coding' },
  { path: '/marketing', icon: TrendingUp, label: 'Vibe Marketing' },
  { path: '/decentralized-test', icon: Network, label: '去中心化智能体' },
  { path: '/ai-config', icon: Cpu, label: 'AI配置' },
  { path: '/settings', icon: Settings, label: '设置' },
]

export function Sidebar() {
  const { isSidebarOpen } = useAppStore()

  return (
    <aside
      className={cn(
        'flex flex-col border-r bg-card transition-all duration-300',
        isSidebarOpen ? 'w-48' : 'w-16'
      )}
    >
      <div className="flex items-center h-16 px-4 border-b">
        <FolderOpen className="w-6 h-6 text-primary" />
        {isSidebarOpen && <span className="ml-3 font-semibold text-base">OPC-HARNESS</span>}
      </div>

      <nav className="flex-1 p-4 space-y-2">
        {navItems.map(item => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              cn(
                'flex items-center px-3 py-2 rounded-lg transition-colors text-sm',
                isActive
                  ? 'bg-primary text-primary-foreground'
                  : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground',
                !isSidebarOpen && 'justify-center'
              )
            }
          >
            <item.icon className={cn(!isSidebarOpen ? 'w-8 h-8' : 'w-5 h-5')} />
            {isSidebarOpen && <span className="ml-3">{item.label}</span>}
          </NavLink>
        ))}
      </nav>

      <div className="p-4 border-t">
        <div className={cn('text-xs text-muted-foreground', !isSidebarOpen && 'hidden')}>
          v0.1.0
        </div>
      </div>
    </aside>
  )
}
