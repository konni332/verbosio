#[cfg(feature = "color")]
pub fn format_level(level: &str) -> String {
    use colored::*;
    match level {
        "INFO" => "[INFO]".blue().bold().to_string(),
        "WARN" => "[WARN]".yellow().bold().to_string(),
        "ERROR" => "[ERROR]".red().bold().to_string(),
        _ => level.to_string(),
    }
}

#[cfg(not(feature = "color"))]
pub fn format_level(level: &str) -> String {
    format!("[{}]", level)
}
