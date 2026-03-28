//! Code Diff Visualizer 实现
//! 
//! 负责解析 Git diff 输出并生成可视化的对比数据

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;

/// 行变更类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LineChangeType {
    /// 未变更（上下文）
    Unchanged,
    /// 新增
    Added,
    /// 删除
    Removed,
}

impl std::fmt::Display for LineChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineChangeType::Unchanged => write!(f, "unchanged"),
            LineChangeType::Added => write!(f, "added"),
            LineChangeType::Removed => write!(f, "removed"),
        }
    }
}

/// 单行差异信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    /// 原文件行号
    pub line_number_old: Option<u32>,
    /// 新文件行号
    pub line_number_new: Option<u32>,
    /// 行内容
    pub content: String,
    /// 变更类型
    pub change_type: LineChangeType,
}

/// 差异块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    /// 块头信息
    pub header: String,
    /// 差异行列表
    pub lines: Vec<DiffLine>,
    /// 原文件起始行号
    pub old_start: u32,
    /// 原文件行数
    pub old_count: u32,
    /// 新文件起始行号
    pub new_start: u32,
    /// 新文件行数
    pub new_count: u32,
}

/// 差异统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffStats {
    /// 总行数
    pub total_lines: u32,
    /// 新增行数
    pub additions: u32,
    /// 删除行数
    pub deletions: u32,
    /// 未变更行数
    pub unchanged: u32,
}

/// 文件差异信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    /// 文件路径
    pub file_path: String,
    /// 原文件路径（重命名时）
    pub old_path: Option<String>,
    /// 新文件路径（重命名时）
    pub new_path: Option<String>,
    /// 差异块列表
    pub hunks: Vec<DiffHunk>,
    /// 统计信息
    pub stats: DiffStats,
}

/// Code Diff Visualizer 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDiffVisualizerConfig {
    /// 项目路径
    pub project_path: String,
    /// 是否包含上下文行
    pub include_context: bool,
    /// 上下文行数（默认 3）
    pub context_lines: u32,
}

/// Code Diff Visualizer
#[derive(Debug, Clone)]
pub struct CodeDiffVisualizer {
    config: CodeDiffVisualizerConfig,
    workspace_root: PathBuf,
}

impl CodeDiffVisualizer {
    /// 创建新的可视化器
    pub fn new(project_path: PathBuf) -> Result<Self, String> {
        if !project_path.exists() {
            return Err(format!("Project path does not exist: {:?}", project_path));
        }

        Ok(Self {
            config: CodeDiffVisualizerConfig {
                project_path: project_path.to_string_lossy().to_string(),
                include_context: true,
                context_lines: 3,
            },
            workspace_root: project_path,
        })
    }

    /// 创建带配置的可视化器
    pub fn with_config(config: CodeDiffVisualizerConfig) -> Result<Self, String> {
        let workspace_root = PathBuf::from(&config.project_path);
        if !workspace_root.exists() {
            return Err(format!("Project path does not exist: {}", config.project_path));
        }

        Ok(Self {
            config,
            workspace_root,
        })
    }

    /// 获取文件的可视化差异
    pub async fn get_file_diff_visual(&self, file_path: &str) -> Result<FileDiff, String> {
        log::info!("Getting visual diff for file: {}", file_path);

        // 运行 git diff --unified 获取详细的差异信息
        let output = Command::new("git")
            .args(["diff", "--unified=0", "--", file_path])
            .current_dir(&self.workspace_root)
            .output()
            .await
            .map_err(|e| format!("Failed to run git diff: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git diff failed: {}", stderr));
        }

        let diff_output = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 解析 unified diff
        let mut file_diff = self.parse_unified_diff(&diff_output, file_path)?;
        
        // 计算统计信息
        file_diff.stats = self.calculate_stats(&file_diff.hunks);

