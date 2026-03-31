//! Performance Benchmark Agent 实现
//! 
//! 负责自动运行性能基准测试并生成分析报告。
//! 支持 Rust 和 TypeScript 两种语言的基准测试。
//! 提供性能对比、回归检测、瓶颈分析等功能。

use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;
use regex::Regex;

/// 基准测试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// 项目路径
    pub project_path: String,
    /// 语言（"rust" 或 "typescript"）
    pub language: String,
    /// 基准测试模式（可选）
    pub benchmark_pattern: Option<String>,
    /// 迭代次数
    pub iterations: u32,
    /// 预热迭代次数
    pub warmup_iterations: u32,
    /// 是否与基线对比
    pub compare_with_baseline: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            project_path: ".".to_string(),
            language: "rust".to_string(),
            benchmark_pattern: None,
            iterations: 100,
            warmup_iterations: 10,
            compare_with_baseline: true,
        }
    }
}

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    /// 平均耗时（毫秒）
    pub mean_time_ms: f64,
    /// 中位数耗时（毫秒）
    pub median_time_ms: f64,
    /// 标准差（毫秒）
    pub std_deviation_ms: f64,
    /// 最小耗时（毫秒）
    pub min_time_ms: f64,
    /// 最大耗时（毫秒）
    pub max_time_ms: f64,
    /// 内存使用（字节，可选）
    pub memory_usage_bytes: Option<u64>,
    /// 吞吐量（每秒操作数，可选）
    pub throughput_ops_per_sec: Option<f64>,
}

impl BenchmarkMetrics {
    pub fn new(
        mean_time_ms: f64,
        median_time_ms: f64,
        std_deviation_ms: f64,
        min_time_ms: f64,
        max_time_ms: f64,
        memory_usage_bytes: Option<u64>,
        throughput_ops_per_sec: Option<f64>,
    ) -> Self {
        Self {
            mean_time_ms,
            median_time_ms,
            std_deviation_ms,
            min_time_ms,
            max_time_ms,
            memory_usage_bytes,
            throughput_ops_per_sec,
        }
    }
}

/// 单次基准测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// 基准测试名称
    pub name: String,
    /// 性能指标
    pub metrics: BenchmarkMetrics,
    /// 基线指标（如果有）
    pub baseline_metrics: Option<BenchmarkMetrics>,
    /// 回归百分比（正数表示变慢，负数表示变快）
    pub regression_percentage: Option<f64>,
    /// 是否检测到性能回归
    pub is_regression: bool,
}

impl BenchmarkResult {
    pub fn new(
        name: String,
        metrics: BenchmarkMetrics,
        baseline_metrics: Option<BenchmarkMetrics>,
        regression_percentage: Option<f64>,
        is_regression: bool,
    ) -> Self {
        Self {
            name,
            metrics,
            baseline_metrics,
            regression_percentage,
            is_regression,
        }
    }
}

/// 基准测试报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    /// 基准测试总数
    pub total_benchmarks: u32,
    /// 性能退化的数量
    pub regressed_count: u32,
    /// 性能提升的数量
    pub improved_count: u32,
    /// 稳定的数量
    pub stable_count: u32,
    /// 所有结果
    pub results: Vec<BenchmarkResult>,
    /// 识别的瓶颈
    pub bottlenecks: Vec<String>,
    /// 优化建议
    pub suggestions: Vec<String>,
}

impl BenchmarkReport {
    pub fn new(
        total_benchmarks: u32,
        regressed_count: u32,
        improved_count: u32,
        stable_count: u32,
        results: Vec<BenchmarkResult>,
        bottlenecks: Vec<String>,
        suggestions: Vec<String>,
    ) -> Self {
        Self {
            total_benchmarks,
            regressed_count,
            improved_count,
            stable_count,
            results,
            bottlenecks,
            suggestions,
        }
    }

    /// 获取整体性能变化百分比
    pub fn overall_change(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.results.iter()
            .filter_map(|r| r.regression_percentage)
            .sum();
        
        sum / self.results.len() as f64
    }
}

