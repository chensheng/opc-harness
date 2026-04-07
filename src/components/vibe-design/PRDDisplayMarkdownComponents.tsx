import React from 'react'
import type { Components } from 'react-markdown'

// Markdown 表自定义组件，确保边框显示并增加上下间距
export const TableComponent = ({ ...props }: React.HTMLAttributes<HTMLTableElement>) => (
  <div className="overflow-x-auto my-6 first:mt-4 last:mb-4">
    <table className="w-full border-collapse border border-border" {...props} />
  </div>
)

export const ThComponent = ({ ...props }: React.HTMLAttributes<HTMLTableCellElement>) => (
  <th
    className="border border-border px-4 py-3 bg-muted/80 text-left font-semibold text-sm"
    {...props}
  />
)

export const TdComponent = ({ ...props }: React.HTMLAttributes<HTMLTableCellElement>) => (
  <td className="border border-border px-4 py-3 text-left text-sm" {...props} />
)

export const TrComponent = ({ ...props }: React.HTMLAttributes<HTMLTableRowElement>) => (
  <tr className="even:bg-muted/30 hover:bg-muted/50 transition-colors" {...props} />
)

// 段落组件，确保与表格有适当间距
export const ParagraphComponent = ({ ...props }: React.HTMLAttributes<HTMLParagraphElement>) => (
  <p className="text-base leading-relaxed mb-4 last:mb-0 text-foreground/90" {...props} />
)

// 完整文档视图的自定义组件 - 更美观的排版
export const FullDocComponents: Partial<Components> = {
  // 标题层级
  h1: ({ ...props }) => (
    <h1
      className="text-3xl font-bold mb-6 mt-8 pb-2 border-b border-border text-primary"
      {...props}
    />
  ),
  h2: ({ ...props }) => (
    <h2
      className="text-2xl font-semibold mb-4 mt-7 pb-1.5 border-b border-border/50 text-foreground"
      {...props}
    />
  ),
  h3: ({ ...props }) => (
    <h3 className="text-xl font-medium mb-3 mt-5 text-foreground/90" {...props} />
  ),
  h4: ({ ...props }) => (
    <h4 className="text-lg font-medium mb-2 mt-4 text-foreground/80" {...props} />
  ),

  // 段落和文本
  p: ParagraphComponent,

  // 列表
  ul: ({ ...props }) => <ul className="list-disc list-outside pl-6 mb-4 space-y-2" {...props} />,
  ol: ({ ...props }) => <ol className="list-decimal list-outside pl-6 mb-4 space-y-2" {...props} />,
  li: ({ ...props }) => <li className="text-base leading-relaxed text-foreground/90" {...props} />,

  // 强调
  strong: ({ ...props }) => <strong className="font-bold text-foreground" {...props} />,
  em: ({ ...props }) => <em className="italic text-foreground/80" {...props} />,

  // 代码
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  code: (props: any) => {
    const { inline, children } = props
    return inline ? (
      <code
        className="bg-muted/80 px-2 py-0.5 rounded-md text-sm font-mono text-primary border border-border/30"
        {...props}
      >
        {children}
      </code>
    ) : (
      <code
        className="block bg-muted p-3 rounded-lg my-3 overflow-x-auto border border-border/50"
        {...props}
      >
        {children}
      </code>
    )
  },
  pre: ({ ...props }) => (
    <pre
      className="bg-gradient-to-br from-muted to-muted/80 p-0 rounded-lg my-4 overflow-hidden border border-border/50 shadow-sm"
      {...props}
    />
  ),

  // 引用块
  blockquote: ({ ...props }) => (
    <blockquote
      className="border-l-4 border-primary pl-4 py-2 my-4 bg-muted/20 rounded-r-lg italic text-foreground/80"
      {...props}
    />
  ),

  // 链接
  a: ({ ...props }) => (
    <a
      className="text-primary hover:text-primary/80 underline decoration-primary/50 hover:decoration-primary transition-all font-medium"
      {...props}
    />
  ),

  // 分隔线
  hr: ({ ...props }) => <hr className="border-border my-8" {...props} />,

  // 表格
  table: TableComponent,
  th: ThComponent,
  td: TdComponent,
  tr: TrComponent,
}

// 流式生成时的简化组件
export const StreamingComponents: Partial<Components> = {
  table: TableComponent,
  th: ThComponent,
  td: TdComponent,
  tr: TrComponent,
  h1: ({ ...props }) => (
    <h1
      className="text-2xl font-bold mb-4 mt-6 pb-2 border-b border-border text-primary"
      {...props}
    />
  ),
  h2: ({ ...props }) => (
    <h2
      className="text-xl font-semibold mb-3 mt-5 pb-1.5 border-b border-border/50 text-foreground"
      {...props}
    />
  ),
  h3: ({ ...props }) => (
    <h3 className="text-lg font-medium mb-2 mt-4 text-foreground/90" {...props} />
  ),
  p: ({ ...props }) => (
    <p className="text-base leading-relaxed mb-3 last:mb-0 text-foreground/90" {...props} />
  ),
  ul: ({ ...props }) => <ul className="list-disc list-outside pl-6 mb-3 space-y-1.5" {...props} />,
  ol: ({ ...props }) => (
    <ol className="list-decimal list-outside pl-6 mb-3 space-y-1.5" {...props} />
  ),
  li: ({ ...props }) => <li className="text-sm leading-relaxed text-foreground/90" {...props} />,
  strong: ({ ...props }) => <strong className="font-bold text-foreground" {...props} />,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  code: (props: any) => {
    const { inline, children } = props
    return inline ? (
      <code
        className="bg-muted/80 px-2 py-0.5 rounded-md text-sm font-mono text-primary border border-border/30"
        {...props}
      >
        {children}
      </code>
    ) : (
      <code
        className="block bg-muted p-3 rounded-lg my-3 overflow-x-auto border border-border/50"
        {...props}
      >
        {children}
      </code>
    )
  },
  pre: ({ ...props }) => (
    <pre
      className="bg-gradient-to-br from-muted to-muted/80 p-0 rounded-lg my-3 overflow-hidden border border-border/50 shadow-sm"
      {...props}
    />
  ),
  blockquote: ({ ...props }) => (
    <blockquote
      className="border-l-4 border-primary pl-4 py-2 my-3 bg-muted/20 rounded-r-lg italic text-foreground/80"
      {...props}
    />
  ),
  a: ({ ...props }) => (
    <a
      className="text-primary hover:text-primary/80 underline decoration-primary/50 hover:decoration-primary transition-all font-medium"
      {...props}
    />
  ),
}