        Ok(file_diff)
    }

    /// 解析 unified diff 输出
    pub fn parse_unified_diff(&self, diff_output: &str, file_path: &str) -> Result<FileDiff, String> {
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;
        let mut old_line_num = 0u32;
        let mut new_line_num = 0u32;

        for line in diff_output.lines() {
            // 跳过空行和文件头
            if line.is_empty() || line.starts_with("diff ") || line.starts_with("index ") {
                continue;
            }

            // 解析文件路径（如果有重命名）
            if line.starts_with("--- ") {
                // 旧文件路径（在 diff 中通常被忽略）
                continue;
            }

            if line.starts_with("+++ ") {
                // 新文件路径（在 diff 中通常被忽略）
                continue;
            }

            // 解析 hunk header: @@ -old_start,old_count +new_start,new_count @@
            if line.starts_with("@@") {
                // 保存之前的 hunk
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }

                // 解析新的 hunk header
                let (old_start, old_count, new_start, new_count) = self.parse_hunk_header(line)?;
                
                // 重置行号计数器
                old_line_num = old_start;
                new_line_num = new_start;

                current_hunk = Some(DiffHunk {
                    header: line.to_string(),
                    lines: Vec::new(),
                    old_start,
                    old_count,
                    new_start,
                    new_count,
                });

                continue;
            }

            // 解析 hunk 内容
            if let Some(ref mut hunk) = current_hunk {
                if line.starts_with('+') {
                    // 新增行
                    hunk.lines.push(DiffLine {
                        line_number_old: None,
                        line_number_new: Some(new_line_num),
                        content: line[1..].to_string(), // 去掉 '+' 前缀
                        change_type: LineChangeType::Added,
                    });
                    new_line_num += 1;
                } else if line.starts_with('-') {
                    // 删除行
                    hunk.lines.push(DiffLine {
                        line_number_old: Some(old_line_num),
                        line_number_new: None,
                        content: line[1..].to_string(), // 去掉 '-' 前缀
                        change_type: LineChangeType::Removed,
                    });
                    old_line_num += 1;
                } else if line.starts_with(' ') {
                    // 上下文行（未变更）
                    hunk.lines.push(DiffLine {
                        line_number_old: Some(old_line_num),
                        line_number_new: Some(new_line_num),
                        content: line[1..].to_string(), // 去掉 ' ' 前缀
                        change_type: LineChangeType::Unchanged,
                    });
                    old_line_num += 1;
                    new_line_num += 1;
                }
                // 忽略其他行（如 "\ No newline at end of file"）
            }
        }

        // 添加最后一个 hunk
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        // 如果没有解析到任何 hunks，返回空的文件差异
        let file_diff = FileDiff {
            file_path: file_path.to_string(),
            old_path: None,
            new_path: None,
            hunks,
            stats: DiffStats {
                total_lines: 0,
                additions: 0,
                deletions: 0,
                unchanged: 0,
            },
        };

        Ok(file_diff)
    }

    /// 解析 hunk header
    fn parse_hunk_header(&self, header: &str) -> Result<(u32, u32, u32, u32), String> {
        // 格式：@@ -old_start,old_count +new_start,new_count @@
        // 例如：@@ -10,7 +10,8 @@
        
        // 去掉首尾的 @@
        let trimmed = header.trim();
        if !trimmed.starts_with("@@") || !trimmed.ends_with("@@") {
            return Err(format!("Invalid hunk header: {}", header));
        }
        
        // 提取中间部分：-old_start,old_count +new_start,new_count
        let content = &trimmed[2..(trimmed.len()-2)].trim();
        
        let mut old_start = 1u32;
        let mut old_count = 1u32;
        let mut new_start = 1u32;
        let mut new_count = 1u32;
        
        // 分割为两部分：-old_start,old_count 和 +new_start,new_count
        let parts: Vec<&str> = content.split_whitespace().collect();
        
        for part in parts {
            if part.starts_with('-') {
                // 解析旧文件范围：-10,7
                let range = &part[1..];
                let nums: Vec<&str> = range.split(',').collect();
                if !nums.is_empty() {
                    old_start = nums[0].parse().unwrap_or(1);
                    if nums.len() > 1 {
                        old_count = nums[1].parse().unwrap_or(1);
                    }
                }
            } else if part.starts_with('+') {
                // 解析新文件范围：+10,8
                let range = &part[1..];
                let nums: Vec<&str> = range.split(',').collect();
                if !nums.is_empty() {
                    new_start = nums[0].parse().unwrap_or(1);
                    if nums.len() > 1 {
                        new_count = nums[1].parse().unwrap_or(1);
                    }
                }
            }
        }
        
        Ok((old_start, old_count, new_start, new_count))
    }

    /// 计算差异统计
    fn calculate_stats(&self, hunks: &[DiffHunk]) -> DiffStats {
        let mut stats = DiffStats {
            total_lines: 0,
            additions: 0,
            deletions: 0,
            unchanged: 0,
        };

        for hunk in hunks {
            for line in &hunk.lines {
                stats.total_lines += 1;
                match line.change_type {
                    LineChangeType::Added => stats.additions += 1,
                    LineChangeType::Removed => stats.deletions += 1,
                    LineChangeType::Unchanged => stats.unchanged += 1,
                }
            }
        }

        stats
    }

    /// 生成并排对比视图数据
    pub fn generate_side_by_side(&self, file_diff: &FileDiff) -> SideBySideView {
        let mut left_lines = Vec::new();
        let mut right_lines = Vec::new();

        for hunk in &file_diff.hunks {
            for line in &hunk.lines {
                match line.change_type {
                    LineChangeType::Unchanged => {
                        // 两侧都显示
                        left_lines.push(SideBySideLine {
                            line_number: line.line_number_old,
                            content: line.content.clone(),
                            change_type: line.change_type.clone(),
                        });
                        right_lines.push(SideBySideLine {
                            line_number: line.line_number_new,
                            content: line.content.clone(),
                            change_type: line.change_type.clone(),
                        });
                    }
                    LineChangeType::Added => {
                        // 左侧空白，右侧显示新增
                        left_lines.push(SideBySideLine {
                            line_number: None,
                            content: String::new(),
                            change_type: LineChangeType::Unchanged,
                        });
                        right_lines.push(SideBySideLine {
                            line_number: line.line_number_new,
                            content: line.content.clone(),
                            change_type: line.change_type.clone(),
                        });
                    }
                    LineChangeType::Removed => {
                        // 左侧显示删除，右侧空白
                        left_lines.push(SideBySideLine {
                            line_number: line.line_number_old,
                            content: line.content.clone(),
                            change_type: line.change_type.clone(),
                        });
                        right_lines.push(SideBySideLine {
                            line_number: None,
                            content: String::new(),
                            change_type: LineChangeType::Unchanged,
                        });
                    }
                }
            }
        }

        SideBySideView {
            left_lines,
            right_lines,
        }
    }
}

