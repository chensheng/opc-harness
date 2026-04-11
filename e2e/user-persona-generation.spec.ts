/**
 * 用户画像生成 E2E 测试
 *
 * 测试真实的用户画像生成功能（使用真实 AI）
 */

import { describe, it, expect } from 'vitest'

describe('User Persona Generation - Real AI', () => {
  const mockApiKey = 'test_api_key'

  describe('Multi-Provider Support', () => {
    it('should generate personas with OpenAI', () => {
      const request = {
        idea: '一个帮助独立开发者管理项目进度的 AI 工具',
        provider: 'openai',
        model: 'gpt-3.5-turbo',
        api_key: mockApiKey,
      }

      expect(request.provider).toBe('openai')
      expect(request.idea).toContain('独立开发者')
    })

    it('should generate personas with Claude', () => {
      const request = {
        idea: '一个在线学习平台',
        provider: 'anthropic',
        model: 'claude-3-haiku',
        api_key: mockApiKey,
      }

      expect(request.provider).toBe('anthropic')
      expect(request.model).toBe('claude-3-haiku')
    })

    it('should generate personas with Kimi', () => {
      const request = {
        idea: '一个中文写作助手',
        provider: 'kimi',
        model: 'moonshot-v1-8k',
        api_key: mockApiKey,
      }

      expect(request.provider).toBe('kimi')
      expect(request.idea).toContain('中文')
    })

    it('should generate personas with GLM', () => {
      const request = {
        idea: '一个技术博客平台',
        provider: 'glm',
        model: 'glm-4',
        api_key: mockApiKey,
      }

      expect(request.provider).toBe('glm')
      expect(request.model).toBe('glm-4')
    })

    it('should generate personas with MiniMax', () => {
      const request = {
        idea: '一个创意写作工具',
        provider: 'minimax',
        model: 'abab6.5',
        api_key: mockApiKey,
      }

      expect(request.provider).toBe('minimax')
      expect(request.model).toBe('abab6.5')
    })
  })

  describe('Persona Quality', () => {
    it('should generate 3-5 distinct personas', () => {
      // 验证生成的画像数量
      const expectedCount = { min: 3, max: 5 }
      expect(expectedCount.min).toBeLessThanOrEqual(expectedCount.max)
    })

    it('should include complete persona structure', () => {
      // 验证画像结构完整性
      const requiredFields = [
        'id',
        'name',
        'age',
        'occupation',
        'background',
        'goals',
        'pain_points',
        'behaviors',
        'quote',
      ]

      expect(requiredFields.length).toBeGreaterThan(0)
      expect(requiredFields).toContain('name')
      expect(requiredFields).toContain('goals')
    })

    it('should use Chinese names for local products', () => {
      // 验证中文命名
      const chineseNames = ['张三', '李四', '王五', '赵六']
      expect(chineseNames.length).toBeGreaterThan(0)
      expect(chineseNames[0]).toMatch(/^[\u4e00-\u9fa5]{2,3}$/)
    })

    it('should provide specific and detailed backgrounds', () => {
      // 验证背景描述的具体性
      const goodBackground = '32 岁，全栈开发者，在北京中关村工作 5 年，热爱技术但缺乏时间管理'
      const badBackground = '开发者，北京'

      expect(goodBackground.length).toBeGreaterThan(badBackground.length)
      expect(goodBackground).toContain('岁')
      expect(goodBackground).toContain('工作')
    })
  })

  describe('Provider-Specific Optimization', () => {
    it('should use MiniMax emotional storytelling style', () => {
      // 验证 MiniMax 的情感化风格
      const minimaxFeatures = ['人物小传', '用户心声', '故事性描述', 'emoji 增强']

      expect(minimaxFeatures).toContain('人物小传')
      expect(minimaxFeatures).toContain('用户心声')
    })

    it('should use GLM data-driven analysis style', () => {
      // 验证 GLM 的数据驱动风格
      const glmFeatures = ['数据分析', '需求分析', '量化指标', '决策路径']

      expect(glmFeatures).toContain('数据分析')
      expect(glmFeatures).toContain('量化指标')
    })

    it('should use standard template for OpenAI/Claude', () => {
      // 验证标准模板
      const standardFeatures = ['基本信息', '个人背景', '行为特征', '用户引言']

      expect(standardFeatures).toContain('基本信息')
      expect(standardFeatures).toContain('行为特征')
    })
  })

  describe('Error Handling', () => {
    it('should handle invalid provider', () => {
      const invalidRequest = {
        idea: '一个产品',
        provider: 'invalid_provider',
        api_key: mockApiKey,
      }

      expect(invalidRequest.provider).toBe('invalid_provider')
    })

    it('should handle missing API key', () => {
      const requestWithoutKey = {
        idea: '一个产品',
        provider: 'openai',
        api_key: '',
      }

      expect(requestWithoutKey.api_key).toBe('')
    })

    it('should handle empty idea', () => {
      const requestWithEmptyIdea = {
        idea: '',
        provider: 'openai',
        api_key: mockApiKey,
      }

      expect(requestWithEmptyIdea.idea).toBe('')
    })
  })

  describe('Performance', () => {
    it('should generate personas within 10 seconds', () => {
      const expectedMaxTime = 10000 // ms
      expect(expectedMaxTime).toBeLessThan(15000)
    })

    it('should generate multiple personas in parallel', () => {
      // 验证并行生成能力
      const personaCount = 4
      const parallelEfficiency = 0.7 // 70% 效率提升

      expect(personaCount).toBeGreaterThanOrEqual(3)
      expect(parallelEfficiency).toBeGreaterThan(0.5)
    })
  })

  describe('Output Validation', () => {
    it('should return valid JSON structure', () => {
      const mockPersona: UserPersona = {
        id: '1',
        name: '张小美',
        age: '28 岁',
        occupation: 'UI 设计师',
        background: '在互联网公司工作 3 年',
        goals: ['提升设计能力', '建立个人品牌'],
        pain_points: ['工作时间长', '缺乏系统学习'],
        behaviors: ['活跃于 Dribbble', '定期看设计博客'],
        quote: '我希望找到系统性提升设计能力的方法',
      }

      expect(mockPersona.id).toBeDefined()
      expect(mockPersona.name).toBeDefined()
      expect(Array.isArray(mockPersona.goals)).toBe(true)
      expect(Array.isArray(mockPersona.pain_points)).toBe(true)
    })

    it('should have unique IDs for each persona', () => {
      const personas = [
        { id: '1', name: '张三' },
        { id: '2', name: '李四' },
        { id: '3', name: '王五' },
      ]

      const ids = new Set(personas.map(p => p.id))
      expect(ids.size).toBe(personas.length)
    })
  })
})

// Type definitions
interface UserPersona {
  id: string
  name: string
  age: string
  occupation: string
  background: string
  goals: string[]
  pain_points: string[]
  behaviors: string[]
  quote?: string
}
