use crate::config::{Config, HashAlgo as ConfigHashAlgo};
use crate::fs_utils::{HashAlgo, write_if_changed};
use crate::scanner::{Module, scan_directory};
use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::{debug, info};
use walkdir::WalkDir;
pub struct Generator {
    config: Config,
    dry_run: bool,
    no_reexport: bool,
    check_mode: bool,
}
impl Generator {
    pub fn new(config: Config, dry_run: bool, no_reexport: bool, check_mode: bool) -> Self {
        Self {
            config,
            dry_run,
            no_reexport,
            check_mode,
        }
    }
    pub fn run(&self, root: &Path) -> Result<()> {
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| !self.config.is_ignored(e.path()))
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if path.is_dir() {
                self.process_directory(path)?;
            }
        }
        Ok(())
    }
    fn process_directory(&self, dir: &Path) -> Result<()> {
        let modules = scan_directory(dir, &self.config)?;
        if modules.is_empty() {
            return Ok(());
        }
        let mod_path = dir.join("mod.rs");
        let existing_content = if mod_path.exists() {
            Some(fs::read_to_string(&mod_path)?)
        } else {
            None
        };
        let new_content = self.build_content(&modules, existing_content.as_deref());
        if self.check_mode {
            if existing_content.as_deref() != Some(new_content.as_str()) {
                eprintln!("{} would change", mod_path.display());
                std::process::exit(1);
            }
            debug!("✓ {} is up-to-date", mod_path.display());
        } else {
            let algo = match self.config.hash_algo {
                ConfigHashAlgo::Simple => HashAlgo::Simple,
                ConfigHashAlgo::Blake3 => HashAlgo::Blake3,
            };
            let changed = write_if_changed(&mod_path, &new_content, algo, self.dry_run)?;
            if changed {
                info!(
                    "{} {}",
                    if self.dry_run {
                        "Would update"
                    } else {
                        "Updated"
                    },
                    mod_path.display()
                );
            } else {
                debug!("No change for {}", mod_path.display());
            }
        }
        Ok(())
    }
    fn build_content(&self, modules: &[Module], existing: Option<&str>) -> String {
        let marker_start = &self.config.marker_start;
        let marker_end = &self.config.marker_end;
        if let Some(existing) = existing {
            if let (Some(start_pos), Some(end_pos)) =
                (existing.find(marker_start), existing.find(marker_end))
            {
                if start_pos < end_pos {
                    let before = &existing[..start_pos + marker_start.len()];
                    let after = &existing[end_pos..];
                    let mut new = String::new();
                    new.push_str(before);
                    new.push('\n');
                    self.write_module_block(&mut new, modules);
                    new.push_str(after);
                    return new;
                }
            }
        }
        let mut content = String::new();
        content.push_str(marker_start);
        content.push('\n');
        self.write_module_block(&mut content, modules);
        content.push_str(marker_end);
        content.push('\n');
        content
    }
    fn write_module_block(&self, output: &mut String, modules: &[Module]) {
        for m in modules {
            output.push_str(&format!("pub mod {};\n", m.name));
        }
        output.push('\n');
        if !self.no_reexport && self.config.reexport {
            for m in modules {
                output.push_str(&format!("pub use {}::*;\n", m.name));
            }
        }
    }
}
