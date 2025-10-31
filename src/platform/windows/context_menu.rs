// TODO: Add context entry on first run in a system. Respect user's choice if they choose to do iris context uninstall
#[cfg(target_os = "windows")]
use anyhow::{Context, Result};
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use std::fs;
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(target_os = "windows")]
fn get_current_exe_path() -> Result<PathBuf> {
    let exe = std::env::current_exe().context("failed to resolve current executable path")?;
    Ok(dunce::canonicalize(exe).unwrap_or_else(|_| std::env::current_exe().unwrap()))
}

#[cfg(target_os = "windows")]
fn write_directory_menu(installed_exe: &str) -> Result<()> {
    // HKCU\Software\Classes\Directory\shell\Sort with Iris\command
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu
        .open_subkey_with_flags("Software\\Classes", KEY_READ | KEY_WRITE)
        .context("failed to open HKCU\\Software\\Classes")?;

    let (shell_key, _) = classes
        .create_subkey("Directory\\shell\\Sort with Iris")
        .context("failed to create context menu key for Directory")?;

    shell_key.set_value("MUIVerb", &"Sort with Iris").ok();
    shell_key.set_value("Icon", &installed_exe).ok();

    let (cmd_key, _) = shell_key
        .create_subkey("command")
        .context("failed to create command subkey")?;
    let command = format!("\"{}\" {} \"%1\"", installed_exe, "sort");
    cmd_key
        .set_value("", &command)
        .context("failed to set command for context menu")?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn write_directory_background_menu(installed_exe: &str) -> Result<()> {
    // HKCU\Software\Classes\Directory\Background\shell\Sort with Iris\command
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu
        .open_subkey_with_flags("Software\\Classes", KEY_READ | KEY_WRITE)
        .context("failed to open HKCU\\Software\\Classes")?;

    let (shell_key, _) = classes
        .create_subkey("Directory\\Background\\shell\\Sort with Iris")
        .context("failed to create context menu key for Directory Background")?;

    shell_key.set_value("MUIVerb", &"Sort with Iris").ok();
    shell_key.set_value("Icon", &installed_exe).ok();

    let (cmd_key, _) = shell_key
        .create_subkey("command")
        .context("failed to create command subkey")?;
    // %V expands to the current folder when clicking background
    let command = format!("\"{}\" {} \"%V\"", installed_exe, "sort");
    cmd_key
        .set_value("", &command)
        .context("failed to set command for background context menu")?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn delete_tree_if_exists(root: &RegKey, path: &str) -> Result<()> {
    match root.open_subkey_with_flags(path, KEY_READ | KEY_WRITE) {
        Ok(sub) => {
            root.delete_subkey_all(path)
                .with_context(|| format!("failed deleting key {}", path))?;
            drop(sub);
            Ok(())
        }
        Err(_) => Ok(()),
    }
}

#[cfg(target_os = "windows")]
pub fn install_context_menu() -> Result<()> {
    // Determine destination: %APPDATA%/Iris/iris.exe
    let current_exe = get_current_exe_path()?;
    let data_dir = dirs::data_dir().context("failed to resolve data_dir (AppData/Roaming)")?;
    let iris_dir = data_dir.join("Iris");
    fs::create_dir_all(&iris_dir).context("failed to create Iris app data directory")?;
    let installed_exe_path = iris_dir.join("iris.exe");
    if installed_exe_path.exists() {
        // Replace existing file
        let _ = fs::remove_file(&installed_exe_path);
    }
    fs::copy(&current_exe, &installed_exe_path)
        .with_context(|| format!("failed to copy iris.exe to {}", installed_exe_path.display()))?;

    let installed_exe = installed_exe_path.to_string_lossy().to_string();
    write_directory_menu(&installed_exe)?;
    write_directory_background_menu(&installed_exe)?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn uninstall_context_menu() -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu
        .open_subkey_with_flags("Software\\Classes", KEY_READ | KEY_WRITE)
        .context("failed to open HKCU\\Software\\Classes")?;

    delete_tree_if_exists(&classes, "Directory\\shell\\Sort with Iris")?;
    delete_tree_if_exists(&classes, "Directory\\Background\\shell\\Sort with Iris")?;
    // Remove installed executable from %APPDATA%/Iris
    if let Some(data_dir) = dirs::data_dir() {
        let installed_exe_path = data_dir.join("Iris").join("iris.exe");
        if installed_exe_path.exists() {
            let _ = fs::remove_file(installed_exe_path);
        }
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn install_context_menu() -> Result<()> { Err(anyhow::anyhow!("context menu is only supported on Windows")) }

#[cfg(not(target_os = "windows"))]
pub fn uninstall_context_menu() -> Result<()> { Err(anyhow::anyhow!("context menu is only supported on Windows")) }