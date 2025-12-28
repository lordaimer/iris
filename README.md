<h1>Iris</h1>
<p>
    <a href="https://github.com/lordaimer/iris/releases/latest">
        <img alt="GitHub Release" src="https://img.shields.io/github/v/release/lordaimer/iris?color=greenlight&label=latest%20release">
    </a>
    <a href="https://github.com/lordaimer/iris/actions">
        <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/lordaimer/iris/ci.yml?label=tests">       </a>
    <a href="https://crates.io/crates/iris-cli">
      <img alt="Crates.io" src="https://img.shields.io/crates/v/iris-cli">
    </a>
    <img alt="License" src="https://img.shields.io/github/license/lordaimer/iris">
</p>

A fast, minimal, config-driven file organizer built with Rust.  
Iris helps you sort and organize your messy folders automatically using defined rules.

<video src="https://github.com/user-attachments/assets/51a888e9-80ba-4c16-8629-2e09ae93aa07" controls width="100%"></video>

## Features
 - ⚡ **Fast**: Built in Rust for high performance.
 - 💻 **Multiplatform**: Runs on Windows, Linux, and macOS.
 - 🖱️ **Context Menu**: Right-click "Sort with Iris" support on Windows.
 - 📁 **Config Driven**: Customize behavior with a simple `iris.toml` file.
 - 📄 **Shell Completion**: Support for shell completion scripts.
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

<details>
    <summary>
        <b>Demo: Windows Context Menu</b>
    </summary>
    <video src="https://github.com/user-attachments/assets/7c366ac9-9c9a-4428-be70-1234846c1de1" controls width="100%"></video>
</details>

## Configuration
Iris automatically creates a default configuration file at:
- **Windows**: `%APPDATA%\Iris\iris.toml`
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

## Shell Completion

Automatically generate and install completion scripts for your shell. You can also generate the completion script manually and install it yourself with `iris completions <SHELL>` command.

```bash
iris completions install
```
