# Changelog

## [1.2.0](https://github.com/lordaimer/iris/compare/v1.1.1...v1.2.0) - 2025-12-28

### What’s New
v1.2.0 introduces shell completion support, with automatic setup on Windows and manual generation support for other platforms.

### Added
- `iris completion` command for generating shell completion scripts
- Automatic installation of shell completion on Windows (PowerShell & Git Bash)
- Manual shell completion script generation for other OSes

### Known Issues
- Auto-install not implemented on non-Windows platforms

---

## [1.1.1](https://github.com/lordaimer/iris/releases/tag/v1.1.1) - 2025-12-27

### What’s New
Initial public release with core file-sorting functionality and a config-driven workflow.

### Added
- Sort files based on their extension
- Simple TOML configuration file support
- Prevent sorting on protected system paths
