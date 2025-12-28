// no unit tests are not needed for this module because
// on GitHub actions, headless environments might not have $SHELL or $PSModulePath
// also almost all functions are I/O or Environment Bound

use super::cli_parser::{Cli, ShellCompletionAction};
use clap::CommandFactory;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_complete::{generate, Generator};
use colored::Colorize;
use std::process::Command;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn handle_completion(action: &ShellCompletionAction) {
    let mut cmd = Cli::command();
    match action {
        ShellCompletionAction::Bash => print_completion(Bash, &mut cmd),
        ShellCompletionAction::Elvish => print_completion(Elvish, &mut cmd),
        ShellCompletionAction::Fish => print_completion(Fish, &mut cmd),
        ShellCompletionAction::Powershell => print_completion(PowerShell, &mut cmd),
        ShellCompletionAction::Zsh => print_completion(Zsh, &mut cmd),
        ShellCompletionAction::Install => install_completion(&mut cmd),
        ShellCompletionAction::Uninstall => uninstall_completion(),
    }
}

fn print_completion<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, "iris", &mut io::stdout());
}

fn install_completion(cmd: &mut clap::Command) {
    // detect shell and install
    // on windows, prioritize PowerShell and git bash (Bash)
    if cfg!(target_os = "windows") {
        install_windows(cmd);
    } else {
        install_unix(cmd);
    }
}

#[cfg(target_os = "windows")]
fn install_windows(cmd: &mut clap::Command) {
    let shell = std::env::var("SHELL").unwrap_or_default();
    // check if running in git bash
    let is_git_bash = shell.ends_with("bash");

    // check if running in powershell
    let is_powershell = std::env::var("PSModulePath").is_ok();

    if is_powershell && !is_git_bash {
        if let Some(policy) = powershell_execution_policy() {
            if matches!(policy.as_str(), "Restricted" | "AllSigned"){
                eprintln!(
                    "\nPowerShell execution policy is too restrictive ({policy})\n\
                    Completion scripts cannot be sourced.\n\n\
                    Fix:\n\
                    Run this once in PowerShell:\n\n\
                    Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned\n"
                );
                return;
            }
        }
        let profile_paths = get_powershell_profiles();
        if !profile_paths.is_empty() {
            // use standard config dir for completions
            let config_dir = crate::paths::config_path::get_config_dir();
            let iris_completions = config_dir.join("Completions");

            if let Err(e) = fs::create_dir_all(&iris_completions) {
                eprintln!("Failed to create completions directory: {}", e);
                return;
            }

            let ps_file = iris_completions.join("_iris.ps1");
            let mut file = match fs::File::create(&ps_file) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to create completion file: {}", e);
                    return;
                }
            };

            generate(PowerShell, cmd, "iris", &mut file);
            println!("Installed PowerShell completion to: {}", ps_file.display());

            for profile_path in profile_paths {
                // check if profile exists, if not create it
                if !profile_path.exists() {
                    if let Some(parent) = profile_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::write(&profile_path, "");
                }

                // check if already sourced
                let profile_content = fs::read_to_string(&profile_path).unwrap_or_default();
                let source_line = format!(". \"{}\"", ps_file.display());

                // source the shell completions file if not already sourced
                if !profile_content.contains(&source_line) {
                    // append to profile
                    use std::io::Write;
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&profile_path)
                        .expect("Failed to open profile for appending");

                    if let Err(e) = writeln!(file, "\n{}", source_line) {
                        eprintln!(
                            "Failed to append to profile {}: {}",
                            profile_path.display(),
                            e
                        );
                    } else {
                        println!(
                            "Added sourcing line to PowerShell profile: {}",
                            profile_path.display().to_string().as_str().cyan()
                        );

                    }
                } else {
                    println!(
                        "PowerShell profile already sources this file: {}",
                        profile_path.display().to_string().as_str().cyan()
                    );
                }
            }
            println!(
                "Restart PowerShell to activate completions!",
            );
        } else {
            println!("Could not detect any PowerShell profile.");
        }
    // if environment is Git Bash:
    } else if is_git_bash {
        if let Some(home) = dirs::home_dir() {
            let completions_dir = home.join(".bash_completions");
            if let Err(_) = fs::create_dir_all(&completions_dir) {
                // ignore
            }

            let bash_file = completions_dir.join("iris.bash");
            if let Ok(mut file) = fs::File::create(&bash_file) {
                generate(Bash, cmd, "iris", &mut file);
                println!("\nInstalled Bash completion to: {}", bash_file.display());
                println!(
                    "To enable, ensure this file is sourced in your ~/.bashrc or ~/.bash_profile:"
                );
                println!(
                    "{}",
                    format!("source \"{}\"", bash_file.display())
                        .as_str()
                        .green()
                );
            }
        }
    } else {
        println!("Could not detect a supported shell (PowerShell or Git Bash) for automatic installation.");
        println!("Please generate the script manually using 'iris completion <SHELL> > <FILE>'");
    }
}

