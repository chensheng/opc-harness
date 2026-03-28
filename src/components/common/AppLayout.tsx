import { Sidebar } from './Sidebar'
import { Header } from './Header'
import { LoadingOverlay } from './LoadingOverlay'

interface AppLayoutProps {
  children: React.ReactNode
}

export function AppLayout({ children }: AppLayoutProps) {
  return (
    <div className="flex h-screen bg-background">
      <Sidebar />
      <div className="flex flex-col flex-1 overflow-hidden">
        <Header />
        <main
          className="flex-1 overflow-auto"
          style={{ paddingBottom: 'calc(var(--safe-area-inset-bottom) + 2rem)' }}
        >
          <div className="p-6 min-h-full">{children}</div>
        </main>
      </div>
      <LoadingOverlay />
    </div>
  )
}
