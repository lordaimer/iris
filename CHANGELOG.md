# Changelog

All notable changes to this project will be documented in this file.  
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),

## [1.3.4](https://github.com/lordaimer/iris/compare/v1.3.3...v1.3.4) - 2025-12-29

### What's changed

v1.3.4 updates the CI workflow to trigger chocolatey update only after release completes & release == success

## [1.3.3](https://github.com/lordaimer/iris/compare/v1.3.2...v1.3.3) - 2025-12-29

### What's changed

v1.3.3 moves all the chocolatey packaging to dist/chocolatey branch and updates the github actions workflow to make new commits related to chocolatey in the dist/chocolatey branch

## [1.3.2](https://github.com/lordaimer/iris/compare/v1.3.1...v1.3.2) - 2025-12-29

### What's New
v1.3.2 introduces a pre-push hook that verifies the changelog entry exists for the version being pushed.

### Added
- pre-push hook to verify changelog entry exists for the version being pushed

### Fixed
- Fix build failures when changelog entry doesn't exist for the version being built

## [1.3.1](https://github.com/lordaimer/iris/compare/v1.3.0...v1.3.1) - 2025-12-29

### What's Changed
v1.3.1 gates PowerShell completions install when execution policy is restrictive and adds Chocolatey package configuration and automation.

### Added
- Gate PowerShell completions install when execution policy is restrictive
- Chocolatey package configuration and automation

## [1.3.0](https://github.com/lordiamer/iris/compare/v1.2.0...v1.3.0) - 2025-12-28

### What's New
v1.3.0 adds support for shell completions on unix platforms.

### Added
- support for auto-install and auto-uninstall of shell completions on unix platforms
- create shell completion file on "~/.config/iris/completions/", source it on rc files
- uninstall: remove source line from rc files
- support for zsh, fish, bash

### Changed
- update README.md

## [1.2.0](https://github.com/lordaimer/iris/compare/v1.1.1...v1.2.0) - 2025-12-28

### What’s New
v1.2.0 introduces shell completion support, with automatic setup on Windows and manual generation support for other platforms.

### Added
- `iris completion` command for generating shell completion scripts
- Automatic installation of shell completion on Windows (PowerShell & Git Bash)
- Manual shell completion script generation for other OSes

### Known Issues
- Auto-install not implemented on non-Windows platforms

## [1.1.1](https://github.com/lordaimer/iris/releases/tag/v1.1.1) - 2025-12-27

### What’s New
Initial public release with core file-sorting functionality and a config-driven workflow.

### Added
- Sort files based on their extension
- Simple TOML configuration file support
- Prevent sorting on protected system paths
