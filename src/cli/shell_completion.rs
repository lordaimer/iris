// TODO: implement auto-install & auto-uninstall for other OSes. currently only supports Windows
use super::cli_parser::{Cli, ShellCompletionAction};
use clap::CommandFactory;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_complete::{generate, Generator};
use colored::Colorize;
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
        println!(
            "Automatic installation is currently only supported on Windows.\n
            Use 'iris completion <SHELL> > <FILE>' to generate the script manually."
        );
    }
}

#[cfg(target_os = "windows")]
fn install_windows(cmd: &mut clap::Command) {
    let shell = std::env::var("SHELL").unwrap_or_default();
    // check if running in git bash
    let is_git_bash = shell.ends_with("/bin/bash");

    // check if running in powershell
    let is_powershell = std::env::var("PSModulePath").is_ok();

    if is_powershell && !is_git_bash {
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
        } else {
            println!("Could not detect any PowerShell profile.");
        }
    } else if is_git_bash {
        if let Some(home) = dirs::home_dir() {
            let _bashrc = home.join(".bashrc");
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

#[cfg(not(target_os = "windows"))]
fn install_windows(_cmd: &mut clap::Command) {}

fn uninstall_completion() {
    if cfg!(target_os = "windows") {
        uninstall_windows();
    } else {
        println!("Automatic uninstallation is currently only supported on Windows.");
        println!("Please remove the completion script and sourcing line manually.");
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

        // If none of the above exist but we found the documents dir,
        // we can still return the modern one as a default target for new installs
        if profiles.is_empty() {
            profiles.push(
                docs.join("PowerShell")
                    .join("Microsoft.PowerShell_profile.ps1"),
            );
        }
    }

    profiles
}
