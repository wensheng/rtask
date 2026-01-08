//! Common test utilities

use std::fs;
use tempfile::TempDir;

/// Create a temporary directory with a rtask.yml file
pub fn create_test_config(content: &str) -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("rtask.yml");
    fs::write(&config_path, content).unwrap();
    (temp_dir, config_path)
}
