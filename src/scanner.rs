use crate::config::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
}
pub fn scan_directory(dir: &Path, config: &Config) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    let entries =
        fs::read_dir(dir).with_context(|| format!("failed to read directory {}", dir.display()))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if config.is_ignored(&path) {
            continue;
        }
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) if !s.is_empty() && s != "mod" => s,
            _ => continue,
        };
        modules.push(Module {
            name: stem.to_string(),
            path,
        });
    }
    modules.sort_by(|a, b| {
        let a_key = a.name.to_ascii_lowercase();
        let b_key = b.name.to_ascii_lowercase();
        a_key.cmp(&b_key)
    });
    Ok(modules)
}
