use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;

use super::rules::SortingRule;

/// Represents a single file move operation (for undo support)
#[derive(Debug, Clone)]
pub struct MoveRecord {
    pub original_path: PathBuf,
    pub new_path: PathBuf,
    /// Whether the target directory was created by this operation
    pub created_dir: Option<PathBuf>,
}

/// Scans a directory and returns a list of files (excluding directories).
/// Only scans the top-level files (depth = 1), not recursively.
pub fn scan_directory<P: AsRef<Path>>(dir_path: P) -> Vec<PathBuf> {
    WalkDir::new(dir_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}

/// Matches a file extension to a target path based on the enabled rules.
pub fn match_rule<'a>(extension: &str, rules: &'a [SortingRule]) -> Option<&'a str> {
    rules
        .iter()
        .filter(|r| r.enabled)
        .find(|r| r.extension.eq_ignore_ascii_case(extension))
        .map(|r| r.target_path.as_str())
}

/// Computes a safe file path to avoid overwriting existing files.
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
/// Returns Ok(Some(MoveRecord)) if moved, Ok(None) if no rule matched.
/// Tracks whether a new directory was created for undo support.
pub fn organize_file(file_path: &Path, source_dir: &Path, rules: &[SortingRule]) -> std::io::Result<Option<MoveRecord>> {
    if let Some(target_folder_name) = file_path
        .extension()
        .and_then(|e| e.to_str())
        .and_then(|ext| match_rule(ext, rules))
    {
        let target_dir = source_dir.join(target_folder_name);

        // Track if we created this directory
        let created_dir = if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
            Some(target_dir.clone())
        } else {
            None
        };

        let original_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown_file");

        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let dest_path = get_safe_destination_path(&target_dir, original_name, ext);

        fs::rename(file_path, &dest_path)?;

        return Ok(Some(MoveRecord {
            original_path: file_path.to_path_buf(),
            new_path: dest_path,
            created_dir,
        }));
    }
    Ok(None)
}

/// Undoes a list of move operations by moving files back to their original paths.
/// Also removes directories that were created by the program (if now empty).
/// Returns the number of successfully restored files.
pub fn undo_moves(records: &[MoveRecord]) -> std::io::Result<usize> {
    let mut restored = 0;
    let mut dirs_to_remove: Vec<PathBuf> = Vec::new();

    // Restore files in reverse order
    for record in records.iter().rev() {
        if record.new_path.exists() {
            if let Some(parent) = record.original_path.parent()
                && !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            fs::rename(&record.new_path, &record.original_path)?;
            restored += 1;

            // Mark created directories for cleanup
            if let Some(dir) = &record.created_dir
                && !dirs_to_remove.contains(dir) {
                    dirs_to_remove.push(dir.clone());
                }
        }
    }

    // Remove directories that were created by the program (only if empty)
    for dir in &dirs_to_remove {
        if dir.exists() && dir.is_dir() && fs::read_dir(dir)?.next().is_none() {
            fs::remove_dir(dir)?;
        }
    }

    Ok(restored)
}
