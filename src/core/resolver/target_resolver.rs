/// Resolve the path to target based on the target field in config file
use crate::config::config_processor::{Target, IrisConfig};
use std::path::PathBuf;
use std::env;

use crate::paths::path_resolve::resolve_path_strict;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TargetResolutionError {
    PathRequiredButNotProvided,
    FailedToGetDownloadsDir,
    FailedToGetCurrentDir,
    ProvidedPathInvalid,
}

impl std::fmt::Display for TargetResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetResolutionError::PathRequiredButNotProvided => {
                write!(f, "target is set to 'required' in config. you must provide a path")
            },
            TargetResolutionError::ProvidedPathInvalid => {
                write!(f, "provided path doesn't exist or is invalid")
            }
            TargetResolutionError::FailedToGetCurrentDir => {
                write!(f, "failed to get current working directory")
            },
            TargetResolutionError::FailedToGetDownloadsDir => {
                write!(f, "failed to get downloads directory")
            }
        }
    }
}

impl std::error::Error for TargetResolutionError {}

#[allow(dead_code)]
fn try_resolve(path: &str) -> Result<PathBuf, TargetResolutionError> {
    resolve_path_strict(path).map_err(|_| TargetResolutionError::ProvidedPathInvalid)
}

#[allow(dead_code)]
/// Resolves the actual target path based on config and CLI arguments
pub fn resolve_target(config: &IrisConfig, cli_path: Option<&String>) -> Result<PathBuf, TargetResolutionError> {
    match &config.general.target {
        Some(Target::Required) | None => {
            cli_path.map_or(
                Err(TargetResolutionError::PathRequiredButNotProvided),
                |p| try_resolve(p),
            )
        }
        Some(Target::Downloads) => {
            if let Some(p) = cli_path {
                return try_resolve(p);
            }
            let dl = dirs::download_dir().ok_or(TargetResolutionError::FailedToGetDownloadsDir)?;
            try_resolve(dl.to_string_lossy().as_ref())
        }
        Some(Target::CurrentDir) => {
            if let Some(p) = cli_path {
                return try_resolve(p);
            }
            let cwd = env::current_dir().map_err(|_| TargetResolutionError::FailedToGetCurrentDir)?;
            try_resolve(cwd.to_string_lossy().as_ref())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::config_processor::{GeneralConfig, Mode};
    use tempfile::tempdir;
    use std::{fs, env};

    fn create_test_config(target: Option<Target>) -> IrisConfig {
        IrisConfig {
            general: GeneralConfig {
                target,
                mode: Mode::Relative,
                presets_path: None,
            },
            presets: vec![],
        }
    }

    #[test]
    fn test_required_with_path() {
        let config = create_test_config(Some(Target::Required));
        let temp_dir = tempdir().unwrap();
        let valid_path = temp_dir.path().join("somefile.txt");
        fs::write(&valid_path, "mock data").unwrap();
        let path_str = valid_path.to_string_lossy().to_string();

        let result = resolve_target(&config, Some(&path_str));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), valid_path);
    }

    #[test]
    fn test_required_without_path() {
        let config = create_test_config(Some(Target::Required));
        let result = resolve_target(&config, None);
        assert!(matches!(
            result,
            Err(TargetResolutionError::PathRequiredButNotProvided)
        ));
    }

    #[test]
    fn test_downloads_without_path() {
        let config = create_test_config(Some(Target::Downloads));
        let result = resolve_target(&config, None);
        assert!(result.is_ok());
        let expected = dirs::download_dir().unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_downloads_with_path() {
        let config = create_test_config(Some(Target::Downloads));
        let temp_dir = tempdir().unwrap();
        let path_str = temp_dir.path().to_string_lossy().to_string();

        let result = resolve_target(&config, Some(&path_str));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_current_dir_without_path() {
        let config = create_test_config(Some(Target::CurrentDir));
        let result = resolve_target(&config, None);
        assert!(result.is_ok());
        let expected = env::current_dir().unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_current_dir_with_path() {
        let config = create_test_config(Some(Target::CurrentDir));
        let temp_dir = tempdir().unwrap();
        let path_str = temp_dir.path().to_string_lossy().to_string();

        let result = resolve_target(&config, Some(&path_str));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_none_target_with_path() {
        let config = create_test_config(None);
        let temp_dir = tempdir().unwrap();
        let path_str = temp_dir.path().to_string_lossy().to_string();

        let result = resolve_target(&config, Some(&path_str));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_invalid_path() {
        let config = create_test_config(Some(Target::Required));
        let path = String::from("/definitely/nonexistent/path");
        let result = resolve_target(&config, Some(&path));
        assert!(matches!(
            result,
            Err(TargetResolutionError::ProvidedPathInvalid)
        ));
    }
}