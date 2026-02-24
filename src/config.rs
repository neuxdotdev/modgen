use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default = "default_true")]
    pub reexport: bool,
    #[serde(default = "default_marker_start")]
    pub marker_start: String,
    #[serde(default = "default_marker_end")]
    pub marker_end: String,
    #[serde(default)]
    pub hash_algo: HashAlgo,
}
fn default_true() -> bool {
    true
}
fn default_marker_start() -> String {
    "// <modgen:start>".to_string()
}
fn default_marker_end() -> String {
    "// <modgen:end>".to_string()
}
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub enum HashAlgo {
    #[default]
    Simple,
    Blake3,
}
impl Config {
    pub fn is_ignored(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.ignore.iter().any(|pattern| path_str.contains(pattern))
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            ignore: vec!["target".into(), ".git".into(), "node_modules".into()],
            reexport: default_true(),
            marker_start: default_marker_start(),
            marker_end: default_marker_end(),
            hash_algo: HashAlgo::default(),
        }
    }
}
pub fn load_config(path: Option<&PathBuf>) -> Result<Config> {
    let mut config = Config::default();
    if let Some(path) = path {
        if path.exists() {
            let content = fs::read_to_string(path)
                .with_context(|| format!("failed to read config file {}", path.display()))?;
            config = toml::from_str(&content).with_context(|| "failed to parse TOML config")?;
        }
    }
    Ok(config)
}
