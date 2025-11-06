// TODO: Implement dry-run (simulation) before actual file operations. provide support for  --dry-run (alias = [ --sim | --simulate ]) flag
// TODO: Implement undo feature as a safe revert option incase something which should'nt be sorted gets sorted
// TODO: A global "*" catch-all extension support to sort files which don't match any presets into a misc folder
// TODO: Break this into smaller modules and functions and make this the entry point for the sort command
// TODO: Write unit tests for this module
// TODO: Skip hashing for small files (less than 1MB) or when rename succeeds
// TODO: in the copy fallback add a fast-verify mode: first compare size; if equal then hash only a small head + tail chunk for large files. then full hash it if that mismatches
// TODO: add a concurrency limit (thread pool size) configurable through config file as well as cli override
use std::path::Path;
use std::{fs, io};
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};
use colored::Colorize;

// directory traversal
use walkdir::WalkDir;
// parallel moves
use rayon::prelude::*;

use crate::config::config_processor::{IrisConfig, Mode, PresetConfig};
use crate::core::resolver::dest_base_resolver;

#[cfg(target_os = "windows")]
const PROTECTED_PATHS: &[&str] = &[
    "C:\\",
    "C:\\Windows",
    "C:\\Program Files",
    "C:\\Program Files (x86)",
    "C:\\Users",
    "C:\\Users\\Administrator",
    "C:\\ProgramData",
    "C:\\System32",
    "C:\\Windows\\System32",
    "C:\\Recovery",
    "C:\\PerfLogs",
];

#[cfg(target_os = "linux")]
const PROTECTED_PATHS: &[&str] = &[
    "/",
    "/bin",
    "/boot",
    "/dev",
    "/etc",
    "/lib",
    "/lib32",
    "/lib64",
    "/libx32",
    "/media",
    "/mnt",
    "/opt",
    "/proc",
    "/root",
    "/run",
    "/sbin",
    "/srv",
    "/sys",
    "/usr",
    "/var",
];

#[cfg(target_os = "macos")]
const PROTECTED_PATHS: &[&str] = &[
    "/",
    "/System",
    "/bin",
    "/sbin",
    "/usr",
    "/private",
    "/var",
    "/etc",
    "/dev",
    "/Applications",
    "/Library",
];

/// Move files from target -> preset-driven destinations safely
pub fn sort(target: &Path, config: &IrisConfig) -> Result<(), Box<dyn std::error::Error>> {
    // target is assumed cleaned/canonicalized by resolver
    let target = target.to_path_buf();

    // Fail-safe: only block if target exactly matches a protected system path
    for p in PROTECTED_PATHS {
        let protected = Path::new(p);
        if target == protected {
            return Err(format!(
                "Operation aborted. '{}' is a protected system path.",
                target.display()
            ).into());
        }
    }

    println!("Sorting files in: {}", target.display());

    let mode: &Mode = &config.general.mode;

    // Pre-build a HashMap for efficient extension-to-preset lookups.
    // The first preset encountered for a given extension takes precedence.
    let mut ext_map: HashMap<String, &PresetConfig> = HashMap::new();
    for preset in config.presets.iter().filter(|p| p.enabled) {
        for ext in &preset.extension {
            ext_map.entry(ext.to_lowercase()).or_insert(preset);
        }
    }

    // Phase 1: walk and plan moves (deterministic, single-threaded)
    let mut planned_moves: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut reserved_dests: HashSet<PathBuf> = HashSet::new();

    for entry in WalkDir::new(target.clone()).min_depth(1).max_depth(1) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error walking directory: {}", e);
                continue;
            }
        };

        // Skip directories. this should later be handled by the recursive flag in the config file
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        // Get the extension of the file (lowercased)
        let extension = match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => ext.to_lowercase(),
            None => continue, // Skip files without an extension
        };

        // Get the preset for the extension
        if let Some(preset) = ext_map.get(&extension) {
            // Resolve the destination base path for the preset
            match dest_base_resolver::get_dest_base(&target, preset, mode.clone()) {
                Ok(dest_base) => {
                    // Guard: destination base should not be a dangerous system path
                    if PROTECTED_PATHS.iter().any(|p| Path::new(p) == dest_base) {
                        eprintln!(
                            "Refusing to sort into protected path: {}",
                            dest_base.display()
                        );
                        continue;
                    }

                    // Compute destination file path with basic collision handling
                    let file_name = match path.file_name() {
                        Some(n) => n.to_owned(),
                        None => continue,
                    };
                    let desired = dest_base.join(file_name);
                    let dest_path = reserve_unique_destination(&desired, &mut reserved_dests);

                    // If source and destination are identical, skip
                    if path == dest_path {
                        continue;
                    }

                    planned_moves.push((path.to_path_buf(), dest_path));
                }
                Err(e) => eprintln!("Could not determine sort destination for '{}': {}", path.display(), e),
            }
        }
    }

    // Phase 2: pre-create all destination directories (deduped with HashSet)
    let mut unique_dirs: HashSet<PathBuf> = HashSet::new();
    for (_, dst) in &planned_moves {
        if let Some(parent) = dst.parent() { unique_dirs.insert(parent.to_path_buf()); }
    }
    // Create all the destination directories
    for dir in unique_dirs {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create destination directory '{}': {}", dir.display(), e);
        }
    }

    // Phase 3: execute moves in parallel using rayon and collect results
    let move_results: Vec<(PathBuf, PathBuf, Result<(), String>)> = planned_moves
        .par_iter()
        .map(|(src, dst)| {
            let result = safe_move(src, dst);
            (src.to_path_buf(), dst.to_path_buf(), result)
        })
        .collect();

    // Phase 4: group and display results by destination
    let mut successful_moves: HashMap<PathBuf, Vec<(PathBuf, PathBuf)>> = HashMap::new();
    let mut failed_moves: Vec<(PathBuf, PathBuf, String)> = Vec::new();
    let mut total_moved = 0;

    // Group successful moves by destination directory
    for (src, dst, result) in move_results {
        match result {
            Ok(()) => {
                let dest_dir = dst.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
                successful_moves
                    .entry(dest_dir)
                    .or_insert_with(Vec::new)
                    .push((src, dst));
                total_moved += 1;
            }
            Err(e) => {
                failed_moves.push((src, dst, e));
            }
        }
    }

    // Display grouped successful moves
    if !successful_moves.is_empty() {
        let mut dest_dirs: Vec<_> = successful_moves.keys().collect();
        dest_dirs.sort();

        for dest_dir in dest_dirs {
            let files = &successful_moves[dest_dir];
            println!("{}", format!("  → {}", dest_dir.display()).bright_cyan());
            
            for (src, _) in files {
                if let Some(file_name) = src.file_name() {
                    println!("{}", format!("    {}", file_name.to_string_lossy()).white());
                }
            }
            println!();
        }
    }

    // Display failed moves
    if !failed_moves.is_empty() {
        for (src, dst, err) in &failed_moves {
            eprintln!("{}", format!("Failed to move '{}' -> '{}': {}", src.display(), dst.display(), err).red());
        }
    }

    // Display summary
    if total_moved > 0 {
        println!("{}", format!("Summary: {} file{} moved", total_moved, if total_moved == 1 { "" } else { "s" }).green());
    }

    Ok(())
}