// omit unnecessary functions
#[cfg(not(target_os = "windows"))]
fn install_windows(_cmd: &mut clap::Command) {}

#[cfg(target_os = "windows")]
fn install_unix(_cmd: &mut clap::Command) {}

#[cfg(target_os = "windows")]
fn uninstall_unix() {}

#[cfg(not(target_os = "windows"))]
fn install_unix(cmd: &mut clap::Command) {
    let shell = std::env::var("SHELL").unwrap_or_default();
    let config_dir = crate::paths::config_path::get_config_dir();
    let completions_dir = config_dir.join("completions");

    if let Err(e) = fs::create_dir_all(&completions_dir) {
        eprintln!("Failed to create completions directory: {}", e);
        return;
    }

    // determine shell type
    if shell.contains("zsh") {
        install_zsh(cmd, &completions_dir);
    } else if shell.contains("fish") {
        install_fish(cmd);
    } else {
        // default to bash if shell is bash or unknown (safe fallback for most Linux users)
        install_bash(cmd, &completions_dir);
    }
}

#[cfg(not(target_os = "windows"))]
fn install_bash(cmd: &mut clap::Command, completions_dir: &std::path::Path) {
    let bash_file = completions_dir.join("iris.bash");
    match fs::File::create(&bash_file) {
        Ok(mut file) => {
            generate(Bash, cmd, "iris", &mut file);
            println!("Installed Bash completion to: {}", bash_file.display());

            // add to .bashrc or .bash_profile
            if let Some(home) = dirs::home_dir() {
                let rc_file = if cfg!(target_os = "macos") {
                    home.join(".bash_profile")
                } else {
                    home.join(".bashrc")
                };

                append_to_rc_file(&rc_file, &format!("source \"{}\"", bash_file.display()));
            }
        }
        Err(e) => eprintln!("Failed to create completion file: {}", e),
    }
}

#[cfg(not(target_os = "windows"))]
fn install_zsh(cmd: &mut clap::Command, completions_dir: &std::path::Path) {
    let zsh_file = completions_dir.join("_iris");
    match fs::File::create(&zsh_file) {
        Ok(mut file) => {
            generate(Zsh, cmd, "iris", &mut file);
            println!("Installed Zsh completion to: {}", zsh_file.display());

            if let Some(home) = dirs::home_dir() {
                let rc_file = home.join(".zshrc");
                let line = format!(
                    "fpath=(\"{}\" $fpath)\nautoload -U compinit; compinit",
                    completions_dir.display()
                );
                append_to_rc_file(&rc_file, &line);
            }
        }
        Err(e) => eprintln!("Failed to create completion file: {}", e),
    }
}

