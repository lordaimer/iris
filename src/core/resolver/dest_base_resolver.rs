// This module is responsible for resolving the destination base path for a given preset and mode
// This should be called in an iterator over the enabled presets
use crate::config::config_processor::{Mode, PresetConfig};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn get_dest_base(target: &Path, preset: &PresetConfig, mode: Mode) -> Result<PathBuf, String> {
    match mode {
        Mode::Relative => {
            let rel = preset
                .relative_path
                .as_ref()
                .ok_or_else(|| format!("preset '{}' missing relative_path", preset.name))?;
            Ok(target.join(rel))
        }
        Mode::Absolute => {
            let abs = preset
                .absolute_path
                .as_ref()
                .ok_or_else(|| format!("preset '{}' missing absolute_path", preset.name))?;
            Ok(abs.to_owned())
        }
    }
}
