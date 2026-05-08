// 用户故事解析模块
// 负责解析AI响应为用户故事数据结构

use crate::commands::quality::types::UserStory;

/// 解析 AI 响应为用户故事列表
pub fn parse_ai_response_to_user_stories(response: &str) -> Result<Vec<UserStory>, String> {
    log::info!("Parsing AI response to user stories...");

    // 尝试解析 Markdown 表格格式
    match parse_markdown_table_to_user_stories(response) {
        Ok(stories) => {
            if !stories.is_empty() {
                log::info!(
                    "Successfully parsed {} user stories from Markdown table",
                    stories.len()
                );
                return Ok(stories);
            }
        }
        Err(e) => {
            log::warn!("Markdown table parsing failed: {}", e);
        }
    }

    // 如果表格解析失败,尝试 JSON 格式(向后兼容)
    log::info!("Attempting JSON format as fallback...");
    use serde_json::Value;

    match extract_json_array(response) {
        Ok(json_str) => {
            let parsed: Value =
                serde_json::from_str(&json_str).map_err(|e| format!("JSON 解析失败：{}", e))?;

            let stories_array = parsed
                .as_array()
                .ok_or_else(|| "AI 响应不是有效的 JSON 数组".to_string())?;

            let mut user_stories = Vec::new();
            for (index, story_value) in stories_array.iter().enumerate() {
                match parse_single_user_story(story_value, index) {
                    Ok(story) => user_stories.push(story),
                    Err(e) => {
                        log::warn!("跳过无效的用户故事 #{}: {}", index + 1, e);
                    }
                }
            }

            if !user_stories.is_empty() {
                log::info!(
                    "Successfully parsed {} user stories from JSON",
                    user_stories.len()
                );
                return Ok(user_stories);
            }
        }
        Err(e) => {
            log::warn!("JSON extraction failed: {}", e);
        }
    }

    // 所有策略都失败
    Err(format!(
        "无法解析 AI 响应。期望 Markdown 表格或 JSON 格式。\n\n\
         AI 响应预览（前300字符）：\n{}",
        if response.len() > 300 {
            &response[..300]
        } else {
            response
        }
    ))
}

/// 解析 Markdown 表格格式为用户故事列表
pub fn parse_markdown_table_to_user_stories(response: &str) -> Result<Vec<UserStory>, String> {
    // 查找 Markdown 表格
    let lines: Vec<&str> = response.lines().collect();

    // 寻找表格分隔行(包含 |---| 或 |-|-| 的行)
    let mut table_start = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains('|') && line.contains("---") {
            table_start = Some(i);
            break;
        }
    }

    let table_start = table_start.ok_or("未找到 Markdown 表格")?;

    // 提取表头和数据行
    if table_start == 0 {
        return Err("表格缺少表头".to_string());
    }

    let header_line = lines[table_start - 1];
    let data_lines: Vec<&str> = lines[table_start + 1..]
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with('|') && !trimmed.contains("---") && trimmed.len() > 2
        })
        .cloned()
        .collect();

    if data_lines.is_empty() {
        return Err("表格没有数据行".to_string());
    }

    log::info!("Found Markdown table with {} data rows", data_lines.len());

    // 解析表头
    let headers = parse_table_row(header_line);
    log::debug!("Table headers: {:?}", headers);

    // 解析每一行数据
    let mut user_stories = Vec::new();
    for (index, data_line) in data_lines.iter().enumerate() {
        let cells = parse_table_row(data_line);

        if cells.len() != headers.len() {
            log::warn!(
                "跳过行 #{}: 列数不匹配 (期望 {}, 实际 {})",
                index + 1,
                headers.len(),
                cells.len()
            );
            continue;
        }

        // 将表格行转换为 UserStory
        match convert_table_row_to_story(&headers, &cells, index) {
            Ok(story) => {
                log::debug!("Successfully parsed story #{}: {}", index + 1, story.title);
                user_stories.push(story);
            }
            Err(e) => {
                log::warn!("转换行 #{} 失败: {}", index + 1, e);
            }
        }
    }

    if user_stories.is_empty() {
        return Err("从表格中未能解析出任何有效的用户故事".to_string());
    }

    Ok(user_stories)
}