/// 并排对比视图行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideBySideLine {
    pub line_number: Option<u32>,
    pub content: String,
    pub change_type: LineChangeType,
}

/// 并排对比视图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideBySideView {
    pub left_lines: Vec<SideBySideLine>,
    pub right_lines: Vec<SideBySideLine>,
}

/// 差异摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSummary {
    pub file_path: String,
    pub stats: DiffStats,
    pub hunk_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizer_creation() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir);
        assert!(visualizer.is_ok());
    }

    #[test]
    fn test_with_config() {
        let temp_dir = std::env::temp_dir();
        let config = CodeDiffVisualizerConfig {
            project_path: temp_dir.to_string_lossy().to_string(),
            include_context: true,
            context_lines: 5,
        };

        let visualizer = CodeDiffVisualizer::with_config(config);
        assert!(visualizer.is_ok());
    }

    #[test]
    fn test_parse_hunk_header() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let header = "@@ -10,7 +10,8 @@";
        let (old_start, old_count, new_start, new_count) = visualizer.parse_hunk_header(header).unwrap();

        // 实际解析结果
        assert_eq!(old_start, 10);
        assert_eq!(old_count, 7);
        assert_eq!(new_start, 10);
        assert_eq!(new_count, 8);
    }

    #[test]
    fn test_parse_hunk_header_single_line() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let header = "@@ -5 +5 @@";
        let (old_start, old_count, new_start, new_count) = visualizer.parse_hunk_header(header).unwrap();

        // 单行格式默认为 count=1
        assert_eq!(old_start, 5);
        assert_eq!(old_count, 1);
        assert_eq!(new_start, 5);
        assert_eq!(new_count, 1);
    }

    #[test]
    fn test_parse_unified_diff_simple() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let diff = r#"--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,4 @@
 line1
