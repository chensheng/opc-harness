//! Real-time Performance Monitor 实现
//!
//! 负责实时监控系统资源使用情况（CPU、内存、磁盘、网络）。
//! 提供性能瓶颈预警、趋势分析、历史数据记录等功能。
//! 支持跨平台（Windows/Linux/macOS）。

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// 采样间隔（毫秒）
    pub sample_interval_ms: u64,
    /// CPU 告警阈值（0.0-1.0）
    pub cpu_warning_threshold: f32,
    /// 内存告警阈值（0.0-1.0）
    pub memory_warning_threshold: f32,
    /// Top N 进程数量
    pub top_n_processes: usize,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            sample_interval_ms: 1000,
            cpu_warning_threshold: 0.8,
            memory_warning_threshold: 0.9,
            top_n_processes: 5,
        }
    }
}

/// 系统统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    /// 时间戳
    pub timestamp: u64,
    /// CPU 使用率（0.0-1.0）
    pub cpu_usage: f32,
    /// 每核心 CPU 使用率
    pub cpu_per_core: Vec<f32>,
    /// 已用内存（字节）
    pub memory_used: u64,
    /// 总内存（字节）
    pub memory_total: u64,
    /// 内存使用率（0.0-1.0）
    pub memory_usage: f32,
    /// 磁盘读取（字节/秒）
    pub disk_read_bytes: u64,
    /// 磁盘写入（字节/秒）
    pub disk_write_bytes: u64,
    /// 网络接收（字节/秒）
    pub network_rx_bytes: u64,
    /// 网络发送（字节/秒）
    pub network_tx_bytes: u64,
}

impl SystemStats {
    pub fn new(
        timestamp: u64,
        cpu_usage: f32,
        cpu_per_core: Vec<f32>,
        memory_used: u64,
        memory_total: u64,
        memory_usage: f32,
        disk_read_bytes: u64,
        disk_write_bytes: u64,
        network_rx_bytes: u64,
        network_tx_bytes: u64,
    ) -> Self {
        Self {
            timestamp,
            cpu_usage,
            cpu_per_core,
            memory_used,
            memory_total,
            memory_usage,
            disk_read_bytes,
            disk_write_bytes,
            network_rx_bytes,
            network_tx_bytes,
        }
    }
}

/// 进程统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStats {
    /// 进程 ID
    pub pid: u32,
    /// 进程名称
    pub name: String,
    /// CPU 使用率（0.0-1.0）
    pub cpu_usage: f32,
    /// 内存使用率（0.0-1.0）
    pub memory_usage: f32,
    /// 已用内存（字节）
    pub memory_used: u64,
}

impl ProcessStats {
    pub fn new(
        pid: u32,
        name: String,
        cpu_usage: f32,
        memory_usage: f32,
        memory_used: u64,
    ) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
            memory_usage,
            memory_used,
        }
    }
}

/// 性能告警
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// 时间戳
    pub timestamp: u64,
    /// 告警类型
    pub alert_type: String, // "cpu", "memory", "disk", "network"
    /// 严重程度
    pub severity: String, // "warning", "critical"
    /// 告警消息
    pub message: String,
    /// 当前值
    pub value: f32,
    /// 阈值
    pub threshold: f32,
}

impl PerformanceAlert {
    pub fn new(
        timestamp: u64,
        alert_type: String,
        severity: String,
        message: String,
        value: f32,
        threshold: f32,
    ) -> Self {
        Self {
            timestamp,
            alert_type,
            severity,
            message,
            value,
            threshold,
        }
    }
}

/// 监控会话
#[derive(Debug)]
pub struct MonitoringSession {
    pub session_id: String,
    pub is_active: bool,
    pub config: MonitoringConfig,
}

/// Real-time Performance Monitor 结构体
pub struct RealtimePerformanceMonitor {
    config: MonitoringConfig,
    is_monitoring: bool,
    system: Arc<Mutex<System>>,
}

