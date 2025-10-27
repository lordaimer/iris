use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use shellexpand;
use dunce;

#[cfg(windows)]
/// Remove surrounding quotes or a stray trailing quote from a path-like input on Windows.
/// This helps with PowerShell/cmd behaviors where a trailing backslash can yield an unmatched quote.
fn dequote(s: &str) -> std::borrow::Cow<'_, str> {
    if s.len() >= 2 {
        let starts_dq = s.starts_with('"');
        let ends_dq = s.ends_with('"');
        let starts_sq = s.starts_with('\'');
        let ends_sq = s.ends_with('\'');
        if (starts_dq && ends_dq) || (starts_sq && ends_sq) {
            return std::borrow::Cow::Owned(s[1..s.len() - 1].to_string());
        }
    }
    if s.ends_with('"') || s.ends_with('\'') {
        return std::borrow::Cow::Owned(s[..s.len() - 1].to_string());
    }
    std::borrow::Cow::Borrowed(s)
}

#[cfg(not(windows))]
/// On non-Windows we do not dequote: shells typically strip quotes already and
/// spaces/quotes may be intentional path characters.
fn dequote(s: &str) -> std::borrow::Cow<'_, str> {
    std::borrow::Cow::Borrowed(s)
}

/// Resolve and normalize a path string.
/// `must_exist` enforces existence (read-only). `canonicalize` follows symlinks (requires existence).
pub fn resolve_path(
    original_input: &str,
    must_exist: bool,
    canonicalize: bool,
) -> Result<PathBuf, String> {
    let input = dequote(original_input);
    if input.trim().is_empty() {
        return Err("empty path provided".into());
    }

    // Pre-expand Windows %VAR% syntax (no operations on non-Windows)
    let preexpanded = pre_expand_percent_vars(&input);

    // Expand ~ and environment variables like $VAR and ${VAR}
    #[cfg(not(windows))]
    let expanded_str: std::borrow::Cow<'_, str> = {
        let s = shellexpand::full(&preexpanded)
            .map_err(|e| format!("failed to expand '{}': {}", original_input, e))?;
        std::borrow::Cow::Owned(s.replace('\\', "/"))
    };

    #[cfg(windows)]
    let expanded_str = shellexpand::full(&preexpanded)
        .map_err(|e| format!("failed to expand '{}': {}", original_input, e))?;

    // Build raw PathBuf from expanded string
    let raw_path = PathBuf::from(expanded_str.as_ref());

    // Determine presence of prefix and root in components
    let mut has_prefix = false;
    let mut has_root = false;
    for comp in raw_path.components() {
        match comp {
            Component::Prefix(_) => has_prefix = true,
            Component::RootDir => has_root = true,
            _ => {}
        }
    }

    // On Windows a leading '/' (root-style) should not be joined to cwd's drive.
    let treat_as_root_style = cfg!(windows) && expanded_str.starts_with('/');

    // Decide whether to join with current dir
    let resolved = if raw_path.is_absolute()
        || treat_as_root_style
        // drive-relative like "C:foo" -> has_prefix true and no RootDir: don't join cwd
        || (cfg!(windows) && has_prefix && !has_root)
    {
        raw_path
    } else {
        let base = env::current_dir()
            .map_err(|e| format!("failed to get current directory: {}", e))?;
        base.join(raw_path)
    };

    // If canonicalize requested then use fs::canonicalize (will fail if not exist)
    let normalized = if canonicalize {
        fs::canonicalize(&resolved)
            .map_err(|e| format!("failed to canonicalize '{}': {}", original_input, e))?
    } else {
        // Normalize components (collapse ., .., mixed separators) without touching symlinks.
        normalize_components(&resolved)
    };

    // Use dunce to clean Windows extended prefixes if any.
    let normalized = dunce::simplified(&normalized).to_path_buf();

    // Validate existence if required
    if must_exist && !normalized.exists() {
        return Err(format!(
            "path does not exist: '{}' (resolved to: {})",
            original_input,
            normalized.display()
        ));
    }

    Ok(normalized)
}