-line2
+line2 modified
+new line3
 line3
"#;

        let file_diff = visualizer.parse_unified_diff(diff, "file.txt").unwrap();

        assert_eq!(file_diff.file_path, "file.txt");
        assert_eq!(file_diff.hunks.len(), 1);
        // 实际解析出 5 行：line1(上下文), line2(删除), line2 modified(新增), new line3(新增), line3(上下文)
        assert_eq!(file_diff.hunks[0].lines.len(), 5);
    }

    #[test]
    fn test_diff_line_parsing() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let diff = r#"@@ -1,2 +1,3 @@
 context
-removed
+added
"#;

        let file_diff = visualizer.parse_unified_diff(diff, "test.txt").unwrap();
        let lines = &file_diff.hunks[0].lines;

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].change_type, LineChangeType::Unchanged);
        assert_eq!(lines[1].change_type, LineChangeType::Removed);
        assert_eq!(lines[2].change_type, LineChangeType::Added);
    }

    #[test]
    fn test_diff_stats_calculation() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let diff = r#"@@ -1,3 +1,4 @@
 line1
-line2
+line2 modified
+new line3
 line3
"#;

        let file_diff = visualizer.parse_unified_diff(diff, "file.txt").unwrap();
        let stats = visualizer.calculate_stats(&file_diff.hunks);

        // 实际：5 行（2 个上下文，1 个删除，2 个新增）
        assert_eq!(stats.total_lines, 5);
        assert_eq!(stats.additions, 2);
        assert_eq!(stats.deletions, 1);
        assert_eq!(stats.unchanged, 2);
    }

    #[test]
    fn test_empty_diff() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let file_diff = visualizer.parse_unified_diff("", "file.txt").unwrap();

        assert_eq!(file_diff.file_path, "file.txt");
        assert_eq!(file_diff.hunks.len(), 0);
        assert_eq!(file_diff.stats.total_lines, 0);
    }

    #[test]
    fn test_multiple_hunks() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let diff = r#"@@ -1,2 +1,3 @@
 line1
+added1
 line2
@@ -10,2 +11,3 @@
 line10
+added2
 line11
"#;

        let file_diff = visualizer.parse_unified_diff(diff, "file.txt").unwrap();

        assert_eq!(file_diff.hunks.len(), 2);
    }

    #[test]
    fn test_context_lines_handling() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let diff = r#"@@ -1,5 +1,6 @@
 context1
 context2
-change1
+change1 modified
 context3
 context4
 context5
"#;

        let file_diff = visualizer.parse_unified_diff(diff, "file.txt").unwrap();
        let lines = &file_diff.hunks[0].lines;

        // 实际：7 行（5 个上下文，1 个删除，1 个新增）
        assert_eq!(lines.len(), 7);
        assert!(lines.iter().filter(|l| l.change_type == LineChangeType::Unchanged).count() == 5);
        assert!(lines.iter().filter(|l| l.change_type == LineChangeType::Removed).count() == 1);
        assert!(lines.iter().filter(|l| l.change_type == LineChangeType::Added).count() == 1);
    }

    #[test]
    fn test_line_number_mapping() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let diff = r#"@@ -5,3 +5,4 @@
 line5
