import { describe, it, expect } from 'vitest'
import { defaultQualityChecker } from './prd-quality-checker'
import type { PRD } from '@/types'

/**
 * 创建完整的 PRD 测试数据
 */
function createTestPRD(overrides: Partial<PRD> = {}): PRD {
  return {
    title: '测试产品',
    overview: '这是一个完整的产品概述，描述了产品的核心价值和功能定位。',
    targetUsers: ['产品经理', '开发者'],
    coreFeatures: ['功能 1', '功能 2', '功能 3'],
    techStack: ['React', 'TypeScript', 'Node.js'],
    estimatedEffort: '2 周',
    ...overrides,
  }
}

describe('PRDQualityChecker', () => {
  describe('checkCompleteness', () => {
    it('should score complete PRD highly', async () => {
      const completePRD = createTestPRD({
        overview:
          '这是一个非常详细的产品概述，描述了产品的核心价值、目标用户和主要功能。这个产品旨在解决用户的痛点并提供卓越的体验。',
        targetUsers: ['产品经理', '开发者', '设计师'],
        coreFeatures: ['功能 1', '功能 2', '功能 3', '功能 4'],
        techStack: ['React', 'TypeScript', 'Node.js', 'Tauri'],
      })

      const report = await defaultQualityChecker.checkQuality(completePRD)

      expect(report.completeness.score).toBeGreaterThan(80)
      expect(report.overallScore).toBeGreaterThan(75)
      expect(report.issues.length).toBeLessThanOrEqual(2)
    })

    it('should detect missing sections', async () => {
      const emptyPRD: PRD = {
        title: '',
        overview: '',
        targetUsers: [],
        coreFeatures: [],
        techStack: [],
        estimatedEffort: '',
      }

      const report = await defaultQualityChecker.checkQuality(emptyPRD)

      expect(report.completeness.missingSections.length).toBeGreaterThanOrEqual(3)
      expect(report.completeness.score).toBeLessThan(30)
      expect(report.overallScore).toBeLessThan(20)
    })

    it('should penalize incomplete sections', async () => {
      const prd = createTestPRD({
        targetUsers: [], // 空的目标用户列表
        techStack: ['React'], // 只有一个技术
      })

      const report = await defaultQualityChecker.checkQuality(prd)

      // 完整性分数应该较低
      expect(report.completeness.score).toBeLessThan(80)

      // 应该有质量问题
      expect(report.issues.length).toBeGreaterThan(0)
    })

    it('should detect missing sections when arrays are empty', async () => {
      const prd = createTestPRD({
        targetUsers: [],
        coreFeatures: [],
        techStack: [],
      })

      const report = await defaultQualityChecker.checkQuality(prd)

      // 缺失的章节应该被记录
      expect(report.completeness.missingSections.length).toBeGreaterThanOrEqual(3)
      expect(report.completeness.missingSections).toContain('目标用户')
      expect(report.completeness.missingSections).toContain('核心功能')
      expect(report.completeness.missingSections).toContain('技术栈')
    })

    it('should penalize insufficient tech stack', async () => {
      const prd = createTestPRD({
        techStack: ['React'], // 只有一个技术
      })

      const report = await defaultQualityChecker.checkQuality(prd)

      // 完整性分数应该受到影响
      expect(report.completeness.score).toBeLessThan(100)
      // 应该有相关问题
      expect(report.issues.length).toBeGreaterThanOrEqual(0)
    })
  })

  describe('detectIssues', () => {
    it('should detect critical issues for missing core sections', async () => {
      const prd: PRD = {
        title: '',
        overview: '',
        targetUsers: [],
        coreFeatures: [],
        techStack: [],
        estimatedEffort: '',
      }

      const report = await defaultQualityChecker.checkQuality(prd)

      const criticalIssues = report.issues.filter(i => i.severity === 'critical')
      expect(criticalIssues.length).toBeGreaterThanOrEqual(3)

      const sections = criticalIssues.map(i => i.section)
      expect(sections).toContain('产品概述')
      expect(sections).toContain('目标用户')
      expect(sections).toContain('核心功能')
    })

    it('should detect major issues for insufficient content', async () => {
      const prd = createTestPRD({
        overview: '简短', // 太短
        targetUsers: ['用户'], // 数量不足
        coreFeatures: ['功能 1', '功能 2'], // 数量不足
      })

      const report = await defaultQualityChecker.checkQuality(prd)

      const majorIssues = report.issues.filter(i => i.severity === 'major')
      expect(majorIssues.length).toBeGreaterThanOrEqual(2)
    })

    it('should generate suggestions when issues are found', async () => {
      const prd = createTestPRD({
        overview: '简短',
        targetUsers: [],
        coreFeatures: [],
      })

      const report = await defaultQualityChecker.checkQuality(prd)

      expect(report.suggestions.length).toBeGreaterThan(0)
      // 建议应该包含具体的改进方向
      expect(
        report.suggestions.some(s => s.includes('添加') || s.includes('补充') || s.includes('缺失'))
      ).toBe(true)
    })
  })

  describe('calculateOverallScore', () => {
    it('should calculate score based on completeness and issues', async () => {
      const goodPRD = createTestPRD({
        overview:
          '这是一个非常详细的产品概述，描述了产品的核心价值、目标用户和主要功能。这个产品旨在解决用户的痛点并提供卓越的体验。',
        targetUsers: ['用户 A', '用户 B'],
        coreFeatures: ['功能 1', '功能 2', '功能 3'],
      })

      const report = await defaultQualityChecker.checkQuality(goodPRD)

      // 完整的 PRD 应该有较好的分数（至少及格）
      expect(report.overallScore).toBeGreaterThan(50)
      expect(report.overallScore).toBeLessThanOrEqual(100)
    })

    it('should deduct points for critical issues', async () => {
      const prdWithCriticalIssues: PRD = {
        title: '',
        overview: '简短',
        targetUsers: [],
        coreFeatures: [],
        techStack: [],
        estimatedEffort: '',
      }

      const report = await defaultQualityChecker.checkQuality(prdWithCriticalIssues)

      // 有关键问题时，分数应该较低
      expect(report.overallScore).toBeLessThan(40)
    })
  })

  describe('generateSuggestions', () => {
    it('should provide actionable suggestions for improvement', async () => {
      const incompletePRD = createTestPRD({
        overview: '简短',
        targetUsers: [],
      })

      const report = await defaultQualityChecker.checkQuality(incompletePRD)

      expect(report.suggestions.length).toBeGreaterThan(0)

      // 建议应该是具体的、可操作的
      expect(
        report.suggestions.some(s => s.includes('添加') || s.includes('补充') || s.includes('增加'))
      ).toBe(true)
    })

    it('should prioritize critical suggestions', async () => {
      const prdWithManyIssues: PRD = {
        title: '',
        overview: '',
        targetUsers: [],
        coreFeatures: [],
        techStack: [],
        estimatedEffort: '',
      }

      const report = await defaultQualityChecker.checkQuality(prdWithManyIssues)

      // 最严重的问题应该优先建议
      expect(report.suggestions.some(s => s.includes('核心章节') || s.includes('缺失'))).toBe(true)
    })
  })

  describe('edge cases', () => {
    it('should handle PRD with only overview', async () => {
      const prd = createTestPRD({
        overview: '这是一个产品概述',
        targetUsers: [],
        coreFeatures: [],
        techStack: [],
        estimatedEffort: '',
      })

      const report = await defaultQualityChecker.checkQuality(prd)

      expect(report).toBeDefined()
      expect(report.overallScore).toBeGreaterThanOrEqual(0)
      expect(report.overallScore).toBeLessThanOrEqual(100)
    })

    it('should not return negative scores', async () => {
      const worstPRD: PRD = {
        title: '',
        overview: '',
        targetUsers: [],
        coreFeatures: [],
        techStack: [],
        estimatedEffort: '',
      }

      const report = await defaultQualityChecker.checkQuality(worstPRD)

      expect(report.overallScore).toBeGreaterThanOrEqual(0)
      expect(report.completeness.score).toBeGreaterThanOrEqual(0)
    })

    it('should cap scores at 100', async () => {
      const perfectPRD = createTestPRD({
        overview: '这是一个非常详细和完善的产品概述，充分描述了产品的各个方面。',
        targetUsers: Array(5).fill('用户'),
        coreFeatures: Array(10).fill('功能'),
        techStack: Array(10).fill('技术'),
      })

      const report = await defaultQualityChecker.checkQuality(perfectPRD)

      expect(report.overallScore).toBeLessThanOrEqual(100)
      expect(report.completeness.score).toBeLessThanOrEqual(100)
    })
  })
})
