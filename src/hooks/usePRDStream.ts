/**
 * VD-020: PRD 流式生成 React Hook
 *
 * 提供打字机效果来实时显示 PRD 生成过程
 */

import { useState, useCallback, useRef } from 'react';

/**
 * VD-020: PRD 流式生成 React Hook
 *
 * 提供打字机效果来实时显示 PRD 生成过程
 */

/**
 * PRD 流式输出状态
 */
interface UsePRDStreamState {
  /** 是否正在生成 */
  isGenerating: boolean;

  /** 当前已生成的 PRD 内容 */
  prd: string;

  /** 错误信息 */
  error: string | null;

  /** 是否已完成生成 */
  isComplete: boolean;

  /** 生成进度 (0-100) */
  progress: number;
}

/**
 * PRD 流式输出操作接口
 */
interface UsePRDStreamActions {
  /** 开始生成 PRD */
  generatePRD: (idea: string, provider?: string) => Promise<void>;

  /** 停止生成 */
  stop: () => void;

  /** 重置状态 */
  reset: () => void;
}

/**
 * PRD 流式生成 Hook
 *
 * @returns 状态和操作函数
 *
 * @example
 * ```typescript
 * function PRDGenerator() {
 *   const {
 *     isGenerating,
 *     prd,
 *     error,
 *     isComplete,
 *     progress,
 *     generatePRD,
 *     stop,
 *     reset
 *   } = usePRDStream();
 *
 *   return (
 *     <div>
 *       {isGenerating && <p>生成中... {progress}%</p>}
 *       <Markdown>{prd}</Markdown>
 *     </div>
 *   );
 * }
 * ```
 */
export function usePRDStream(): UsePRDStreamState & UsePRDStreamActions {
  // 状态管理
  const [isGenerating, setIsGenerating] = useState(false);
  const [prd, setPrd] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isComplete, setIsComplete] = useState(false);
  const [progress, setProgress] = useState(0);

  // Refs
  const abortControllerRef = useRef<AbortController | null>(null);
  const streamBufferRef = useRef('');
  const displayIndexRef = useRef(0);

  /**
   * 打字机效果 - 逐字显示内容
   */
  const typewriterEffect = useCallback((fullText: string) => {
    const chars = fullText.split('');
    let currentIndex = 0;

    const typeChar = () => {
      if (currentIndex < chars.length) {
        setPrd(prev => prev + chars[currentIndex]);
        currentIndex++;

        // 根据字符类型调整速度
        const char = chars[currentIndex - 1] || '';
        const delay = char === '\n' ? 50 : /[,.!?]/.test(char) ? 80 : 30;

        setTimeout(typeChar, delay);
      }
    };

    typeChar();
  }, []);

  /**
   * 开始生成 PRD
   */
  const generatePRD = async (idea: string) => {
    if (!idea.trim()) {
      setError('请输入产品想法');
      return;
    }

    // 重置状态
    reset();
    setIsGenerating(true);
    setError(null);
    setProgress(0);

    try {
      // 使用 Tauri command 调用后端 API
      // TODO: 实现真实的流式调用
      // const result = await invoke('generate_prd_stream', { idea });

      // 模拟流式生成（用于演示）
      const sections = [
        '## 1. 产品概述\n这是一个基于你的想法的产品，旨在解决用户的核心痛点。\n\n',
        '## 2. 目标用户\n- 主要用户群体：需要此解决方案的用户\n- 次要用户群体：相关领域的从业者\n\n',
        '## 3. 市场分析\n市场规模庞大且持续增长，存在明显的市场机会。\n\n',
        '## 4. 核心功能\n### 3.1 功能一\n描述第一个核心功能\n\n### 3.2 功能二\n描述第二个核心功能\n\n',
        '## 5. 技术架构\n- 前端：React + TypeScript\n- 后端：Tauri + Rust\n- 数据库：SQLite\n\n',
        '## 6. 商业模式\n多元化的收入模式，包括订阅制和增值服务。\n\n',
        '## 7. 开发计划\n- Phase 1: MVP (2-3 周)\n- Phase 2: 功能完善 (4-6 周)\n- Phase 3: 商业化 (8-10 周)\n\n',
        '## 8. 风险评估\n- 技术风险：中等\n- 市场风险：低\n- 竞争风险：中等\n\n',
      ];

      // 模拟流式输出
      for (let i = 0; i < sections.length; i++) {
        if (!isGenerating) break;

        // 添加新段落
        const newContent = sections[i];
        streamBufferRef.current += newContent;

        // 使用打字机效果显示
        typewriterEffect(newContent || '');

        // 更新进度
        const currentProgress = Math.round(((i + 1) / sections.length) * 100);
        setProgress(currentProgress);

        // 模拟网络延迟
        await new Promise(resolve => setTimeout(resolve, 800 + Math.random() * 400));
      }

      setIsComplete(true);
      setIsGenerating(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '生成 PRD 失败';
      setError(errorMessage);
      setIsGenerating(false);
      setIsComplete(false);
    }
  };

  /**
   * 停止生成
   */
  const stop = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }
    setIsGenerating(false);
  }, []);

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    setIsGenerating(false);
    setPrd('');
    setError(null);
    setIsComplete(false);
    setProgress(0);
    streamBufferRef.current = '';
    displayIndexRef.current = 0;
  }, []);

  return {
    isGenerating,
    prd,
    error,
    isComplete,
    progress,
    generatePRD,
    stop,
    reset,
  };
}
