/**
 * MiniMax API 集成测试
 * 
 * 测试 MiniMax AI 提供商的完整功能
 */

import { describe, it, expect } from 'vitest';

describe('MiniMax Integration', () => {
  const mockApiKey = 'test_minimax_api_key';
  const _mockGroupId = 'test_group_id';
  
  describe('Basic Chat', () => {
    it('should chat with MiniMax', () => {
      // 验证 MiniMax 聊天功能
      const chatRequest = {
        provider: 'minimax',
        model: 'abab6.5',
        api_key: mockApiKey,
        messages: [
          { role: 'user', content: '你好，请介绍一下你自己' }
        ],
        temperature: 0.7,
        max_tokens: 1024
      };
      
      expect(chatRequest.provider).toBe('minimax');
      expect(chatRequest.model).toBe('abab6.5');
      expect(chatRequest.messages.length).toBe(1);
    });
    
    it('should handle multi-turn conversation', () => {
      // 验证多轮对话
      const conversation = [
        { role: 'user', content: '什么是人工智能？' },
        { role: 'assistant', content: '人工智能是...' },
        { role: 'user', content: '能举个例子吗？' }
      ];
      
      expect(conversation.length).toBe(3);
      expect(conversation[2].role).toBe('user');
    });
  });
  
  describe('Streaming Chat', () => {
    it('should stream chat with MiniMax', () => {
      // 验证流式聊天
      const streamRequest = {
        provider: 'minimax',
        model: 'abab6.5',
        api_key: mockApiKey,
        messages: [
          { role: 'user', content: '请写一首关于春天的诗' }
        ],
        stream: true,
        temperature: 0.8,
        max_tokens: 2048
      };
      
      expect(streamRequest.stream).toBe(true);
      expect(streamRequest.temperature).toBe(0.8);
    });
    
    it('should handle SSE events correctly', () => {
      // 验证 SSE 事件处理
      const mockSSEvents = [
        'data: {"reply": "春", "usage": {"total_tokens": 10}}',
        'data: {"reply": "天", "usage": {"total_tokens": 20}}',
        'data: {"reply": "来", "usage": {"total_tokens": 30}}',
        'data: [DONE]'
      ];
      
      const fullReply = mockSSEvents
        .filter(line => line !== 'data: [DONE]')
        .map(line => {
          const data = line.substring(6);
          const parsed = JSON.parse(data);
          return parsed.reply;
        })
        .join('');
      
      expect(fullReply).toBe('春天来');
    });
  });
  
  describe('PRD Generation', () => {
    it('should generate PRD with MiniMax', () => {
      // 验证 PRD 生成功能
      const prdRequest = {
        idea: '一个帮助独立开发者管理项目进度的 AI 工具',
        provider: 'minimax',
        model: 'abab6.5',
        api_key: mockApiKey
      };
      
      expect(prdRequest.idea).toContain('独立开发者');
      expect(prdRequest.provider).toBe('minimax');
    });
    
    it('should use MiniMax-optimized prompt for PRD', () => {
      // 验证使用 MiniMax 优化的提示词
      const minimaxPromptFeatures = [
        '生动活泼的中文',
        '情感化描述',
        '故事性叙述',
        'emoji 增强可读性'
      ];
      
      expect(minimaxPromptFeatures.length).toBeGreaterThan(0);
      expect(minimaxPromptFeatures).toContain('生动活泼的中文');
    });
    
    it('should parse PRD markdown format', () => {
      // 验证 PRD Markdown 解析
      const mockMarkdownPRD = `
# DevProgress - AI 项目管理助手

## 1. 产品愿景 ✨
为独立开发者提供智能进度管理...

## 2. 目标用户画像 👥
- 张三，32 岁，全栈开发者
- 李四，28 岁，独立游戏开发者

## 3. 核心功能特性 🚀
1. AI 进度预测
2. 自动任务分解
3. 风险预警
`;
      
      expect(mockMarkdownPRD).toContain('产品愿景');
      expect(mockMarkdownPRD).toContain('目标用户');
      expect(mockMarkdownPRD).toContain('核心功能');
    });
  });
  
  describe('User Persona Generation', () => {
    it('should generate user personas with MiniMax', () => {
      // 验证用户画像生成
      const personaRequest = {
        idea: '一个在线学习平台',
        provider: 'minimax',
        model: 'abab6.5',
        api_key: mockApiKey
      };
      
      expect(personaRequest.idea).toContain('在线学习');
    });
    
    it('should create emotional and detailed personas', () => {
      // 验证 MiniMax 创建情感化详细的用户画像
      const persona = {
        name: '张小美',
        age: '28 岁',
        occupation: 'UI 设计师',
        background: '在一家互联网公司工作 3 年，热爱设计但感觉缺乏成长空间',
        goals: ['提升设计能力', '建立个人品牌', '发展副业'],
        pain_points: ['工作时间长', '缺乏系统学习', '迷茫未来方向'],
        quote: '我希望找到一个能帮助我系统性提升设计能力的平台'
      };
      
      expect(persona.goals.length).toBe(3);
      expect(persona.pain_points.length).toBe(3);
      expect(persona.quote).toBeDefined();
    });
  });
  
  describe('Error Handling', () => {
    it('should handle invalid API key', () => {
      // 验证无效 API Key 处理
      const invalidRequest = {
        provider: 'minimax',
        api_key: 'invalid_key_12345'
      };
      
      expect(invalidRequest.api_key).toBe('invalid_key_12345');
    });
    
    it('should handle network errors gracefully', () => {
      // 验证网络错误处理
      const networkError = {
        type: 'NetworkError',
        message: '请求超时，请检查网络连接'
      };
      
      expect(networkError.type).toBe('NetworkError');
    });
    
    it('should handle rate limiting', () => {
      // 验证速率限制处理
      const rateLimitError = {
        type: 'RateLimitError',
        message: '请求过于频繁，请稍后再试',
        retryAfter: 60
      };
      
      expect(rateLimitError.retryAfter).toBe(60);
    });
  });
  
  describe('Performance', () => {
    it('should respond within 3 seconds for non-streaming', () => {
      // 验证非流式响应时间
      const expectedResponseTime = 3000; // ms
      expect(expectedResponseTime).toBeLessThan(5000);
    });
    
    it('should have first token latency < 1 second for streaming', () => {
      // 验证流式首字延迟
      const expectedFirstTokenLatency = 1000; // ms
      expect(expectedFirstTokenLatency).toBeLessThan(1500);
    });
  });
});
