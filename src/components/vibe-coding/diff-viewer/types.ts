// VC-036: Code Diff Visualizer Types

export interface DiffLine {
  line_number_old: number | null
  line_number_new: number | null
  content: string
  change_type: 'unchanged' | 'added' | 'removed'
}

export interface DiffHunk {
  header: string
  lines: DiffLine[]
  old_start: number
  old_count: number
  new_start: number
  new_count: number
}

export interface DiffStats {
  total_lines: number
  additions: number
  deletions: number
  unchanged: number
}

export interface FileDiff {
  file_path: string
  old_path: string | null
  new_path: string | null
  hunks: DiffHunk[]
  stats: DiffStats
}

export interface DiffSummary {
  file_path: string
  stats: DiffStats
  hunk_count: number
}

export interface DiffViewerProps {
  fileDiff: FileDiff | null
  viewMode?: 'side-by-side' | 'unified'
  onFileSelect?: (file: string) => void
}

export interface SideBySideLine {
  line_number: number | null
  content: string
  change_type: 'unchanged' | 'added' | 'removed'
}
