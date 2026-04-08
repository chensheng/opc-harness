import { useEffect, useState, useRef } from 'react'
import { ChevronUp } from 'lucide-react'
import { Button } from '@/components/ui/button'

interface ScrollToTopProps {
  className?: string
  threshold?: number
  scrollContainerRef?: React.RefObject<HTMLElement>
}

export function ScrollToTop({ 
  className = '', 
  threshold = 300,
  scrollContainerRef 
}: ScrollToTopProps) {
  const [isVisible, setIsVisible] = useState(false)
  const containerRef = useRef<HTMLElement | null>(null)

  useEffect(() => {
    // 如果提供了外部 ref，使用它；否则使用 window
    if (scrollContainerRef?.current) {
      containerRef.current = scrollContainerRef.current
    } else {
      containerRef.current = document.querySelector('main') as HTMLElement
    }

    const target = containerRef.current

    const toggleVisibility = () => {
      if (!target) return
      
      const scrollTop = target.scrollTop
      if (scrollTop > threshold) {
        setIsVisible(true)
      } else {
        setIsVisible(false)
      }
    }

    // 初始检查
    toggleVisibility()

    if (target) {
      target.addEventListener('scroll', toggleVisibility)
      return () => target.removeEventListener('scroll', toggleVisibility)
    }
  }, [threshold, scrollContainerRef])

  const scrollToTop = () => {
    const target = containerRef.current
    
    if (target) {
      target.scrollTo({
        top: 0,
        behavior: 'smooth',
      })
    }
  }

  if (!isVisible) return null

  return (
    <Button
      size="icon"
      onClick={scrollToTop}
      className={`fixed bottom-8 right-8 z-50 rounded-full shadow-lg transition-all duration-300 hover:scale-110 ${className}`}
      style={{
        bottom: 'calc(4rem + env(safe-area-inset-bottom, 0px))',
        right: 'calc(2rem + env(safe-area-inset-right, 0px))',
      }}
      aria-label="回到顶部"
    >
      <ChevronUp className="h-5 w-5" />
    </Button>
  )
}
