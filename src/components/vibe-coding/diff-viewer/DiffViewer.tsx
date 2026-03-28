import React, { useState } from 'react'
import { FileDiff, DiffViewerProps, SideBySideLine } from './types'
import './styles.css'

const DiffViewer: React.FC<DiffViewerProps> = ({
  fileDiff,
  viewMode = 'side-by-side',
  onFileSelect,
}) => {
  const [expandedHunks, setExpandedHunks] = useState<number[]>([])

  if (!fileDiff) {
    return <div className="diff-viewer-empty">No diff data available</div>
  }

  const toggleHunk = (index: number) => {
    setExpandedHunks(prev =>
      prev.includes(index) ? prev.filter(i => i !== index) : [...prev, index]
    )
  }

  const renderLineNumber = (line: any, side: 'left' | 'right') => {
    const lineNum = side === 'left' ? line.line_number_old : line.line_number_new
    if (lineNum === null) {
      return <span className="diff-line-number-empty"></span>
    }
    return <span className="diff-line-number">{lineNum}</span>
  }

  const renderLine = (line: any, index: number, side: 'left' | 'right') => {
    const className = `diff-line ${
      line.change_type === 'added'
        ? 'diff-line-added'
        : line.change_type === 'removed'
          ? 'diff-line-removed'
          : 'diff-line-unchanged'
    }`

    return (
      <div key={`${side}-${index}`} className={className}>
        {renderLineNumber(line, side)}
        <span className="diff-line-content">{line.content || '\u00A0'}</span>
      </div>
    )
  }

  const renderSideBySide = () => {
    return fileDiff.hunks.map((hunk, hunkIndex) => (
      <div key={hunkIndex} className="diff-hunk">
        <div className="diff-hunk-header" onClick={() => toggleHunk(hunkIndex)}>
          <span className="diff-hunk-toggle">{expandedHunks.includes(hunkIndex) ? '▼' : '▶'}</span>
          <code>{hunk.header}</code>
        </div>

        {(expandedHunks.includes(hunkIndex) || expandedHunks.length === 0) && (
          <div className="diff-hunk-content">
            <div className="diff-lines-container">
              <div className="diff-lines-side">
                {hunk.lines.map((line, lineIndex) => renderLine(line, lineIndex, 'left'))}
              </div>
              <div className="diff-lines-side">
                {hunk.lines.map((line, lineIndex) => renderLine(line, lineIndex, 'right'))}
              </div>
            </div>
          </div>
        )}
      </div>
    ))
  }

  const renderUnified = () => {
    return fileDiff.hunks.map((hunk, hunkIndex) => (
      <div key={hunkIndex} className="diff-hunk">
        <div className="diff-hunk-header" onClick={() => toggleHunk(hunkIndex)}>
          <span className="diff-hunk-toggle">{expandedHunks.includes(hunkIndex) ? '▼' : '▶'}</span>
          <code>{hunk.header}</code>
        </div>

        {(expandedHunks.includes(hunkIndex) || expandedHunks.length === 0) && (
          <div className="diff-hunk-content">
            {hunk.lines.map((line, lineIndex) => {
              const className = `diff-line ${
                line.change_type === 'added'
                  ? 'diff-line-added'
                  : line.change_type === 'removed'
                    ? 'diff-line-removed'
                    : 'diff-line-unchanged'
              }`

              return (
                <div key={lineIndex} className={className}>
                  <span className="diff-line-prefix">
                    {line.change_type === 'added'
                      ? '+'
                      : line.change_type === 'removed'
                        ? '-'
                        : ' '}
                  </span>
                  <span className="diff-line-number">
                    {line.line_number_new || line.line_number_old}
                  </span>
                  <span className="diff-line-content">{line.content || '\u00A0'}</span>
                </div>
              )
            })}
          </div>
        )}
      </div>
    ))
  }

  return (
    <div className="diff-viewer">
      <div className="diff-viewer-header">
        <h3 className="diff-viewer-title">{fileDiff.file_path}</h3>
        <div className="diff-viewer-stats">
          <span className="stat-additions">+{fileDiff.stats.additions}</span>
          <span className="stat-deletions">-{fileDiff.stats.deletions}</span>
          <span className="stat-total">{fileDiff.stats.total_lines} lines</span>
        </div>
        <div className="diff-viewer-modes">
          <button
            className={`mode-btn ${viewMode === 'side-by-side' ? 'active' : ''}`}
            onClick={() => {}}
          >
            Side by Side
          </button>
          <button
            className={`mode-btn ${viewMode === 'unified' ? 'active' : ''}`}
            onClick={() => {}}
          >
            Unified
          </button>
        </div>
      </div>

      <div className="diff-viewer-content">
        {viewMode === 'side-by-side' ? renderSideBySide() : renderUnified()}
      </div>
    </div>
  )
}

export default DiffViewer
