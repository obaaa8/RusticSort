use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;

use super::rules::SortingRule;

/// Scans a directory and returns a list of files (excluding directories)
pub fn scan_directory<P: AsRef<Path>>(dir_path: P) -> Vec<PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}

/// Matches a file extension to a target path based on the rules.
pub fn match_rule<'a>(extension: &str, rules: &'a [SortingRule]) -> Option<&'a str> {
    rules
        .iter()
        .find(|r| r.extension.eq_ignore_ascii_case(extension))
        .map(|r| r.target_path.as_str())
}

/// Computes a safe file path to avoid overwriting existing files.
/// Example: if `document.pdf` exists, tries `document_1.pdf`.
pub fn get_safe_destination_path(target_dir: &Path, original_name: &str, extension: &str) -> PathBuf {
    let mut candidate = target_dir.join(format!("{}.{}", original_name, extension));
    let mut counter = 1;

    while candidate.exists() {
        candidate = target_dir.join(format!("{}_{}.{}", original_name, counter, extension));
        counter += 1;
    }
    candidate
}

/// Moves a single file based on the sorting rules.
/// If no rule matches, returns Ok(false).
/// If it was moved successfully, returns Ok(true).
pub fn organize_file(file_path: &Path, rules: &[SortingRule]) -> std::io::Result<bool> {
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        if let Some(target_dir_str) = match_rule(ext, rules) {
            let target_dir = Path::new(target_dir_str);
            
            // Create target directory if it doesn't exist
            if !target_dir.exists() {
                fs::create_dir_all(target_dir)?;
            }

            let original_name = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown_file");

            let dest_path = get_safe_destination_path(target_dir, original_name, ext);

            // Move the file
            fs::rename(file_path, &dest_path)?;

            return Ok(true);
        }
    }
    Ok(false)
}
