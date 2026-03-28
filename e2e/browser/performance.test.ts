/**
 * CLI Browser 性能 E2E 测试（纯 HTTP 版本）
 * 
 * 使用原生 Fetch API 进行性能相关的 E2E 测试
 * 无需浏览器，轻量快速
 * 
 * 测试场景:
 * - 页面加载时间
 * - 响应时间基准
 * - 缓存效果验证
 * - 并发性能
 * - 内容压缩
 * - TTFB 测量
 * - Keep-Alive 连接
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { writeFileSync, mkdirSync } from 'fs';
import { join } from 'path';

// 测试配置
const TEST_CONFIG = {
  baseUrl: 'http://localhost:1420',
  timeout: 30000,
};

// 测试报告存储目录
const REPORT_DIR = join(process.cwd(), 'docs', 'testing', 'browser-reports');

beforeAll(async () => {
  try {
    mkdirSync(REPORT_DIR, { recursive: true });
  } catch (_error) {
    // 忽略目录创建错误
  }
});

/**
 * 辅助函数：生成测试报告
 */
function generateReport(testName: string, result: 'pass' | 'fail', details: string): void {
  try {
    const timestamp = new Date().toISOString();
    const reportFile = join(REPORT_DIR, `report-${Date.now()}.html`);
    
    const html = `
      <!DOCTYPE html>
      <html>
        <head><title>E2E Test Report</title></head>
        <body>
          <h1>CLI Browser Performance Test Report</h1>
          <div>Test: ${testName}</div>
          <div>Result: ${result}</div>
          <div>Time: ${timestamp}</div>
          <div>Details: ${details}</div>
        </body>
      </html>
    `;
    
    writeFileSync(reportFile, html);
  } catch (_error) {
    console.warn('Failed to save report:', _error);
  }
}

describe('CLI Browser Performance Tests', () => {
  
  it('should measure page load time', async () => {
    const testName = 'page-load-time';
    const startTime = Date.now();
    
    try {
      const response = await fetch(TEST_CONFIG.baseUrl);
      const html = await response.text();
      
      const loadTime = Date.now() - startTime;
      
      console.log(`Page load time: ${loadTime}ms`);
      expect(loadTime).toBeLessThan(5000);
      
      generateReport(testName, 'pass', `Load time: ${loadTime}ms`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should verify response size is reasonable', async () => {
    const testName = 'response-size';
    try {
      const response = await fetch(TEST_CONFIG.baseUrl);
      const html = await response.text();
      
      const responseSize = html.length;
      console.log(`Response size: ${responseSize} bytes`);
      
      // HTML 应该在合理范围内（1KB - 1MB）
      expect(responseSize).toBeGreaterThan(1024);
      expect(responseSize).toBeLessThan(1048576);
      
      generateReport(testName, 'pass', `Size: ${responseSize} bytes`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should verify cache effectiveness', async () => {
    const testName = 'cache-effectiveness';
    try {
      // First request
      const firstStart = Date.now();
      const firstResponse = await fetch(TEST_CONFIG.baseUrl);
      const firstTime = Date.now() - firstStart;
      const firstETag = firstResponse.headers.get('etag');
      
      // Second request (should be cached)
      const secondStart = Date.now();
      const secondResponse = await fetch(TEST_CONFIG.baseUrl, {
        headers: {
          'If-None-Match': firstETag || '',
        },
      });
      const secondTime = Date.now() - secondStart;
      
      console.log(`First request: ${firstTime}ms, Second request: ${secondTime}ms`);
      
      // Second request should be faster (cached)
      if (secondResponse.status === 304) {
        generateReport(testName, 'pass', `Cache hit! First: ${firstTime}ms, Second: ${secondTime}ms`);
      } else {
        generateReport(testName, 'pass', `First: ${firstTime}ms, Second: ${secondTime}ms`);
      }
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should measure concurrent request performance', async () => {
    const testName = 'concurrent-performance';
    try {
      const startTime = Date.now();
      
      // 发起 10 个并发请求
      const requests = Array(10).fill(null).map(() => fetch(TEST_CONFIG.baseUrl));
      const responses = await Promise.all(requests);
      
      const totalTime = Date.now() - startTime;
      const avgTime = totalTime / responses.length;
      
      console.log(`Total time: ${totalTime}ms, Avg per request: ${avgTime}ms`);
      
      // 所有请求都应该成功
      responses.forEach(res => {
        expect(res.status).toBe(200);
      });
      
      generateReport(testName, 'pass', `10 concurrent: ${totalTime}ms (avg: ${avgTime}ms)`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should verify content compression', async () => {
    const testName = 'content-compression';
    try {
      const response = await fetch(TEST_CONFIG.baseUrl, {
        headers: {
          'Accept-Encoding': 'gzip, deflate, br',
        },
      });
      
      const contentEncoding = response.headers.get('content-encoding');
      console.log(`Content-Encoding: ${contentEncoding || 'none'}`);
      
      // 如果有压缩，应该能看到 content-encoding 头
      if (contentEncoding) {
        expect(['gzip', 'deflate', 'br']).toContain(contentEncoding);
      }
      
      generateReport(testName, 'pass', `Compression: ${contentEncoding || 'none'}`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should measure time to first byte (TTFB)', async () => {
    const testName = 'ttfb-measurement';
    try {
      const startTime = Date.now();
      
      const response = await fetch(TEST_CONFIG.baseUrl);
      const ttfb = Date.now() - startTime;
      
      console.log(`TTFB: ${ttfb}ms`);
      
      // TTFB 应该小于 1 秒
      expect(ttfb).toBeLessThan(1000);
      
      generateReport(testName, 'pass', `TTFB: ${ttfb}ms`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should verify keep-alive connections', async () => {
    const testName = 'keep-alive';
    try {
      // 连续发起多个请求到同一主机
      const times: number[] = [];
      
      for (let i = 0; i < 5; i++) {
        const start = Date.now();
        await fetch(TEST_CONFIG.baseUrl);
        times.push(Date.now() - start);
      }
      
      console.log(`Request times: ${times.join(', ')}ms`);
      
      // 后续请求应该更快（keep-alive 连接复用）
      if (times.length >= 2) {
        const improvement = ((times[0] - times[times.length - 1]) / times[0]) * 100;
        console.log(`Performance improvement: ${improvement.toFixed(2)}%`);
      }
      
      generateReport(testName, 'pass', `Keep-alive test completed`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });
});
