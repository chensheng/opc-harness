/**
 * VD-015: AI 流式输出 React Hook
 *
 * 提供简单易用的流式输出接口，支持实时响应显示
 */

import { useState, useCallback, useRef, useEffect } from 'react';
import { streamChat } from '@/utils/aiStream';
import type { ChatMessage, AIProvider } from '@/types';

/**
 * 流式输出状态
 */
interface UseAIStreamState {
  /** 是否正在流式输出 */
  isLoading: boolean;

  /** 完整的响应文本 */
  response: string;

  /** 错误信息 */
  error: string | null;

  /** 是否已完成 */
  isComplete: boolean;
}

/**
 * 流式输出操作接口
 */
interface UseAIStreamActions {
  /** 开始流式对话 */
  sendMessage: (messages: ChatMessage[]) => Promise<void>;

  /** 停止流式输出（暂未实现） */
  stop: () => void;

  /** 重置状态 */
  reset: () => void;
}

/**
 * AI 流式输出 Hook
 *
 * @returns 状态和操作函数
 *
 * @example
 * ```typescript
 * function ChatComponent() {
 *   const {
 *     isLoading,
 *     response,
 *     error,
 *     sendMessage,
 *     reset
 *   } = useAIStream('openai');
 *
 *   const handleSend = async () => {
 *     await sendMessage([{ role: 'user', content: 'Hello' }]);
 *   };
 *
 *   return (
 *     <div>
 *       <button onClick={handleSend}>Send</button>
 *       <div>{response}</div>
 *       {error && <div>Error: {error}</div>}
 *     </div>
 *   );
 * }
 * ```
 */
export function useAIStream(
  provider: AIProvider,
  options?: {
    /** 初始响应文本 */
    initialResponse?: string;

    /** 数据块回调 */
    onChunk?: (chunk: string, fullResponse: string) => void;

    /** 完成回调 */
    onComplete?: (finalResponse: string) => void;

    /** 错误回调 */
    onError?: (error: string) => void;
  }
): UseAIStreamState & UseAIStreamActions {
  const [state, setState] = useState<UseAIStreamState>({
    isLoading: false,
    response: options?.initialResponse ?? '',
    error: null,
    isComplete: false,
  });

  // 使用 ref 保存最新的回调函数，避免闭包问题
  const callbacks = useRef({
    onChunk: options?.onChunk,
    onComplete: options?.onComplete,
    onError: options?.onError,
  });

  // 在 useEffect 中更新 refs，避免在 render 期间修改 ref
  useEffect(() => {
    callbacks.current.onChunk = options?.onChunk;
    callbacks.current.onComplete = options?.onComplete;
    callbacks.current.onError = options?.onError;
  }, [options?.onChunk, options?.onComplete, options?.onError]);

  /**
   * 发送消息并开始流式接收
   */
  const sendMessage = useCallback(
    async (messages: ChatMessage[]) => {
      setState(prev => ({
        ...prev,
        isLoading: true,
        response: options?.initialResponse ?? '',
        error: null,
        isComplete: false,
      }));

      try {
        await streamChat({
          provider,
          messages,
          onChunk: (chunk: string) => {
            setState(prev => {
              const newResponse = prev.response + chunk;

              // 调用数据块回调
              if (callbacks.current.onChunk) {
                callbacks.current.onChunk(chunk, newResponse);
              }

              return {
                ...prev,
                response: newResponse,
              };
            });
          },
          onComplete: () => {
            setState(prev => {
              // 调用完成回调
              if (callbacks.current.onComplete) {
                callbacks.current.onComplete(prev.response);
              }

              return {
                ...prev,
                isLoading: false,
                isComplete: true,
              };
            });
          },
          onError: (_error: string) => {
            setState(prev => {
              // 调用错误回调
              if (callbacks.current.onError) {
                callbacks.current.onError(_error);
              }

              return {
                ...prev,
                isLoading: false,
                error: _error,
                isComplete: false,
              };
            });
          },
        });
      } catch (_error) {
        const errorMessage = _error instanceof Error ? _error.message : String(_error);

        setState(prev => ({
          ...prev,
          isLoading: false,
          error: errorMessage,
          isComplete: false,
        }));

        // 抛出错误让调用方也能处理
        throw _error;
      }
    },
    [provider, options?.initialResponse]
  );

  /**
   * 停止流式输出（预留功能）
   */
  const stop = useCallback(() => {
    // TODO: 实现停止逻辑（需要后端支持）
    console.warn('Stop functionality not yet implemented');
  }, []);

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    setState({
      isLoading: false,
      response: options?.initialResponse ?? '',
      error: null,
      isComplete: false,
    });
  }, [options?.initialResponse]);

  return {
    ...state,
    sendMessage,
    stop,
    reset,
  };
}