/// Pre-expand Windows %VAR% style environment variables.
/// On non-Windows this returns the original string unchanged.
fn pre_expand_percent_vars(s: &str) -> String {
    #[cfg(windows)]
    {
        use std::env;
        let bytes = s.as_bytes();
        let mut i = 0;
        let mut out = String::with_capacity(s.len());
        while i < bytes.len() {
            if bytes[i] == b'%' {
                // find next '%'
                if let Some(rel_j) = (&s[i + 1..]).find('%') {
                    let j = i + 1 + rel_j;
                    let var = &s[i + 1..j];
                    let val = env::var(var).unwrap_or_default();
                    out.push_str(&val);
                    i = j + 1;
                    continue;
                }
            }
            out.push(bytes[i] as char);
            i += 1;
        }
        out
    }
    #[cfg(not(windows))]
    {
        s.to_string()
    }
}

/// Normalize a path by collapsing '.' and '..' and normalizing separators.
/// Does not canonicalize symlinks and does not require the path to exist.
/// Component-based. Keeps drive prefixes and UNC intact.
fn normalize_components(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    let mut seen_root = false;

    for comp in p.components() {
        match comp {
            Component::Prefix(pref) => {
                // drive letter or UNC prefix on Windows
                out.push(pref.as_os_str());
            }
            Component::RootDir => {
                // remember we saw a root so leading ".." shouldn't escape above it
                seen_root = true;
                // push a platform root marker so subsequent pushes produce absolute paths
                // using MAIN_SEPARATOR ensures correct platform separator
                out.push(std::path::MAIN_SEPARATOR.to_string());
            }
            Component::CurDir => { /* skip */ }
            Component::ParentDir => {
                // Try to pop one segment. If we can't and path is not rooted, preserve ".."
                if out.as_os_str().is_empty() {
                    if !seen_root {
                        out.push("..");
                    }
                } else {
                    let popped = out.pop();
                    if !popped && !seen_root {
                        out.push("..");
                    }
                }
            }
            Component::Normal(s) => out.push(s),
        }
    }

    if out.as_os_str().is_empty() {
        out.push(".");
    }

    out
}

// Convenience wrappers
#[allow(dead_code)]
/// resolve path where path must exist. no symlink expansion
pub fn resolve_path_strict(input: &str) -> Result<PathBuf, String> {
    resolve_path(input, true, false)
}

