/**
 * CLI Browser 网络请求 E2E 测试（纯 HTTP 版本）
 * 
 * 使用原生 Fetch API 进行网络相关的 E2E 测试
 * 无需浏览器，轻量快速
 * 
 * 测试场景:
 * - 请求响应验证
 * - 错误处理
 * - 性能基准
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
  // 确保报告目录存在
  try {
    mkdirSync(REPORT_DIR, { recursive: true });
  } catch {
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
          <h1>CLI Browser Network Test Report</h1>
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

describe('CLI Browser Network Requests', () => {
  
  it('should verify base URL is accessible', async () => {
    const testName = 'base-url-accessible';
    try {
      const response = await fetch(TEST_CONFIG.baseUrl);
      expect(response.status).toBe(200);
      
      const html = await response.text();
      expect(html).toContain('OPC-HARNESS');
      
      generateReport(testName, 'pass', 'Base URL accessible');
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should handle different HTTP methods', async () => {
    const testName = 'http-methods';
    try {
      // GET request
      const getResponse = await fetch(TEST_CONFIG.baseUrl);
      expect(getResponse.status).toBe(200);
      
      // HEAD request
      const headResponse = await fetch(TEST_CONFIG.baseUrl, { method: 'HEAD' });
      expect(headResponse.status).toBe(200);
      
      generateReport(testName, 'pass', 'HTTP methods OK');
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should verify response headers', async () => {
    const testName = 'response-headers';
    try {
      const response = await fetch(TEST_CONFIG.baseUrl);
      
      expect(response.headers.has('content-type')).toBe(true);
      
      generateReport(testName, 'pass', 'Headers verified');
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should measure response time', async () => {
    const testName = 'response-time';
    try {
      const startTime = Date.now();
      await fetch(TEST_CONFIG.baseUrl);
      const responseTime = Date.now() - startTime;
      
      console.log(`Response time: ${responseTime}ms`);
      expect(responseTime).toBeLessThan(5000);
      
      generateReport(testName, 'pass', `Response time: ${responseTime}ms`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should handle 404 responses gracefully', async () => {
    const testName = 'handle-404';
    try {
      const response = await fetch(`${TEST_CONFIG.baseUrl}/non-existent-page`);
      
      // 可能是 404 或者重定向到 200
      expect([200, 404]).toContain(response.status);
      
      generateReport(testName, 'pass', '404 handled gracefully');
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should verify content encoding', async () => {
    const testName = 'content-encoding';
    try {
      const response = await fetch(TEST_CONFIG.baseUrl);
      
      const contentEncoding = response.headers.get('content-encoding');
      const contentLength = response.headers.get('content-length');
      
      console.log(`Content-Encoding: ${contentEncoding || 'none'}`);
      console.log(`Content-Length: ${contentLength || 'unknown'}`);
      
      generateReport(testName, 'pass', `Encoding: ${contentEncoding || 'none'}`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });

  it('should handle concurrent requests', async () => {
    const testName = 'concurrent-requests';
    try {
      const startTime = Date.now();
      
      // 并发发起 3 个请求
      const [res1, res2, res3] = await Promise.all([
        fetch(TEST_CONFIG.baseUrl),
        fetch(TEST_CONFIG.baseUrl),
        fetch(TEST_CONFIG.baseUrl),
      ]);
      
      expect(res1.status).toBe(200);
      expect(res2.status).toBe(200);
      expect(res3.status).toBe(200);
      
      const totalTime = Date.now() - startTime;
      console.log(`Concurrent requests completed in ${totalTime}ms`);
      
      generateReport(testName, 'pass', `3 concurrent requests: ${totalTime}ms`);
    } catch (_error) {
      generateReport(testName, 'fail', String(_error));
      throw _error;
    }
  });
});