/// 解析表格行为单元格数组
fn parse_table_row(line: &str) -> Vec<String> {
    line.split('|')
        .skip(1) // 跳过第一个空单元格
        .map(|cell| cell.trim().to_string())
        .filter(|cell| !cell.is_empty())
        .collect()
}

/// 将表格行转换为 UserStory
fn convert_table_row_to_story(
    headers: &[String],
    cells: &[String],
    index: usize,
) -> Result<UserStory, String> {
    // 创建字段映射(不区分大小写)
    let field_map: std::collections::HashMap<String, String> = headers
        .iter()
        .zip(cells.iter())
        .map(|(h, c)| (h.to_lowercase().trim().to_string(), c.clone()))
        .collect();

    // 提取字段值,提供默认值
    let story_number = field_map
        .get("序号")
        .or_else(|| field_map.get("story_number"))
        .or_else(|| field_map.get("编号"))
        .cloned()
        .unwrap_or_else(|| format!("US-{:03}", index + 1));

    let title = field_map
        .get("标题")
        .or_else(|| field_map.get("title"))
        .cloned()
        .unwrap_or_else(|| format!("用户故事 #{}", index + 1));

    let role = field_map
        .get("角色")
        .or_else(|| field_map.get("role"))
        .cloned()
        .unwrap_or_else(|| "用户".to_string());

    let feature = field_map
        .get("功能")
        .or_else(|| field_map.get("feature"))
        .cloned()
        .unwrap_or_default();

    let benefit = field_map
        .get("价值")
        .or_else(|| field_map.get("benefit"))
        .cloned()
        .unwrap_or_default();

    let priority = field_map
        .get("优先级")
        .or_else(|| field_map.get("priority"))
        .cloned()
        .unwrap_or_else(|| "P1".to_string());

    let story_points_str = field_map
        .get("故事点")
        .or_else(|| field_map.get("story_points"))
        .cloned()
        .unwrap_or_else(|| "5".to_string());

    let story_points = story_points_str.parse::<u32>().unwrap_or(5);

    let acceptance_criteria_str = field_map
        .get("验收标准")
        .or_else(|| field_map.get("acceptance_criteria"))
        .cloned()
        .unwrap_or_default();

    // 分号分隔的验收标准
    let acceptance_criteria: Vec<String> = acceptance_criteria_str
        .split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let feature_module = field_map
        .get("模块")
        .or_else(|| field_map.get("feature_module"))
        .or_else(|| field_map.get("module"))
        .cloned()
        .unwrap_or_else(|| "通用".to_string());

    let labels_str = field_map
        .get("标签")
        .or_else(|| field_map.get("labels"))
        .cloned()
        .unwrap_or_default();

    // 逗号分隔的标签
    let labels: Vec<String> = labels_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let dependencies_str = field_map
        .get("依赖")
        .or_else(|| field_map.get("dependencies"))
        .cloned()
        .unwrap_or_else(|| "无".to_string());

    // 解析依赖关系
    let dependencies = if dependencies_str == "无" || dependencies_str.is_empty() {
        None
    } else {
        let deps: Vec<String> = dependencies_str
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty() && s != "无")
            .collect();
        if deps.is_empty() {
            None
        } else {
            Some(deps)
        }
    };

    // 构建描述
    let description = format!("作为{},我想要{},以便{}", role, feature, benefit);

    Ok(UserStory {
        id: story_number.to_lowercase(),
        story_number,
        title,
        role,
        feature,
        benefit,
        description,
        acceptance_criteria,
        priority: validate_priority(&priority),
        status: "draft".to_string(),
        story_points: Some(story_points),
        dependencies,
        feature_module: Some(feature_module),
        sprint_id: None, // 解析时默认为None，后续由用户分配
        labels,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    })
}

/// 验证并规范化优先级
fn validate_priority(priority: &str) -> String {
    match priority.to_uppercase().as_str() {
        "P0" => "P0".to_string(),
        "P1" => "P1".to_string(),
        "P2" => "P2".to_string(),
        "P3" => "P3".to_string(),
        _ => "P1".to_string(), // 默认 P1
    }
}

