/**
 * VD-015: AI 流式输出工具函数
 *
 * 提供与后端 ai_stream_chat 命令的交互接口
 */

import { invoke } from '@tauri-apps/api/tauri';
import type { ChatMessage, StreamChatRequest } from '@/types';

/**
 * 流式聊天 - 使用 SSE 接收实时响应
 *
 * @param request - 流式聊天请求参数
 * @returns Promise<void>
 *
 * @example
 * ```typescript
 * await streamChat({
 *   provider: 'openai',
 *   messages: [{ role: 'user', content: 'Hello' }],
 *   onChunk: (chunk) => console.log(chunk),
 *   onComplete: () => console.log('Complete!'),
 *   onError: (error) => console.error(error)
 * });
 * ```
 */
export async function streamChat(request: StreamChatRequest): Promise<void> {
  const { provider, messages, onChunk, onComplete, onError } = request;

  try {
    // 调用 Tauri 命令
    await invoke('ai_stream_chat', {
      provider,
      messages,
      callback: (chunk: string) => {
        // 每次接收到数据块时调用回调
        onChunk(chunk);
      },
    });

    // 完成后调用完成回调
    if (onComplete) {
      onComplete();
    }
  } catch (error) {
    // 错误处理
    const errorMessage = error instanceof Error ? error.message : String(error);
    if (onError) {
      onError(errorMessage);
    } else {
      console.error('Stream chat error:', errorMessage);
    }
    throw error;
  }
}

/**
 * 简化的流式聊天辅助函数
 * 直接返回一个异步生成器，方便使用 for-await-of 循环
 *
 * @param provider - AI 厂商标识
 * @param messages - 消息数组
 * @returns AsyncGenerator<string, void, unknown>
 *
 * @example
 * ```typescript
 * const stream = createStream('openai', [
 *   { role: 'user', content: 'Hello' }
 * ]);
 *
 * for await (const chunk of stream) {
 *   console.log(chunk);
 * }
 * ```
 */
export async function* createStream(
  provider: string,
  messages: ChatMessage[]
): AsyncGenerator<string, void, unknown> {
  let resolveCallback: ((value: string) => void) | null = null;
  let rejectCallback: ((reason?: unknown) => void) | null = null;

  // 创建一个 promise 用于等待下一个数据块
  const waitForChunk = (): Promise<string> => {
    return new Promise((resolve, reject) => {
      resolveCallback = resolve;
      rejectCallback = reject;
    });
  };

  // 调用 Tauri 命令
  invoke('ai_stream_chat', {
    provider,
    messages,
    callback: (chunk: string) => {
      if (resolveCallback) {
        resolveCallback(chunk);
        resolveCallback = null;
        rejectCallback = null;
      }
    },
  }).catch(() => {
    if (rejectCallback) {
      rejectCallback(new Error('Stream chat failed'));
      rejectCallback = null;
      resolveCallback = null;
    }
  });

  // 使用生成器持续产生数据块
  while (true) {
    try {
      const chunk = await waitForChunk();
      yield chunk;
    } catch {
      // 流结束或出错
      break;
    }
  }
}