#[allow(dead_code)]
/// resolve path where path may not exist. no symlink expansion
pub fn resolve_path_permissive(input: &str) -> Result<PathBuf, String> {
    resolve_path(input, false, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_tilde() {
        let result = resolve_path("~/test", false, false).unwrap();
        let expected = dirs::home_dir()
            .expect("home directory should exist")
            .join("test");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_env_var() {
        env::set_var("TEST_VAR", "/tmp");
        let result = resolve_path("$TEST_VAR/file", false, false).unwrap();
        assert_eq!(result, PathBuf::from("/tmp/file"));
    }

    #[test]
    fn test_env_var_with_braces() {
        env::set_var("TEST_VAR2", "/var");
        let result = resolve_path("${TEST_VAR2}/log/app.log", false, false).unwrap();
        assert_eq!(result, PathBuf::from("/var/log/app.log"));
    }

    #[test]
    fn test_relative_current_dir() {
        let result = resolve_path("./src", false, false).unwrap();
        let expected = env::current_dir().unwrap().join("src");
        assert_eq!(result, dunce::simplified(&expected));
    }

    #[test]
    fn test_relative_parent_dir() {
        let result = resolve_path("../", false, false).unwrap();
        let expected = env::current_dir().unwrap().parent().unwrap().to_path_buf();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_absolute_path() {
        let result = resolve_path("/usr/bin", false, false).unwrap();
        assert_eq!(result, PathBuf::from("/usr/bin"));
    }

    #[test]
    fn test_nonexistent_lenient() {
        let result = resolve_path("/nonexistent/path/to/file", false, false).unwrap();
        assert_eq!(result, PathBuf::from("/nonexistent/path/to/file"));
    }

    #[test]
    fn test_nonexistent_strict() {
        let result = resolve_path("/nonexistent/path/to/file", true, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_empty_path() {
        let result = resolve_path("", false, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty path"));
    }

    #[test]
    fn test_whitespace_only() {
        let result = resolve_path("   ", false, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty path"));
    }

    #[test]
    fn test_dot_normalization() {
        let result = resolve_path("./foo/../bar/./baz", false, false).unwrap();
        let expected = env::current_dir().unwrap().join("bar/baz");
        assert_eq!(result, dunce::simplified(&expected));
    }

    #[test]
    fn test_canonicalize_with_existing_file() {
        // Create a temporary file to test canonicalization
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_canonicalize.txt");
        fs::write(&test_file, "test").expect("failed to create test file");

        let result = resolve_path(test_file.to_str().unwrap(), true, true).unwrap();
        assert!(result.is_absolute());

        // Cleanup the test file
        fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_canonicalize_nonexistent_fails() {
        let result = resolve_path("/nonexistent/file", true, true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("canonicalize"));
    }

    #[test]
    fn test_mixed_separators_windows_style() {
        let result = resolve_path("./foo\\bar/baz", false, false).unwrap();
        let expected = env::current_dir().unwrap().join("foo/bar/baz");
        assert_eq!(result, dunce::simplified(&expected));
    }

    #[test]
    fn test_tilde_with_relative_path() {
        let result = resolve_path("~/Documents/../Downloads", false, false).unwrap();
        let expected = dirs::home_dir()
            .expect("home directory should exist")
            .join("Downloads");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_env_and_tilde() {
        env::set_var("PROJECT", "myapp");
        let result = resolve_path("~/projects/$PROJECT/src", false, false).unwrap();
        let expected = dirs::home_dir()
            .expect("home directory should exist")
            .join("projects/myapp/src");
        assert_eq!(result, expected);
    }

    // Windows-only tests
    #[cfg(windows)]
    mod windows_tests {
        use super::*;
        use std::env;
        use std::path::PathBuf;

        #[test]
        fn test_drive_relative_not_joined() {
            // "C:foo" should be preserved as drive-relative, not joined to cwd
            let input = "C:some\\dir";
            let result = resolve_path(input, false, false).unwrap();
            assert_eq!(result, PathBuf::from("C:some\\dir"));
        }

        #[test]
        fn test_percent_env_expansion() {
            env::set_var("MYTESTVAR", "C:\\tmp");
            let result = resolve_path("%MYTESTVAR%/file.txt", false, false).unwrap();
            assert_eq!(result, PathBuf::from("C:\\tmp\\file.txt"));
        }

        #[test]
        fn test_unc_path_preserved() {
            let input = r"\\server\share\folder\file.txt";
            let result = resolve_path(input, false, false).unwrap();
            assert_eq!(result, PathBuf::from(r"\\server\share\folder\file.txt"));
        }

        #[test]
        fn test_dequote_surrounding_quotes_and_spaces_preserved() {
            let s = r#""C:\\path with spaces""#;
            let dq = dequote(s);
            assert_eq!(dq, r#"C:\\path with spaces"#);
        }

        #[test]
        fn test_dequote_stray_trailing_quote_removed() {
            let s = r#"C:\\Program Files\\""#;
            let dq = dequote(s);
            assert_eq!(dq, r#"C:\\Program Files\\"#);
        }

        #[test]
        fn test_dequote_embedded_quote_unchanged() {
            let s = r#"C:\\foo\"bar"#;
            let dq = dequote(s);
            assert_eq!(dq, s);
        }
    }

    // Non-Windows: dequote is a no-op
    #[cfg(not(windows))]
    mod non_windows_tests {
        use super::*;

        #[test]
        fn test_dequote_noop_preserves_whitespace_and_quotes() {
            let s = r#" "/path with spaces" "#;
            let dq = dequote(s);
            assert_eq!(dq, s);
        }
    }
}