/// Reserve a unique destination path.
/// If `desired` already exists on disk or has been reserved in this run,
/// generate a hyphenated numeric suffix before the extension (file-1.txt, file-2.txt, ...)
/// and return the first available path while recording it in `reserved`.
fn reserve_unique_destination(desired: &Path, reserved: &mut HashSet<PathBuf>) -> PathBuf {
    if !desired.exists() && !reserved.contains(desired) {
        reserved.insert(desired.to_path_buf());
        return desired.to_path_buf();
    }
    let parent = desired.parent().unwrap_or_else(|| Path::new("."));
    let stem = desired
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = desired.extension().and_then(|s| s.to_str());
    for i in 1.. {
        let candidate = match ext {
            Some(e) if !e.is_empty() => parent.join(format!("{}-{}.{}", stem, i, e)),
            _ => parent.join(format!("{}-{}", stem, i)),
        };
        if !candidate.exists() && !reserved.contains(&candidate) {
            reserved.insert(candidate.clone());
            return candidate;
        }
        if i == usize::MAX { break; }
    }
    // Fallback to desired; record it
    reserved.insert(desired.to_path_buf());
    desired.to_path_buf()
}

/// Safely move the source file to the destination file
fn safe_move(src: &Path, dst: &Path) -> Result<(), String> {
    // Try fast path: rename (atomic, same volume)
    match fs::rename(src, dst) {
        Ok(_) => return Ok(()),
        Err(e) => {
            // If not a cross-device error, still attempt copy fallback; log the rename failure
            let _ = e;
        }
    }

    // Fallback: copy + verify + delete
    copy_verify_delete(src, dst)
}

/// Copy the source file to the destination file, verify the size and hash, and delete the source file
fn copy_verify_delete(src: &Path, dst: &Path) -> Result<(), String> {
    // Perform copy
    fs::copy(src, dst)
        .map_err(|e| format!("copy failed: {}", e))?;

    // Verify size first (quick check)
    let src_meta = fs::metadata(src).map_err(|e| format!("stat src failed: {}", e))?;
    let dst_meta = fs::metadata(dst).map_err(|e| format!("stat dst failed: {}", e))?;
    if src_meta.len() != dst_meta.len() {
        let _ = fs::remove_file(dst);
        return Err("size mismatch after copy".into());
    }

    // Verify hash (blake3)
    let src_hash = hash_file(src).map_err(|e| format!("hash src failed: {}", e))?;
    let dst_hash = hash_file(dst).map_err(|e| format!("hash dst failed: {}", e))?;

    if src_hash != dst_hash {
        let _ = fs::remove_file(dst);
        return Err("hash mismatch after copy".into());
    }

    // Delete source only after successful verification
    fs::remove_file(src).map_err(|e| format!("remove src failed: {}", e))?;
    Ok(())
}

/// Hash the file using blake3
fn hash_file(path: &Path) -> io::Result<blake3::Hash> {
    use std::io::{Read, BufReader};
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; 1024 * 1024];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize())
}