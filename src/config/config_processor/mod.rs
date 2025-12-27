#[allow(dead_code)]
mod process_general;
mod process_presets;
mod process_utils;

use process_general::process_general;
use process_presets::process_presets;
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IrisConfig {
    pub general: GeneralConfig,
    // if no presets exist in the global iris.toml file, presets will be an empty vector
    pub presets: Vec<PresetConfig>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GeneralConfig {
    pub target: Option<Target>,
    pub mode: Mode,
    pub presets_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Target {
    Required,
    Downloads,
    CurrentDir,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Relative,
    Absolute,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PresetConfig {
    pub name: String,
    pub enabled: bool,
    pub extension: Vec<String>,
    pub relative_path: Option<PathBuf>,
    pub absolute_path: Option<PathBuf>,
}

impl IrisConfig {
    pub fn from_value(value: &toml::Value) -> Result<Self, anyhow::Error> {
        let general = process_general(value);
        let presets = process_presets(value);
        let config = IrisConfig { general, presets };
        Ok(config)
    }
}
