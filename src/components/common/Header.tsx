import { Menu, User } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { useAppStore } from '@/stores'

export function Header() {
  const { toggleSidebar } = useAppStore()

  return (
    <header className="flex items-center justify-between h-16 px-6 border-b bg-card">
      <div className="flex items-center">
        <Button
          variant="ghost"
          size="icon"
          onClick={toggleSidebar}
          className="mr-4"
        >
          <Menu className="w-5 h-5" />
        </Button>
        <h1 className="text-lg font-medium">AI驱动的一人公司操作系统</h1>
      </div>

      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon">
          <User className="w-5 h-5" />
        </Button>
      </div>
    </header>
  )
}
