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
          className="flex-1 overflow-auto p-6"
          style={{ paddingBottom: 'calc(0.5rem + var(--safe-area-inset-bottom))' }}
        >
          {children}
        </main>
      </div>
      <LoadingOverlay />
    </div>
  )
}
