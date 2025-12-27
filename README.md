<h1>Iris</h1>
<p>
    <a href="https://github.com/lordaimer/iris/releases/latest"><img alt="GitHub Release" src="https://img.shields.io/github/v/release/lordaimer/iris?color=greenlight&label=latest%20release"></a>
    <a href="https://github.com/lordaimer/iris/actions"><img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/lordaimer/iris/ci.yml?label=tests"></a>
</p>

A fast, minimal, config-driven file organizer built with Rust.  
Iris helps you sort and organize your messy folders automatically using defined rules.

## Features
 - ⚡ **Fast**: Built in Rust for high performance.
 - 💻 **Multiplatform**: Runs on Windows, Linux, and macOS.
 - 🖱️ **Context Menu**: Right-click "Sort with Iris" support on Windows.
 - 📁 **Config Driven**: Customize behavior with a simple `iris.toml` file.
 - 🤖 **Smart Presets**: Comes with sensible defaults for common file types.

## Installation

### Download Binary
Download the latest release for your platform from the [Releases Page](https://github.com/lordaimer/iris/releases).

### From Crates.io
```bash
cargo install iris-cli
```

## Usage

### Basic Sorting
To sort a directory using default settings:
```bash
iris sort /path/to/folder
```

### Windows Context Menu
On Windows, you can add Iris to the right-click menu:
```powershell
iris context install
```
Now simply right-click any folder background and select **"Sort with Iris"**.

## Configuration
Iris automatically creates a default configuration file at:
- **Windows**: `%APPDATA%\iris\iris.toml`
- **Linux/macOS**: `~/.config/iris/iris.toml`

Can be customized to define where files go based on extensions or patterns.
```bash
iris config edit
```

```toml
[preset.images]
enabled = true
# Sorts .jpg and .png into "Pictures" folder
extension = ["jpg", "png"]
relative_path = "Pictures"
```