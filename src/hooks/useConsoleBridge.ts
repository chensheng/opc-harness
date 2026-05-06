import { invoke } from '@tauri-apps/api/core';

/**
 * 安全的对象序列化函数,处理循环引用和不可序列化的值
 */
function safeSerialize(value: unknown): string {
  try {
    // 处理特殊类型
    if (value === undefined) return 'undefined';
    if (value === null) return 'null';
    if (typeof value === 'function') return value.toString();
    if (typeof value === 'symbol') return value.toString();
    
    // 对于对象,尝试 JSON.stringify
    if (typeof value === 'object') {
      // 使用 WeakSet 检测循环引用
      const seen = new WeakSet();
      return JSON.stringify(value, (key, val) => {
        if (typeof val === 'object' && val !== null) {
          if (seen.has(val)) {
            return '[Circular]';
          }
          seen.add(val);
        }
        return val;
      }, 2);
    }
    
    // 其他类型直接转换
    return String(value);
  } catch (error) {
    // 降级策略:序列化失败时返回占位符
    console.warn('[ConsoleBridge] Serialization failed:', error);
    return '[Serialization Error]';
  }
}

/**
 * Console Bridge Hook - 拦截前端 console 方法并转发到后端
 * 
 * @param enabled - 是否启用 console bridge(默认根据开发模式自动判断)
 */
export function useConsoleBridge(enabled?: boolean) {
  initializeConsoleBridge(enabled);
  return { enabled: shouldEnableConsoleBridge(enabled) };
}

/**
 * 检查是否应该启用 console bridge
 */
function shouldEnableConsoleBridge(enabled?: boolean): boolean {
  const meta = import.meta as any;
  return enabled ?? (meta.env?.DEV || meta.env?.VITE_ENABLE_CONSOLE_BRIDGE === 'true');
}

/**
 * 初始化 Console Bridge - 可以在模块级别调用
 */
export function initializeConsoleBridge(enabled?: boolean) {
  if (!shouldEnableConsoleBridge(enabled)) {
    return;
  }

  // 防止重复初始化
  if ((window as any).__consoleBridgeInitialized) {
    console.log('[ConsoleBridge] Already initialized, skipping');
    return;
  }
  (window as any).__consoleBridgeInitialized = true;

  // 保存原始 console 方法
  const originalConsole = {
    log: console.log.bind(console),
    info: console.info.bind(console),
    warn: console.warn.bind(console),
    error: console.error.bind(console),
    debug: console.debug.bind(console),
  };

  console.log('[ConsoleBridge] Initialized - Frontend logs will be forwarded to backend');

  /**
   * 创建 console 方法包装器
   */
  const createConsoleWrapper = (level: string, originalMethod: (...args: any[]) => void) => {
    return (...args: any[]) => {
      // 1. 首先调用原始 console 方法,确保 DevTools 仍能看到日志
      originalMethod(...args);

      // 2. 异步发送到后端(不阻塞 UI)
      try {
        // 提取主消息(第一个参数)
        const mainMessage = args[0];
        
        // 序列化额外参数
        const metadata = args.slice(1)
          .map(arg => safeSerialize(arg))
          .filter(str => str.length > 0)
          .join(' ');

        // 构建完整消息
        const fullMessage = metadata 
          ? `${safeSerialize(mainMessage)} ${metadata}`
          : safeSerialize(mainMessage);

        // 异步调用 Tauri command
        invoke('console_log', { 
          level, 
          message: fullMessage 
        }).catch(err => {
          // invoke 失败不影响原始 console 功能
          originalConsole.error('[ConsoleBridge] Failed to send log to backend:', err);
        });
      } catch (error) {
        // 捕获任何意外错误,确保不影响原始 console
        console.warn('[ConsoleBridge] Error in console wrapper:', error);
      }
    };
  };

  // 拦截 console 方法
  console.log = createConsoleWrapper('log', originalConsole.log);
  console.info = createConsoleWrapper('info', originalConsole.info);
  console.warn = createConsoleWrapper('warn', originalConsole.warn);
  console.error = createConsoleWrapper('error', originalConsole.error);
  console.debug = createConsoleWrapper('debug', originalConsole.debug);

  console.log('[ConsoleBridge] Initialized - Frontend logs will be forwarded to backend');
}

/**
 * 清理 console bridge,恢复原始 console 方法
 */
export function cleanupConsoleBridge() {
  // 注意:实际使用中很少需要清理,因为应用生命周期内只需初始化一次
  // 此函数主要用于测试或特殊情况
  console.log('[ConsoleBridge] Cleanup not implemented - console methods are permanently wrapped');
}
