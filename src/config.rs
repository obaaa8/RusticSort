use std::fs;
use std::path::PathBuf;
use std::io;

use crate::engine::rules::SortingRule;

const APP_DIR_NAME: &str = "RusticSort";
const CONFIG_FILE_NAME: &str = "config.json";

/// Returns the path to the application configuration directory based on the OS.
pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push(APP_DIR_NAME);
        path
    })
}

/// Returns the full path to the configuration file (config.json).
pub fn get_config_file_path() -> Option<PathBuf> {
    get_config_dir().map(|mut path| {
        path.push(CONFIG_FILE_NAME);
        path
    })
}

/// Loads sorting rules from the config file. If the file doesn't exist, returns an empty list.
pub fn load_rules() -> io::Result<Vec<SortingRule>> {
    let config_path = get_config_file_path()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find config directory"))?;

    if !config_path.exists() {
        return Ok(Vec::new());
    }

    let file_content = fs::read_to_string(config_path)?;
    let rules: Vec<SortingRule> = serde_json::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(rules)
}

/// Saves sorting rules to the config file (creates the file and directories if needed).
pub fn save_rules(rules: &[SortingRule]) -> io::Result<()> {
    let config_dir = get_config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find config directory"))?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = get_config_file_path().unwrap();
    let json = serde_json::to_string_pretty(rules)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    fs::write(config_path, json)?;

    Ok(())
}