/// Performance Benchmark Agent 结构体
pub struct PerformanceBenchmarkAgent {
    /// 配置信息
    config: BenchmarkConfig,
}

impl PerformanceBenchmarkAgent {
    /// 创建新的性能基准测试 Agent
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// 运行基准测试
    pub async fn run_benchmarks(&self) -> Result<BenchmarkReport, String> {
        log::info!("开始运行性能基准测试，语言：{}", self.config.language);

        let results = match self.config.language.as_str() {
            "rust" => self.run_rust_benchmarks().await?,
            "typescript" | "ts" => self.run_ts_benchmarks().await?,
            _ => return Err(format!("不支持的语言：{}", self.config.language)),
        };

        // 统计结果
        let total = results.len() as u32;
        let regressed = results.iter().filter(|r| r.is_regression).count() as u32;
        let improved = results.iter()
            .filter(|r| r.regression_percentage.map_or(false, |p| p < -5.0))
            .count() as u32;
        let stable = total - regressed - improved;

        // 识别瓶颈和生成建议
        let bottlenecks = self.identify_bottlenecks(&results);
        let suggestions = self.generate_optimization_suggestions(&results, &bottlenecks);

        log::info!(
            "基准测试完成：{} 个测试，退化：{}, 提升：{}, 稳定：{}",
            total,
            regressed,
            improved,
            stable
        );

        Ok(BenchmarkReport::new(
            total,
            regressed,
            improved,
            stable,
            results,
            bottlenecks,
            suggestions,
        ))
    }