/// 检测异常的AI响应模式
pub fn detect_abnormal_response(content: &str) -> Option<String> {
    // 检查1: 空响应
    if content.is_empty() {
        return Some("响应为空".to_string());
    }

    // 检查2: 统计唯一字符数
    let unique_chars: std::collections::HashSet<char> = content.chars().collect();

    // 如果唯一字符极少（<=3个），可能是异常响应
    if unique_chars.len() <= 3 {
        let chars_vec: Vec<char> = unique_chars.iter().cloned().collect();

        // 全下划线
        if chars_vec.contains(&'_') && chars_vec.len() == 1 {
            return Some("响应只包含下划线字符，AI可能遇到错误或限制".to_string());
        }

        // 全横线
        if chars_vec.contains(&'-') && chars_vec.len() == 1 {
            return Some("响应只包含横线字符，AI可能遇到错误或限制".to_string());
        }

        // 全空格
        if chars_vec.contains(&' ') && chars_vec.len() == 1 {
            return Some("响应只包含空格".to_string());
        }

        // 重复的Base64-like模式 (如 7A7A7A...)
        if is_repetitive_pattern(content, 2) {
            return Some("响应包含重复的编码模式，可能是Base64数据损坏或API错误".to_string());
        }
    }

    // 检查3: 检测Base64编码特征（大量字母数字混合，无正常文本结构）
    if looks_like_corrupted_base64(content) {
        return Some("响应看起来像损坏的Base64编码数据，API可能返回了二进制内容".to_string());
    }

    // 检查4: 检测HTML错误页面
    if content.starts_with("<!DOCTYPE") || content.starts_with("<html") {
        return Some("响应是HTML页面，可能是API认证失败或服务不可用".to_string());
    }

    // 检查5: 检测JSON错误信息
    if content.starts_with("{\"error\"") || content.starts_with("{ \"error\"") {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            if let Some(error_obj) = json.get("error") {
                if let Some(error_msg) = error_obj.get("message").and_then(|m| m.as_str()) {
                    return Some(format!("API返回错误：{}", error_msg));
                }
            }
        }
    }

    None
}

/// 检测是否为重复模式（检查前N个字符是否在整个字符串中重复）
fn is_repetitive_pattern(content: &str, pattern_len: usize) -> bool {
    if content.len() < pattern_len * 10 {
        return false; // 内容太短，不判断
    }

    let pattern = &content[..pattern_len.min(content.len())];
    let mut repeat_count = 0;
    let check_len = (content.len() / pattern_len).min(20); // 最多检查20次重复

    for i in 0..check_len {
        let start = i * pattern_len;
        let end = (start + pattern_len).min(content.len());
        if start < content.len() && &content[start..end] == pattern {
            repeat_count += 1;
        }
    }

    // 如果80%以上的片段都匹配，认为是重复模式
    repeat_count as f64 / check_len as f64 > 0.8
}

/// 检测是否像损坏的Base64编码
fn looks_like_corrupted_base64(content: &str) -> bool {
    // Base64特征：大量大写字母、小写字母、数字，很少有空格或标点
    let mut alpha_count = 0;
    let mut digit_count = 0;
    let mut other_count = 0;

    for c in content.chars().take(500) {
        // 只检查前500字符
        if c.is_ascii_alphabetic() {
            alpha_count += 1;
        } else if c.is_ascii_digit() {
            digit_count += 1;
        } else {
            other_count += 1;
        }
    }

    let total = alpha_count + digit_count + other_count;
    if total == 0 {
        return false;
    }

    let alpha_digit_ratio = (alpha_count + digit_count) as f64 / total as f64;

    // 如果90%以上是字母数字，且长度较长，可能是Base64
    alpha_digit_ratio > 0.9 && content.len() > 100
}

// ==================== JSON 格式解析(向后兼容) ====================

/// 从 AI 响应中提取 JSON 数组
fn extract_json_array(response: &str) -> Result<String, String> {
    let trimmed = response.trim();

    // 策略1: 尝试直接解析为 JSON
    if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
        return Ok(trimmed.to_string());
    }

    // 策略2: 查找并提取 JSON 数组（支持嵌套）
    if let Some(json_str) = find_json_array_smart(trimmed) {
        return Ok(json_str);
    }

    // 策略3: 查找代码块中的 JSON
    if let Some(json_str) = extract_from_code_block(trimmed) {
        return Ok(json_str);
    }

    Err("无法从 AI 响应中提取有效的 JSON 数组".to_string())
}

