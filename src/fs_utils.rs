use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
#[derive(Debug, Clone, Copy)]
pub enum HashAlgo {
    Simple,
    Blake3,
}
pub fn hash_file(path: &Path, algo: HashAlgo) -> Result<String> {
    let content = fs::read(path)?;
    Ok(match algo {
        HashAlgo::Simple => {
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            hasher.finish().to_string()
        }
        HashAlgo::Blake3 => blake3::hash(&content).to_string(),
    })
}
pub fn hash_content(content: &str, algo: HashAlgo) -> String {
    match algo {
        HashAlgo::Simple => {
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            hasher.finish().to_string()
        }
        HashAlgo::Blake3 => blake3::hash(content.as_bytes()).to_string(),
    }
}
pub fn write_if_changed(path: &Path, content: &str, algo: HashAlgo, dry_run: bool) -> Result<bool> {
    let new_hash = hash_content(content, algo);
    let changed = if path.exists() {
        let old_hash = hash_file(path, algo)?;
        old_hash != new_hash
    } else {
        true
    };
    if changed && !dry_run {
        fs::write(path, content)?;
    }
    Ok(changed)
}
