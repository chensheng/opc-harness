/**
 * VD-015: AI 流式聊天示例组件
 *
 * 演示如何使用 useAIStream Hook 实现流式对话功能
 */

import React, { useState } from 'react';
import { useAIStream } from '@/hooks/useAIStream';
import type { ChatMessage, AIProvider } from '@/types';

interface AIStreamChatProps {
  /** AI 厂商标识 */
  provider?: AIProvider;

  /** 初始提示词 */
  initialPrompt?: string;

  /** 自定义样式类名 */
  className?: string;
}

/**
 * AI 流式聊天组件
 */
export const AIStreamChat: React.FC<AIStreamChatProps> = ({
  provider = 'openai',
  initialPrompt = '你好，请介绍一下自己',
  className = '',
}) => {
  const [inputMessage, setInputMessage] = useState(initialPrompt);
  const [conversation, setConversation] = useState<ChatMessage[]>([]);

  const { isLoading, response, error, sendMessage, reset } = useAIStream(provider, {
    onChunk: (_chunk, _fullResponse) => {
      // 可以在这里添加自定义逻辑，比如打字机效果
      console.log('Received chunk');
    },
    onComplete: finalResponse => {
      console.log('Stream complete:', finalResponse);

      // 将完整的对话添加到历史记录
      setConversation(prev => [...prev, { role: 'assistant', content: finalResponse }]);
    },
    onError: error => {
      console.error('Stream error:', error);
    },
  });

  /**
   * 处理发送消息
   */
  const handleSend = async () => {
    if (!inputMessage.trim() || isLoading) return;

    const userMessage: ChatMessage = {
      role: 'user',
      content: inputMessage,
    };

    // 添加用户消息到历史记录
    setConversation(prev => [...prev, userMessage]);

    try {
      await sendMessage([userMessage]);
      setInputMessage(''); // 清空输入框
    } catch (error) {
      console.error('Failed to send message:', error);
    }
  };

  /**
   * 处理重置
   */
  const handleReset = () => {
    setConversation([]);
    reset();
    setInputMessage(initialPrompt);
  };

  return (
    <div className={`ai-stream-chat ${className}`}>
      {/* 对话历史 */}
      <div className="conversation-history">
        {conversation.map((msg, index) => (
          <div key={index} className={`message ${msg.role}`}>
            <strong>{msg.role === 'user' ? '👤 你' : '🤖 AI'}:</strong>
            <p>{msg.content}</p>
          </div>
        ))}

        {/* 正在接收的响应 */}
        {isLoading && (
          <div className="message assistant streaming">
            <strong>🤖 AI:</strong>
            <p>{response || '思考中...'}</p>
            <span className="streaming-indicator">⚡ 流式中...</span>
          </div>
        )}
      </div>

      {/* 错误信息 */}
      {error && (
        <div className="error-message">
          <strong>❌ 错误:</strong>
          <p>{error}</p>
        </div>
      )}

      {/* 输入区域 */}
      <div className="input-area">
        <textarea
          value={inputMessage}
          onChange={e => setInputMessage(e.target.value)}
          placeholder="输入你的问题..."
          disabled={isLoading}
          rows={3}
          className="message-input"
        />

        <div className="button-group">
          <button
            onClick={handleSend}
            disabled={isLoading || !inputMessage.trim()}
            className="send-button"
          >
            {isLoading ? '发送中...' : '发送'}
          </button>

          <button onClick={handleReset} disabled={isLoading} className="reset-button">
            重置
          </button>
        </div>
      </div>

      {/* 样式 */}
      <style>{`
        .ai-stream-chat {
          max-width: 800px;
          margin: 0 auto;
          padding: 20px;
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        .conversation-history {
          background: #f5f5f5;
          border-radius: 8px;
          padding: 16px;
          margin-bottom: 16px;
          min-height: 200px;
          max-height: 500px;
          overflow-y: auto;
        }

        .message {
          margin-bottom: 16px;
          padding: 12px;
          border-radius: 6px;
          background: white;
        }

        .message.user {
          background: #e3f2fd;
          border-left: 4px solid #2196f3;
        }

        .message.assistant {
          background: #fff3e0;
          border-left: 4px solid #ff9800;
        }

        .message.streaming {
          animation: pulse 1.5s ease-in-out infinite;
        }

        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.7; }
        }

        .streaming-indicator {
          display: inline-block;
          margin-left: 8px;
          padding: 2px 8px;
          background: #ff9800;
          color: white;
          border-radius: 4px;
          font-size: 12px;
          animation: blink 1s ease-in-out infinite;
        }

        @keyframes blink {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.5; }
        }

        .error-message {
          background: #ffebee;
          border: 1px solid #f44336;
          border-radius: 6px;
          padding: 12px;
          margin-bottom: 16px;
          color: #c62828;
        }

        .input-area {
          display: flex;
          flex-direction: column;
          gap: 12px;
        }

        .message-input {
          width: 100%;
          padding: 12px;
          border: 2px solid #ddd;
          border-radius: 6px;
          font-size: 14px;
          font-family: inherit;
          resize: vertical;
          transition: border-color 0.2s;
        }

        .message-input:focus {
          outline: none;
          border-color: #2196f3;
        }

        .message-input:disabled {
          background: #f5f5f5;
          cursor: not-allowed;
        }

        .button-group {
          display: flex;
          gap: 8px;
        }

        .send-button,
        .reset-button {
          padding: 10px 20px;
          border: none;
          border-radius: 6px;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
          transition: all 0.2s;
        }

        .send-button {
          background: #2196f3;
          color: white;
          flex: 1;
        }

        .send-button:hover:not(:disabled) {
          background: #1976d2;
        }

        .send-button:disabled {
          background: #bdbdbd;
          cursor: not-allowed;
        }

        .reset-button {
          background: #f5f5f5;
          color: #616161;
        }

        .reset-button:hover:not(:disabled) {
          background: #e0e0e0;
        }

        .reset-button:disabled {
          background: #bdbdbd;
          cursor: not-allowed;
        }
      `}</style>
    </div>
  );
};
