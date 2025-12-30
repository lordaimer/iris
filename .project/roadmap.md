# Iris Feature Roadmap

## Core Architecture & Safety
*Fundamental stability and safety mechanisms.*

- [ ] **Protected Paths** `[Priority: Critical]`
    - *Goal*: Prevent Iris from sorting inside critical system/code directories.
    - [ ] Expand `PROTECTED_PATHS` list.
    - [ ] Logic to ignore sort if folder contains identifiers like `.git`, `.idea`.
- [ ] **Dry Run / Simulation** `[Priority: Critical]`
    - *Goal*: Allow users to preview actions without making changes.
    - [ ] Implement `--dry-run` flag.
    - [ ] Print "Would move X to Y" for all operations.
- [ ] **Operation Logging** `[Priority: High]`
    - *Goal*: Audit trail for user actions.
    - [ ] Implement local log file (e.g., `~/.iris/history.log`).
    - [ ] Log timestamp, source, destination, and rule used.

## Configuration System
*Settings management and flexibility.*

- [ ] **Presets Path Refactoring** `[Priority: Critical]`
    - *Goal*: Reduce `iris.toml` complexity (currently ~400 lines).
    - [ ] Implement `presets_path` option.
    - [ ] Move default presets out of `iris.toml` to external files in `presets_path`.
- [ ] **Custom Target Paths** `[Priority: Medium]`
    - [ ] Add `custom` option for target paths.
    - [ ] Allow `iris sort` to use custom target without explicit path arg.
- [ ] **Conflict Resolution Strategy** `[Priority: High]`
    - *Goal*: Customized handling of file collisions.
    - [ ] Add `on_conflict` option in config.
    - [ ] Supported modes: `skip`, `overwrite`, `rename` (append suffix).

## Automation & Workflow
*Background processes and user interaction flows.*

- [ ] **Watcher Command** `[Priority: High]`
    - *Goal*: Continuous background sorting with minimal overhead.
    - [ ] Implement `iris watch <folder>` (e.g., `iris watch ~/Downloads`).
- [ ] **Undo Command** `[Priority: Low/Future]`
    - [ ] `iris undo` to reverse last operation.

## Cleanup & Archiving
*Automated file maintenance and archiving strategies.*

- [ ] **Duplicate Detection** `[Priority: High]`
    - *Goal*: Identify and handle duplicate files.
    - [ ] Implement hash-based duplicate detection
    - [ ] Add `iris cleanup duplicates` command.
    - [ ] Config for duplicate handling modes: `delete`, `move-to-folder`, `keep-newest`, `keep-largest`, `interactive`.
    - [ ] Config for duplicate handling criteria: keep `oldest`, `newest`.
- [ ] **Old File Cleanup** `[Priority: High]`
    - *Goal*: Automatically remove or archive files based on age.
    - [ ] Add `age_threshold` config option (e.g., `30d`, `6m`, `1y`).
    - [ ] Implement `iris cleanup old` command.
    - [ ] Config for old file cleanup actions: `delete`, `archive`, `move-to-trash`.
- [ ] **Archive Command** `[Priority: Medium]`
    - *Goal*: Compress and archive old or infrequently accessed files.
    - [ ] Implement `iris archive <folder>` command.
    - [ ] Support compression formats: `.zip`, `.tar.gz`, `.tar.zst`, `.7z`.
    - [ ] Config for archive naming: timestamp-based or custom pattern.
    - [ ] Config for archive deletion: delete originals after successful archiving.
    - [ ] Config for archive date ranges (e.g., archive all files from 2023).
- [ ] **Temporary File Cleanup** `[Priority: Medium]`
    - *Goal*: Remove system and application temporary files.
    - [ ] Detect common temp file patterns (`.tmp`, `.cache`, `~$*`, etc.).
    - [ ] Add `iris cleanup temp` command.
    - [ ] Config for safe deletion with whitelist/blacklist (only delete files matching patterns in whitelist while omitting patterns in blacklist).
- [ ] **Empty Directory Removal** `[Priority: Low]`
    - *Goal*: Clean up empty folders after sorting operations.
    - [ ] Config for empty directory removal.
    - [ ] Implement `iris cleanup empty` command.
    - [ ] Recursive empty directory detection config flag.
- [ ] **Archive Rotation** `[Priority: Low/Future]`
    - *Goal*: Maintain archive history with automatic rotation.
    - [ ] Config for archive rotation interval (e.g., `7d`, `1m`, `1y`).
    - [ ] Config for archive rotation retention policy (e.g., `keep_last_n`, `keep_within_time_window`).
    - [ ] Config for automatic deletion of old archives based on retention policy.

## Sorting Logic & Filters
*Rules for how and what to sort.*

- [ ] **File Naming System** `[Priority: High]`
    - [ ] Global switch: `rename_files = true`.
    - [ ] `sanitize_names = true` (remove invalid chars).
    - [ ] `rename_method` styles: `lower`, `upper`, `title`, `camel`, `pascal`, `snake`, `kebab`.
    - [ ] `space_replacement` (optional).
- [ ] **Exclusions** `[Priority: High]`
    - [ ] `file_exclusions` list (regex support).
    - [ ] `directory_exclusions` list (regex support).
- [ ] **Filter Conditions** `[Priority: Medium]`
    - [ ] Filename patterns: `contains`, `!contains`, `regex`.
    - [ ] Size filtering: `min_file_size`, `max_file_size`.
- [ ] **Advanced Sorting Options** `[Priority: Future]`
    - [ ] Sort by: Time Created, Time Modified, File Size, File Type, File Owner.
- [ ] **Smart Media Sorting** `[Priority: Future]`
    - [ ] Integration with external APIs (MusicBrainz, TMDB) for metadata-based sorting.
