//! Utility functions

use std::path::PathBuf;

/// Get application data directory
pub fn app_data_dir() -> Result<PathBuf, std::io::Error> {
    let app_dirs = directories::ProjectDirs::from("com", "opc-harness", "OPC-HARNESS")
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine app data directory"
        ))?;
    
    let data_dir = app_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;
    
    Ok(data_dir.to_path_buf())
}

/// Format file size
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if size == 0 {
        return "0 B".to_string();
    }
    
    let exp = (size as f64).log(1024.0).min(UNITS.len() as f64 - 1.0) as usize;
    let size = size as f64 / 1024f64.powi(exp as i32);
    
    format!("{:.1} {}", size, UNITS[exp])
}

/// Generate a unique ID
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Truncate string with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
