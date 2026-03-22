import { Loader2 } from 'lucide-react'
import { useAppStore } from '@/stores'

export function LoadingOverlay() {
  const { isLoading, loadingMessage } = useAppStore()

  if (!isLoading) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="flex flex-col items-center p-8 bg-card rounded-lg shadow-lg">
        <Loader2 className="w-10 h-10 animate-spin text-primary" />
        {loadingMessage && (
          <p className="mt-4 text-sm text-muted-foreground">{loadingMessage}</p>
        )}
      </div>
    </div>
  )
}