    /// 运行 Rust 基准测试（criterion）
    async fn run_rust_benchmarks(&self) -> Result<Vec<BenchmarkResult>, String> {
        log::info!("运行 Rust 基准测试...");

        // 构建 cargo bench 命令
        let mut cmd = Command::new("cargo");
        cmd.arg("bench")
            .current_dir(&self.config.project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // 添加基准测试模式
        if let Some(pattern) = &self.config.benchmark_pattern {
            cmd.arg("--").arg(pattern);
        }

        // 执行命令
        let child = cmd.spawn()
            .map_err(|e| format!("启动 cargo bench 失败：{}", e))?;

        let output = child.wait_with_output().await
            .map_err(|e| format!("等待 cargo bench 完成失败：{}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // 解析基准测试输出
        let results = self.parse_criterion_output(&stdout, &stderr)?;

        // 如果需要对比基线
        let final_results = if self.config.compare_with_baseline {
            self.load_and_compare_baseline(results).await?
        } else {
            results
        };

        Ok(final_results)
    }

    /// 运行 TypeScript 基准测试（benchmark.js）
    async fn run_ts_benchmarks(&self) -> Result<Vec<BenchmarkResult>, String> {
        log::info!("运行 TypeScript 基准测试...");

        // 构建 npm run bench 命令
        let mut cmd = Command::new("npm");
        cmd.arg("run")
            .arg("bench")
            .current_dir(&self.config.project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // 执行命令
        let child = cmd.spawn()
            .map_err(|e| format!("启动 npm run bench 失败：{}", e))?;

        let output = child.wait_with_output().await
            .map_err(|e| format!("等待 npm run bench 完成失败：{}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // 解析基准测试输出
        let results = self.parse_benchmark_js_output(&stdout, &stderr)?;

        // 如果需要对比基线
        let final_results = if self.config.compare_with_baseline {
            self.load_and_compare_baseline(results).await?
        } else {
            results
        };

        Ok(final_results)
    }

    /// 解析 Criterion 输出
    fn parse_criterion_output(&self, stdout: &str, stderr: &str) -> Result<Vec<BenchmarkResult>, String> {
        let mut results = Vec::new();

        // Criterion 输出格式示例：
        // bench_fibonacci          time:   [12.345 ms 12.456 ms 12.567 ms]
        //                         change: [+5.1234% +6.2345% +7.3456%] (p = 0.00 < 0.05)
        
        let re_bench = Regex::new(r"^(\w+)\s+time:\s+\[(\d+\.\d+) ms (\d+\.\d+) ms (\d+\.\d+) ms\]").unwrap();
        let re_change = Regex::new(r"change:\s+\[([+-]?\d+\.\d+)% ([+-]?\d+\.\d+)% ([+-]?\d+\.\d+)%\]").unwrap();

        for line in stdout.lines() {
            if let Some(caps) = re_bench.captures(line) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown").to_string();
                let min = caps.get(2).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
                let median = caps.get(3).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
                let max = caps.get(4).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
                
                // 尝试读取下一行的变化信息
                let regression_pct = stdout.lines()
                    .skip_while(|l| !l.contains(&name))
                    .nth(1)
                    .and_then(|line| re_change.captures(line))
                    .and_then(|caps| caps.get(2))
                    .and_then(|m| m.as_str().parse::<f64>().ok());

                let metrics = BenchmarkMetrics::new(median, median, (max - min) / 2.0, min, max, None, None);
                let is_regression = regression_pct.map_or(false, |p| p > 5.0);

                results.push(BenchmarkResult::new(name, metrics, None, regression_pct, is_regression));
            }
        }

        // 如果解析失败，检查是否有错误
        if results.is_empty() && !stderr.is_empty() {
            return Err(format!("Criterion 执行失败：{}", stderr));
        }

        Ok(results)
    }

    /// 解析 benchmark.js 输出
    fn parse_benchmark_js_output(&self, stdout: &str, stderr: &str) -> Result<Vec<BenchmarkResult>, String> {
        let mut results = Vec::new();

        // benchmark.js 输出格式示例：
        // fibonacci x 1,234 ops/sec ±5.67% (98 runs sampled)
        
        let re_bench = Regex::new(r"(.+) x ([\d,]+) ops/sec ±([\d.]+)%").unwrap();

        for line in stdout.lines() {
            if let Some(caps) = re_bench.captures(line) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown").to_string();
                let ops = caps.get(2)
                    .map(|m| m.as_str().replace(",", ""))
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                
                // 将 ops/sec 转换为每次操作的耗时（ms）
                let time_ms = if ops > 0.0 { 1000.0 / ops } else { 0.0 };
                
                let metrics = BenchmarkMetrics::new(time_ms, time_ms, 0.0, time_ms, time_ms, None, Some(ops));
                
                results.push(BenchmarkResult::new(name, metrics, None, None, false));
            }
        }

        // 如果解析失败，检查是否有错误
        if results.is_empty() && !stderr.is_empty() {
            return Err(format!("benchmark.js 执行失败：{}", stderr));
        }

        Ok(results)
    }

    /// 加载基线数据并对比
    async fn load_and_compare_baseline(&self, results: Vec<BenchmarkResult>) -> Result<Vec<BenchmarkResult>, String> {
        // 简化实现：返回原始结果
        // 实际应该从文件或其他存储中加载历史基线数据
        
        let mut final_results = Vec::new();
        
        for mut result in results {
            // 模拟加载基线（实际应该从 JSON 文件读取）
            if let Some(baseline) = self.load_baseline_for_test(&result.name).await {
                result.baseline_metrics = Some(baseline.clone());
                result.regression_percentage = Some(self.compare_metrics(&result.metrics, &baseline));
                result.is_regression = self.detect_regression(result.regression_percentage.unwrap());
            }
            final_results.push(result);
        }

        Ok(final_results)
    }

    /// 加载单个测试的基线数据
    async fn load_baseline_for_test(&self, _name: &str) -> Option<BenchmarkMetrics> {
        // 简化实现：返回 None
        // 实际应该从 .harness/benchmarks/baseline.json 读取
        None
    }

    /// 对比两个指标集
    fn compare_metrics(&self, current: &BenchmarkMetrics, baseline: &BenchmarkMetrics) -> f64 {
        if baseline.mean_time_ms == 0.0 {
            return 0.0;
        }
        
        ((current.mean_time_ms - baseline.mean_time_ms) / baseline.mean_time_ms) * 100.0
    }

    /// 检测性能回归
    fn detect_regression(&self, regression_pct: f64) -> bool {
        // 如果性能下降超过 5%，认为是回归
        regression_pct > 5.0
    }

    /// 识别性能瓶颈
    fn identify_bottlenecks(&self, results: &[BenchmarkResult]) -> Vec<String> {
        let mut bottlenecks = Vec::new();

        // 找出最慢的基准测试
        if let Some(slowest) = results.iter().max_by(|a, b| {
            a.metrics.mean_time_ms.partial_cmp(&b.metrics.mean_time_ms).unwrap_or(std::cmp::Ordering::Equal)
        }) {
            if slowest.metrics.mean_time_ms > 100.0 {
                bottlenecks.push(format!("最慢操作：{} ({:.2} ms)", slowest.name, slowest.metrics.mean_time_ms));
            }
        }

        // 找出性能退化的测试
        let regressed: Vec<_> = results.iter()
            .filter(|r| r.is_regression)
            .collect();
        
        if !regressed.is_empty() {
            bottlenecks.push(format!("{} 个测试出现性能退化", regressed.len()));
        }

        bottlenecks
    }

    /// 生成优化建议
    fn generate_optimization_suggestions(&self, results: &[BenchmarkResult], bottlenecks: &[String]) -> Vec<String> {
        let mut suggestions = Vec::new();

        // 基于瓶颈生成建议
        for bottleneck in bottlenecks {
            if bottleneck.contains("最慢操作") {
                suggestions.push("考虑优化慢操作的算法复杂度或使用缓存".to_string());
            }
            if bottleneck.contains("性能退化") {
                suggestions.push("检查最近的代码变更，定位导致性能下降的具体提交".to_string());
            }
        }

        // 通用建议
        if results.iter().any(|r| r.metrics.std_deviation_ms > r.metrics.mean_time_ms * 0.2) {
            suggestions.push("性能波动较大，建议检查系统负载和资源竞争".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("性能表现良好，继续保持".to_string());
        }

        suggestions
    }

    /// 保存基线数据
    pub async fn save_baseline(&self, report: &BenchmarkReport) -> Result<(), String> {
        // 简化实现：记录日志
        // 实际应该保存到 .harness/benchmarks/baseline.json
        log::info!("基线数据已保存（模拟实现）");
        Ok(())
    }

    /// 获取配置信息
    pub fn get_config(&self) -> &BenchmarkConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_config_creation() {
        let config = BenchmarkConfig {
            project_path: "/tmp/test".to_string(),
            language: "rust".to_string(),
            benchmark_pattern: Some("fibonacci".to_string()),
            iterations: 50,
            warmup_iterations: 5,
            compare_with_baseline: true,
        };

        assert_eq!(config.project_path, "/tmp/test");
        assert_eq!(config.language, "rust");
        assert_eq!(config.iterations, 50);
        assert!(config.compare_with_baseline);
    }

    #[test]
    fn test_benchmark_config_default() {
        let config = BenchmarkConfig::default();

        assert_eq!(config.project_path, ".");
        assert_eq!(config.language, "rust");
        assert_eq!(config.iterations, 100);
        assert!(config.compare_with_baseline);
    }

    #[test]
    fn test_benchmark_metrics_creation() {
        let metrics = BenchmarkMetrics::new(
            12.5,  // mean
            12.3,  // median
            0.5,   // std_dev
            11.8,  // min
            13.2,  // max
            None,  // memory
            None,  // throughput
        );

        assert_eq!(metrics.mean_time_ms, 12.5);
        assert_eq!(metrics.median_time_ms, 12.3);
        assert_eq!(metrics.std_deviation_ms, 0.5);
    }

    #[test]
    fn test_benchmark_result_creation() {
        let metrics = BenchmarkMetrics::new(12.5, 12.3, 0.5, 11.8, 13.2, None, None);
        let result = BenchmarkResult::new(
            "fibonacci".to_string(),
            metrics,
            None,
            Some(6.5),
            true,
        );

        assert_eq!(result.name, "fibonacci");
        assert!(result.is_regression);
        assert_eq!(result.regression_percentage, Some(6.5));
    }

    #[test]
    fn test_benchmark_report_creation() {
        let results = vec![
            BenchmarkResult::new("test1".to_string(), BenchmarkMetrics::new(10.0, 10.0, 0.5, 9.5, 10.5, None, None), None, Some(-2.0), false),
            BenchmarkResult::new("test2".to_string(), BenchmarkMetrics::new(15.0, 15.0, 1.0, 14.0, 16.0, None, None), None, Some(8.0), true),
        ];

        let report = BenchmarkReport::new(2, 1, 1, 0, results.clone(), vec![], vec![]);

        assert_eq!(report.total_benchmarks, 2);
        assert_eq!(report.regressed_count, 1);
        assert_eq!(report.improved_count, 1);
    }

    #[test]
    fn test_overall_change_calculation() {
        let results = vec![
            BenchmarkResult::new("test1".to_string(), BenchmarkMetrics::new(10.0, 10.0, 0.0, 10.0, 10.0, None, None), None, Some(-5.0), false),
            BenchmarkResult::new("test2".to_string(), BenchmarkMetrics::new(15.0, 15.0, 0.0, 15.0, 15.0, None, None), None, Some(10.0), true),
        ];

        let report = BenchmarkReport::new(2, 1, 0, 1, results, vec![], vec![]);
        let overall = report.overall_change();

        assert!((overall - 2.5).abs() < 0.01);
    }

    #[test]
    fn test_compare_metrics() {
        let agent = PerformanceBenchmarkAgent::new(BenchmarkConfig::default());
        
        let current = BenchmarkMetrics::new(11.0, 11.0, 0.5, 10.5, 11.5, None, None);
        let baseline = BenchmarkMetrics::new(10.0, 10.0, 0.5, 9.5, 10.5, None, None);

        let change = agent.compare_metrics(&current, &baseline);

        assert!((change - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_detect_regression_true() {
        let agent = PerformanceBenchmarkAgent::new(BenchmarkConfig::default());
        
        assert!(agent.detect_regression(6.0));
        assert!(agent.detect_regression(10.0));
        assert!(!agent.detect_regression(5.0));
        assert!(!agent.detect_regression(0.0));
    }

    #[test]
    fn test_detect_regression_false() {
        let agent = PerformanceBenchmarkAgent::new(BenchmarkConfig::default());
        
        assert!(!agent.detect_regression(3.0));
        assert!(!agent.detect_regression(-5.0));
        assert!(!agent.detect_regression(0.0));
    }

    #[test]
    fn test_identify_bottlenecks_slow() {
        let agent = PerformanceBenchmarkAgent::new(BenchmarkConfig::default());
        
        let results = vec![
            BenchmarkResult::new("slow_op".to_string(), BenchmarkMetrics::new(150.0, 150.0, 5.0, 145.0, 155.0, None, None), None, None, false),
            BenchmarkResult::new("fast_op".to_string(), BenchmarkMetrics::new(10.0, 10.0, 1.0, 9.0, 11.0, None, None), None, None, false),
        ];

        let bottlenecks = agent.identify_bottlenecks(&results);
        
        assert!(!bottlenecks.is_empty());
        assert!(bottlenecks.iter().any(|b| b.contains("最慢操作")));
    }

    #[test]
    fn test_generate_optimization_suggestions() {
        let agent = PerformanceBenchmarkAgent::new(BenchmarkConfig::default());
        
        let bottlenecks = vec!["最慢操作：slow_op (150.00 ms)".to_string()];
        let suggestions = agent.generate_optimization_suggestions(&[], &bottlenecks);

        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("优化")));
    }

    #[test]
    fn test_agent_creation() {
        let config = BenchmarkConfig::default();
        let agent = PerformanceBenchmarkAgent::new(config.clone());

        assert_eq!(agent.get_config().language, "rust");
        assert_eq!(agent.get_config().project_path, ".");
    }
}
