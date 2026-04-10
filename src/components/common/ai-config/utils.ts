/**
 * AIConfig 工具函数
 */

/**
 * 格式化 token 数为更易读的单位
 */
export function formatTokens(tokens: number): string {
  if (tokens >= 1000000) {
    return `${(tokens / 1000000).toFixed(1)}M`
  } else if (tokens >= 1000) {
    return `${(tokens / 1000).toFixed(0)}K`
  }
  return tokens.toString()
}

/**
 * 提取错误消息
 */
export function extractErrorMessage(err: unknown): string {
  if (err instanceof Error) {
    return err.message
  } else if (typeof err === 'string') {
    return err
  } else if (err && typeof err === 'object') {
    try {
      return JSON.stringify(err, null, 2)
    } catch {
      return String(err)
    }
  }
  return String(err)
}

/**
 * 生成详细的验证错误信息
 */
export function generateValidationError(errorMessage: string, providerName: string): string {
  return `${errorMessage}

可能原因:
1. API Key 格式不正确
2. API Key 已过期或无效
3. 网络连接问题
4. ${providerName} API 服务不可用`
}