-line6
+line6 modified
+new line
 line7
"#;

        let file_diff = visualizer.parse_unified_diff(diff, "file.txt").unwrap();
        let lines = &file_diff.hunks[0].lines;

        // 实际解析结果：5 行
        // line5: old=5, new=5 (上下文)
        // line6: old=6, new=None (删除)
        // line6 modified: old=None, new=6 (新增)
        // new line: old=None, new=7 (新增)
        // line7: old=7, new=8 (上下文)
        
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0].line_number_old, Some(5));
        assert_eq!(lines[0].line_number_new, Some(5));
        assert_eq!(lines[1].line_number_old, Some(6));
        assert_eq!(lines[1].line_number_new, None);
        assert_eq!(lines[2].line_number_old, None);
        assert_eq!(lines[2].line_number_new, Some(6));
        assert_eq!(lines[3].line_number_old, None);
        assert_eq!(lines[3].line_number_new, Some(7));
        assert_eq!(lines[4].line_number_old, Some(7));
        assert_eq!(lines[4].line_number_new, Some(8));
    }

    #[test]
    fn test_generate_side_by_side() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let file_diff = FileDiff {
            file_path: "test.txt".to_string(),
            old_path: None,
            new_path: None,
            hunks: vec![
                DiffHunk {
                    header: "@@ -1,2 +1,3 @@".to_string(),
                    lines: vec![
                        DiffLine {
                            line_number_old: Some(1),
                            line_number_new: Some(1),
                            content: "line1".to_string(),
                            change_type: LineChangeType::Unchanged,
                        },
                        DiffLine {
                            line_number_old: Some(2),
                            line_number_new: None,
                            content: "line2".to_string(),
                            change_type: LineChangeType::Removed,
                        },
                        DiffLine {
                            line_number_old: None,
                            line_number_new: Some(2),
                            content: "line2 new".to_string(),
                            change_type: LineChangeType::Added,
                        },
                    ],
                    old_start: 1,
                    old_count: 2,
                    new_start: 1,
                    new_count: 3,
                },
            ],
            stats: DiffStats {
                total_lines: 3,
                additions: 1,
                deletions: 1,
                unchanged: 1,
            },
        };

        let view = visualizer.generate_side_by_side(&file_diff);

        assert_eq!(view.left_lines.len(), 3);
        assert_eq!(view.right_lines.len(), 3);
        
        // 检查第一行（未变更）
        assert_eq!(view.left_lines[0].content, "line1");
        assert_eq!(view.right_lines[0].content, "line1");
        
        // 检查第二行（左侧删除，右侧空白）
        assert_eq!(view.left_lines[1].content, "line2");
        assert_eq!(view.left_lines[1].change_type, LineChangeType::Removed);
        assert_eq!(view.right_lines[1].content, "");
        
        // 检查第三行（左侧空白，右侧新增）
        assert_eq!(view.left_lines[2].content, "");
        assert_eq!(view.right_lines[2].content, "line2 new");
        assert_eq!(view.right_lines[2].change_type, LineChangeType::Added);
    }

    #[test]
    fn test_whitespace_handling() {
        let temp_dir = std::env::temp_dir();
        let visualizer = CodeDiffVisualizer::new(temp_dir).unwrap();

        let diff = r#"@@ -1 +1 @@
-old content
+new content  
"#;

        let file_diff = visualizer.parse_unified_diff(diff, "file.txt").unwrap();
        let lines = &file_diff.hunks[0].lines;

        assert_eq!(lines[0].content, "old content");
        assert_eq!(lines[1].content, "new content  "); // 保留尾部空格
    }
}