#[cfg(not(target_os = "windows"))]
fn install_fish(cmd: &mut clap::Command) {
    // fish standard is ~/.config/fish/completions/
    if let Some(home) = dirs::home_dir() {
        let completions_dir = home.join(".config/fish/completions");
        if let Err(e) = fs::create_dir_all(&completions_dir) {
            eprintln!("Failed to create fish completions directory: {}", e);
            return;
        }

        let fish_file = completions_dir.join("iris.fish");
        match fs::File::create(&fish_file) {
            Ok(mut file) => {
                generate(Fish, cmd, "iris", &mut file);
                println!("Installed Fish completion to: {}", fish_file.display());
            }
            Err(e) => eprintln!("Failed to create completion file: {}", e),
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn append_to_rc_file(path: &std::path::Path, content: &str) {
    // check if file exists, if not modify it
    if !path.exists() {
        if let Ok(mut file) = fs::File::create(path) {
            use std::io::Write;
            if let Err(e) = writeln!(file, "\n{}", content) {
                eprintln!("Failed to write to {}: {}", path.display(), e);
            } else {
                println!(
                    "Added sourcing line to: {}",
                    path.display().to_string().as_str().cyan()
                );
            }
        }
        return;
    }

    if let Ok(existing_content) = fs::read_to_string(path) {
        // check if content already exists to avoid duplicates
        // for zsh, check if the fpath dir is already added
        // for bash, check if the source command exists
        let fpath_prefix = if content.contains("fpath") {
            content.split_once(' ').map(|(prefix, _)| prefix)
        } else {
            None
        };

        let already_exists = existing_content.contains(content)
            || fpath_prefix.map_or(false, |prefix| existing_content.contains(prefix));

        if already_exists {
            println!(
                "File {} already contains the sourcing line.",
                path.display().to_string().as_str().cyan()
            );
            return;
        }

        use std::io::Write;
        let mut file = match fs::OpenOptions::new().write(true).append(true).open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open {} for appending: {}", path.display(), e);
                return;
            }
        };

        if let Err(e) = writeln!(file, "\n{}", content) {
            eprintln!("Failed to append to {}: {}", path.display(), e);
        } else {
            println!(
                "Added sourcing line to: {}",
                path.display().to_string().as_str().cyan()
            );
        }
    }
}

fn uninstall_completion() {
    if cfg!(target_os = "windows") {
        uninstall_windows();
    } else {
        uninstall_unix();
    }
}

#[cfg(target_os = "windows")]
fn uninstall_windows() {
    let config_dir = crate::paths::config_path::get_config_dir();
    let iris_completions = config_dir.join("Completions");
    let ps_file = iris_completions.join("_iris.ps1");

    // 1. remove the completion file
    if ps_file.exists() {
        match fs::remove_file(&ps_file) {
            Ok(_) => println!("Removed completion file: {}", ps_file.display()),
            Err(e) => eprintln!("Failed to remove completion file: {}", e),
        }
    } else {
        println!("Completion file not found: {}", ps_file.display());
    }

    // 2. remove sourcing line from profiles
    let profile_paths = get_powershell_profiles();
    if !profile_paths.is_empty() {
        for profile_path in profile_paths {
            if profile_path.exists() {
                let source_line = format!(". \"{}\"", ps_file.display());

                match fs::read_to_string(&profile_path) {
                    Ok(content) => {
                        if content.contains(&source_line) {
                            let new_content: String = content
                                .lines()
                                .filter(|line| !line.contains(&source_line))
                                .collect::<Vec<_>>()
                                .join("\n");

                            match fs::write(&profile_path, new_content) {
                                Ok(_) => println!(
                                    "Removed sourcing line from PowerShell profile: {}",
                                    profile_path.display().to_string().as_str().cyan()
                                ),
                                Err(e) => eprintln!(
                                    "Failed to write to profile {}: {}",
                                    profile_path.display(),
                                    e
                                ),
                            }
                        } else {
                            println!(
                                "PowerShell profile does not contain the sourcing line: {}",
                                profile_path.display().to_string().as_str().cyan()
                            );
                        }
                    }
                    Err(e) => eprintln!("Failed to read profile {}: {}", profile_path.display(), e),
                }
            }
        }
    } else {
        println!("Could not detect any PowerShell profile paths.");
    }
}

#[cfg(not(target_os = "windows"))]
fn uninstall_windows() {}

#[cfg(not(target_os = "windows"))]
fn uninstall_unix() {
    let config_dir = crate::paths::config_path::get_config_dir();
    let completions_dir = config_dir.join("completions");

    // attempt to remove bash/zsh files
    let bash_file = completions_dir.join("iris.bash");
    if bash_file.exists() {
        let _ = fs::remove_file(&bash_file);
        println!("Removed Bash completions.");
    }
    let zsh_file = completions_dir.join("_iris");
    if zsh_file.exists() {
        let _ = fs::remove_file(&zsh_file);
        println!("Removed Zsh completions.");
    }

    // attempt to remove fish file (standard location)
    if let Some(home) = dirs::home_dir() {
        let fish_file = home.join(".config/fish/completions/iris.fish");
        if fish_file.exists() {
            let _ = fs::remove_file(&fish_file);
            println!("Removed Fish completions.");
        }

        // remove lines from RC files
        // bash
        let rc_file = if cfg!(target_os = "macos") {
            home.join(".bash_profile")
        } else {
            home.join(".bashrc")
        };
        remove_line_from_file(&rc_file, &format!("source \"{}\"", bash_file.display()));

        // zsh (fpath)
        let zshrc = home.join(".zshrc");
        let zsh_line = format!("fpath=(\"{}\" $fpath)", completions_dir.display());
        // just trying to remove the fpath addition part
        remove_line_from_file(&zshrc, &zsh_line);
    }
}

#[cfg(not(target_os = "windows"))]
fn remove_line_from_file(path: &std::path::Path, partial_content: &str) {
    if !path.exists() {
        return;
    }
    match fs::read_to_string(path) {
        Ok(content) => {
            if content.contains(partial_content) {
                let new_content: String = content
                    .lines()
                    .filter(|line| !line.contains(partial_content))
                    .collect::<Vec<_>>()
                    .join("\n");

                match fs::write(path, new_content) {
                    Ok(_) => println!(
                        "Removed sourcing line from: {}",
                        path.display().to_string().as_str().cyan()
                    ),
                    Err(e) => eprintln!("Failed to write to {}: {}", path.display(), e),
                }
            }
        }
        Err(e) => eprintln!("Failed to read {}: {}", path.display(), e),
    }
}

fn powershell_execution_policy() -> Option<String> {
    for exe in ["pwsh", "powershell"] {
        let output = Command::new(exe)
            .args([
                "-NoProfile",
                "-Command",
                "Get-ExecutionPolicy -Scope CurrentUser",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                return Some(
                    String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string(),
                );
            }
        }
    }
    None
}

fn get_powershell_profiles() -> Vec<PathBuf> {
    let mut profiles = Vec::new();

    // 1. try to get it from the PROFILE environment variable
    if let Ok(profile) = std::env::var("PROFILE") {
        let path = PathBuf::from(profile);
        if !path.as_os_str().is_empty() {
            profiles.push(path);
        }
    }

    // 2. fallback to standard locations
    if let Some(docs) = dirs::document_dir() {
        // Documents\PowerShell\Microsoft.PowerShell_profile.ps1 (Powershell Core / Modern)
        let pwsh_profile = docs
            .join("PowerShell")
            .join("Microsoft.PowerShell_profile.ps1");
        if pwsh_profile.exists() {
            profiles.push(pwsh_profile);
        }

        // Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1 (Old PowerShell)
        let ps_profile = docs
            .join("WindowsPowerShell")
            .join("Microsoft.PowerShell_profile.ps1");
        if ps_profile.exists() {
            profiles.push(ps_profile);
        }

        // if none of the above exist but we found the documents dir,
        // we can still return the modern one as a default target for new installs
        if profiles.is_empty() {
            profiles.push(
                docs.join("PowerShell")
                    .join("Microsoft.PowerShell_profile.ps1"),
            );
        }
    }

    profiles.sort();
    profiles.dedup();
    profiles
}