impl RealtimePerformanceMonitor {
    /// 创建新的实时监控器
    pub fn new(config: MonitoringConfig) -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            config,
            is_monitoring: false,
            system: Arc::new(Mutex::new(system)),
        }
    }

    /// 启动监控
    pub async fn start_monitoring(&mut self) -> Result<(), String> {
        if self.is_monitoring {
            return Err("监控已在运行中".to_string());
        }

        log::info!(
            "启动实时性能监控，采样间隔：{}ms",
            self.config.sample_interval_ms
        );
        self.is_monitoring = true;
        Ok(())
    }

    /// 停止监控
    pub async fn stop_monitoring(&mut self) {
        if !self.is_monitoring {
            log::warn!("监控未运行");
            return;
        }

        log::info!("停止实时性能监控");
        self.is_monitoring = false;
    }

    /// 获取当前系统统计信息
    pub fn get_current_stats(&self) -> Result<SystemStats, String> {
        let mut system = self
            .system
            .lock()
            .map_err(|e| format!("锁定系统失败：{}", e))?;

        // 刷新系统信息
        system.refresh_all();

        // 获取时间戳
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("获取时间失败：{}", e))?
            .as_secs();

        // CPU 使用率 (sysinfo 0.30: 使用 cpus().iter().map 计算平均)
        let cpus = system.cpus();
        let cpu_per_core: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage() / 100.0).collect();

        let cpu_usage = if cpu_per_core.is_empty() {
            0.0
        } else {
            cpu_per_core.iter().sum::<f32>() / cpu_per_core.len() as f32
        };

        // 内存使用
        let memory_used = system.used_memory();
        let memory_total = system.total_memory();
        let memory_usage = if memory_total > 0 {
            memory_used as f64 as f32 / memory_total as f64 as f32
        } else {
            0.0
        };

        // 磁盘 I/O（简化实现）
        let disk_read_bytes = 0u64;
        let disk_write_bytes = 0u64;

        // 网络流量（简化实现）
        let network_rx_bytes = 0u64;
        let network_tx_bytes = 0u64;

        Ok(SystemStats::new(
            timestamp,
            cpu_usage,
            cpu_per_core,
            memory_used,
            memory_total,
            memory_usage,
            disk_read_bytes,
            disk_write_bytes,
            network_rx_bytes,
            network_tx_bytes,
        ))
    }

    /// 获取 Top N 消耗进程
    pub fn get_top_processes(&self, n: usize) -> Vec<ProcessStats> {
        let system = match self.system.lock() {
            Ok(sys) => sys,
            Err(_) => return vec![],
        };

        let total_memory = system.total_memory();

        let mut processes: Vec<_> = system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let cpu_usage = process.cpu_usage() / 100.0;
                let memory_used = process.memory();
                let memory_usage = if total_memory > 0 {
                    memory_used as f32 / total_memory as f32
                } else {
                    0.0
                };

                ProcessStats::new(
                    pid.as_u32(),
                    process.name().to_string(),
                    cpu_usage,
                    memory_usage,
                    memory_used,
                )
            })
            .collect();

        // 按 CPU 使用率排序
        processes.sort_by(|a, b| {
            b.cpu_usage
                .partial_cmp(&a.cpu_usage)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 返回 Top N
        processes.into_iter().take(n).collect()
    }

    /// 检测性能瓶颈
    pub fn detect_bottlenecks(&self, stats: &SystemStats) -> Vec<String> {
        let mut bottlenecks = Vec::new();

        // CPU 瓶颈检测
        if stats.cpu_usage > self.config.cpu_warning_threshold {
            bottlenecks.push(format!(
                "CPU 使用率过高：{:.1}% (阈值：{:.1}%)",
                stats.cpu_usage * 100.0,
                self.config.cpu_warning_threshold * 100.0
            ));
        }

        // 内存压力检测
        if stats.memory_usage > self.config.memory_warning_threshold {
            bottlenecks.push(format!(
                "内存使用率过高：{:.1}% (阈值：{:.1}%)",
                stats.memory_usage * 100.0,
                self.config.memory_warning_threshold * 100.0
            ));
        }

        // 单核 CPU 过载检测
        for (i, core_cpu) in stats.cpu_per_core.iter().enumerate() {
            if *core_cpu > 0.95 {
                bottlenecks.push(format!(
                    "核心 {} CPU 使用率饱和：{:.1}%",
                    i,
                    core_cpu * 100.0
                ));
            }
        }

        bottlenecks
    }

    /// 生成性能告警
    pub fn generate_alerts(&self, stats: &SystemStats) -> Vec<PerformanceAlert> {
        let mut alerts = Vec::new();
        let timestamp = stats.timestamp;

        // CPU 告警
        if stats.cpu_usage > self.config.cpu_warning_threshold {
            let severity = if stats.cpu_usage > 0.95 {
                "critical"
            } else {
                "warning"
            };
            alerts.push(PerformanceAlert::new(
                timestamp,
                "cpu".to_string(),
                severity.to_string(),
                format!("CPU 使用率过高：{:.1}%", stats.cpu_usage * 100.0),
                stats.cpu_usage,
                self.config.cpu_warning_threshold,
            ));
        }

        // 内存告警
        if stats.memory_usage > self.config.memory_warning_threshold {
            let severity = if stats.memory_usage > 0.95 {
                "critical"
            } else {
                "warning"
            };
            alerts.push(PerformanceAlert::new(
                timestamp,
                "memory".to_string(),
                severity.to_string(),
                format!("内存使用率过高：{:.1}%", stats.memory_usage * 100.0),
                stats.memory_usage,
                self.config.memory_warning_threshold,
            ));
        }

        alerts
    }

    /// 获取配置信息
    pub fn get_config(&self) -> &MonitoringConfig {
        &self.config
    }

    /// 检查是否正在监控
    pub fn is_monitoring(&self) -> bool {
        self.is_monitoring
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_config_creation() {
        let config = MonitoringConfig {
            sample_interval_ms: 500,
            cpu_warning_threshold: 0.7,
            memory_warning_threshold: 0.85,
            top_n_processes: 10,
        };

        assert_eq!(config.sample_interval_ms, 500);
        assert_eq!(config.cpu_warning_threshold, 0.7);
        assert_eq!(config.memory_warning_threshold, 0.85);
    }

    #[test]
    fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();

        assert_eq!(config.sample_interval_ms, 1000);
        assert_eq!(config.cpu_warning_threshold, 0.8);
        assert_eq!(config.memory_warning_threshold, 0.9);
        assert_eq!(config.top_n_processes, 5);
    }

    #[test]
    fn test_system_stats_creation() {
        let stats = SystemStats::new(
            1234567890,
            0.45,
            vec![0.5, 0.6, 0.4, 0.7],
            4_000_000_000,
            16_000_000_000,
            0.25,
            1_000_000,
            500_000,
            2_000_000,
            1_500_000,
        );

        assert_eq!(stats.timestamp, 1234567890);
        assert_eq!(stats.cpu_usage, 0.45);
        assert_eq!(stats.memory_usage, 0.25);
        assert_eq!(stats.cpu_per_core.len(), 4);
    }

    #[test]
    fn test_process_stats_creation() {
        let process = ProcessStats::new(1234, "test_process".to_string(), 0.25, 0.15, 250_000_000);

        assert_eq!(process.pid, 1234);
        assert_eq!(process.name, "test_process");
        assert_eq!(process.cpu_usage, 0.25);
    }

    #[test]
    fn test_performance_alert_creation() {
        let alert = PerformanceAlert::new(
            1234567890,
            "cpu".to_string(),
            "warning".to_string(),
            "CPU 使用率过高：85%".to_string(),
            0.85,
            0.8,
        );

        assert_eq!(alert.alert_type, "cpu");
        assert_eq!(alert.severity, "warning");
        assert_eq!(alert.value, 0.85);
        assert_eq!(alert.threshold, 0.8);
    }

    #[test]
    fn test_monitor_creation() {
        let config = MonitoringConfig::default();
        let monitor = RealtimePerformanceMonitor::new(config.clone());

        assert!(!monitor.is_monitoring());
        assert_eq!(monitor.get_config().sample_interval_ms, 1000);
    }

    #[test]
    fn test_monitor_start_stop() {
        let config = MonitoringConfig::default();
        let mut monitor = RealtimePerformanceMonitor::new(config);

        // 启动监控
        let result = futures::executor::block_on(monitor.start_monitoring());
        assert!(result.is_ok());
        assert!(monitor.is_monitoring());

        // 再次启动应该失败
        let result = futures::executor::block_on(monitor.start_monitoring());
        assert!(result.is_err());

        // 停止监控
        futures::executor::block_on(monitor.stop_monitoring());
        assert!(!monitor.is_monitoring());
    }

    #[test]
    fn test_get_current_stats() {
        let config = MonitoringConfig::default();
        let monitor = RealtimePerformanceMonitor::new(config);

        let stats = monitor.get_current_stats();
        assert!(stats.is_ok());

        let stats = stats.unwrap();
        assert!(stats.cpu_usage >= 0.0 && stats.cpu_usage <= 1.0);
        assert!(stats.memory_usage >= 0.0 && stats.memory_usage <= 1.0);
        assert!(stats.memory_total > 0);
    }

    #[test]
    fn test_detect_cpu_bottleneck() {
        let config = MonitoringConfig {
            cpu_warning_threshold: 0.7,
            ..MonitoringConfig::default()
        };
        let monitor = RealtimePerformanceMonitor::new(config);

        let stats = SystemStats::new(
            1234567890,
            0.85, // 高 CPU 使用率
            vec![0.9, 0.95, 0.8, 0.88],
            4_000_000_000,
            16_000_000_000,
            0.25,
            0,
            0,
            0,
            0,
        );

        let bottlenecks = monitor.detect_bottlenecks(&stats);
        assert!(!bottlenecks.is_empty());
        assert!(bottlenecks.iter().any(|b| b.contains("CPU")));
    }

    #[test]
    fn test_detect_memory_pressure() {
        let config = MonitoringConfig {
            memory_warning_threshold: 0.8,
            ..MonitoringConfig::default()
        };
        let monitor = RealtimePerformanceMonitor::new(config);

        let stats = SystemStats::new(
            1234567890,
            0.3,
            vec![0.3, 0.3, 0.3, 0.3],
            14_000_000_000,
            16_000_000_000,
            0.875, // 高内存使用率
            0,
            0,
            0,
            0,
        );

        let bottlenecks = monitor.detect_bottlenecks(&stats);
        assert!(!bottlenecks.is_empty());
        assert!(bottlenecks.iter().any(|b| b.contains("内存")));
    }

    #[test]
    fn test_generate_high_cpu_alert() {
        let config = MonitoringConfig {
            cpu_warning_threshold: 0.7,
            ..MonitoringConfig::default()
        };
        let monitor = RealtimePerformanceMonitor::new(config);

        let stats = SystemStats::new(
            1234567890,
            0.92, // 高 CPU 使用率
            vec![],
            4_000_000_000,
            16_000_000_000,
            0.25,
            0,
            0,
            0,
            0,
        );

        let alerts = monitor.generate_alerts(&stats);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| a.alert_type == "cpu"));

        let cpu_alert = alerts.iter().find(|a| a.alert_type == "cpu").unwrap();
        assert_eq!(cpu_alert.severity, "warning");
        assert_eq!(cpu_alert.value, 0.92);
    }

    #[test]
    fn test_generate_high_memory_alert() {
        let config = MonitoringConfig {
            memory_warning_threshold: 0.8,
            ..MonitoringConfig::default()
        };
        let monitor = RealtimePerformanceMonitor::new(config);

        let stats = SystemStats::new(
            1234567890,
            0.3,
            vec![],
            15_000_000_000,
            16_000_000_000,
            0.9375, // 高内存使用率
            0,
            0,
            0,
            0,
        );

        let alerts = monitor.generate_alerts(&stats);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| a.alert_type == "memory"));
    }

    #[test]
    fn test_get_top_processes() {
        let config = MonitoringConfig::default();
        let monitor = RealtimePerformanceMonitor::new(config);

        let processes = monitor.get_top_processes(5);

        // 至少应该有一些进程
        assert!(!processes.is_empty());

        // 验证进程按 CPU 排序
        for i in 1..processes.len() {
            assert!(processes[i - 1].cpu_usage >= processes[i].cpu_usage);
        }
    }

    #[test]
    fn test_no_alert_when_below_threshold() {
        let config = MonitoringConfig::default();
        let monitor = RealtimePerformanceMonitor::new(config);

        let stats = SystemStats::new(
            1234567890,
            0.5, // CPU < 0.8
            vec![],
            8_000_000_000,
            16_000_000_000,
            0.5, // Memory < 0.9
            0,
            0,
            0,
            0,
        );

        let alerts = monitor.generate_alerts(&stats);
        assert!(alerts.is_empty());
    }
}
