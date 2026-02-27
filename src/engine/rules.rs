use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortingRule {
    pub extension: String,
    pub target_path: String,
    pub category_name: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl SortingRule {
    pub fn new(extension: &str, target_path: &str, category_name: &str) -> Self {
        Self {
            extension: extension.to_string(),
            target_path: target_path.to_string(),
            category_name: category_name.to_string(),
            enabled: true,
        }
    }
}

/// Returns a set of commonly used default sorting rules.
/// Target paths are relative folder names to be created inside the source directory.
pub fn default_rules() -> Vec<SortingRule> {
    vec![
        // Images
        SortingRule::new("jpg", "Images", "Images"),
        SortingRule::new("jpeg", "Images", "Images"),
        SortingRule::new("png", "Images", "Images"),
        SortingRule::new("gif", "Images", "Images"),
        SortingRule::new("svg", "Images", "Images"),
        SortingRule::new("webp", "Images", "Images"),
        // Documents
        SortingRule::new("pdf", "Documents", "Documents"),
        SortingRule::new("doc", "Documents", "Documents"),
        SortingRule::new("docx", "Documents", "Documents"),
        SortingRule::new("txt", "Documents", "Documents"),
        SortingRule::new("xlsx", "Documents", "Documents"),
        SortingRule::new("pptx", "Documents", "Documents"),
        // Videos
        SortingRule::new("mp4", "Videos", "Videos"),
        SortingRule::new("mkv", "Videos", "Videos"),
        SortingRule::new("avi", "Videos", "Videos"),
        SortingRule::new("mov", "Videos", "Videos"),
        SortingRule::new("webm", "Videos", "Videos"),
        // Audio
        SortingRule::new("mp3", "Audio", "Audio"),
        SortingRule::new("wav", "Audio", "Audio"),
        SortingRule::new("flac", "Audio", "Audio"),
        SortingRule::new("ogg", "Audio", "Audio"),
        // Archives
        SortingRule::new("zip", "Archives", "Archives"),
        SortingRule::new("rar", "Archives", "Archives"),
        SortingRule::new("7z", "Archives", "Archives"),
        SortingRule::new("tar", "Archives", "Archives"),
        SortingRule::new("gz", "Archives", "Archives"),
        // Programs
        SortingRule::new("exe", "Programs", "Programs"),
        SortingRule::new("msi", "Programs", "Programs"),
        SortingRule::new("deb", "Programs", "Programs"),
        SortingRule::new("rpm", "Programs", "Programs"),
        SortingRule::new("AppImage", "Programs", "Programs"),
    ]
}