/// 智能查找 JSON 数组（支持嵌套括号匹配）
fn find_json_array_smart(response: &str) -> Option<String> {
    let chars: Vec<char> = response.chars().collect();
    let len = chars.len();

    for i in 0..len {
        if chars[i] == '[' {
            let mut depth = 0;
            let mut in_string = false;
            let mut escape_next = false;

            for j in i..len {
                let c = chars[j];

                if escape_next {
                    escape_next = false;
                    continue;
                }

                if c == '\\' && in_string {
                    escape_next = true;
                    continue;
                }

                if c == '"' {
                    in_string = !in_string;
                    continue;
                }

                if !in_string {
                    if c == '[' {
                        depth += 1;
                    } else if c == ']' {
                        depth -= 1;
                        if depth == 0 {
                            let json_str: String = chars[i..=j].iter().collect();
                            if serde_json::from_str::<serde_json::Value>(&json_str).is_ok() {
                                return Some(json_str);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// 从 Markdown 代码块中提取 JSON
fn extract_from_code_block(response: &str) -> Option<String> {
    let patterns = vec![("```json", "```"), ("```", "```")];

    for (start_marker, end_marker) in patterns {
        if let Some(start_pos) = response.find(start_marker) {
            let content_start = start_pos + start_marker.len();
            if let Some(end_pos) = response[content_start..].find(end_marker) {
                let json_str = response[content_start..content_start + end_pos].trim();
                if serde_json::from_str::<serde_json::Value>(json_str).is_ok() {
                    return Some(json_str.to_string());
                }
            }
        }
    }

    None
}

/// 解析单个用户故事(JSON格式)
fn parse_single_user_story(value: &serde_json::Value, index: usize) -> Result<UserStory, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| format!("故事 #{} 不是有效的 JSON 对象", index + 1))?;

    // 提取必填字段
    let title = obj
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("故事 #{} 缺少 title 字段", index + 1))?
        .to_string();

    let role = obj
        .get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("用户")
        .to_string();

    let feature = obj
        .get("feature")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let benefit = obj
        .get("benefit")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let description = obj
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // 提取验收标准
    let acceptance_criteria = obj
        .get("acceptance_criteria")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    // 提取优先级
    let priority = obj.get("priority").and_then(|v| v.as_str()).unwrap_or("P1");

    let priority = validate_priority(priority);

    // 提取故事点
    let story_points = obj
        .get("story_points")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

    // 提取依赖
    let dependencies = obj
        .get("dependencies")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        });

    // 提取模块
    let feature_module = obj
        .get("feature_module")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // 提取标签
    let labels = obj
        .get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    // 提取故事编号和ID
    let story_number = obj
        .get("story_number")
        .and_then(|v| v.as_str())
        .unwrap_or("US-001")
        .to_string();

    let id = obj
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("us-001")
        .to_string();

    Ok(UserStory {
        id,
        story_number,
        title,
        role,
        feature,
        benefit,
        description,
        acceptance_criteria,
        priority,
        status: "draft".to_string(),
        story_points,
        dependencies,
        feature_module,
        sprint_id: None, // 解析时默认为None，后续由用户分配
        labels,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_priority() {
        assert_eq!(validate_priority("P0"), "P0");
        assert_eq!(validate_priority("p1"), "P1");
        assert_eq!(validate_priority("P2"), "P2");
        assert_eq!(validate_priority("invalid"), "P1");
    }

    #[test]
    fn test_detect_abnormal_empty_response() {
        assert!(detect_abnormal_response("").is_some());
    }

    #[test]
    fn test_detect_abnormal_underscores() {
        let response = "____________________";
        assert!(detect_abnormal_response(response).is_some());
    }

    #[test]
    fn test_parse_table_row() {
        let row = "| 1 | Test Title | User |";
        let cells = parse_table_row(row);
        assert_eq!(cells.len(), 3);
        assert_eq!(cells[0], "1");
        assert_eq!(cells[1], "Test Title");
        assert_eq!(cells[2], "User");
    }
}
