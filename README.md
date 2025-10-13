# Iris

A fast, minimal file organizer built with Rust.  
Sort, clean, watch, and archive your files with simple commands.

## Overview

Iris is a command-line tool that organizes your files automatically.  
It sorts folders by file type or time based sort options, removes empty files or directories, archives old data, and can run in the background to keep everything tidy in real time.

### Iris in Action

#### Before
```
/home/user/Downloads/
├── img_20230515_140322.jpg
├── vacation_plan.docx
├── budget2023.xlsx
├── meeting_notes.txt
├── recipe_chocolate_cake.pdf
├── movie_night.mp4
├── movie (1).mp4
├── Something in The Way.mp3
├── installer.exe
├── app-release.apk
└── project_notes/
    └── notes.docx

```
#### After

```
/home/user/
├── Documents/
│   ├── excel/
│   │   └── budget2023.xlsx
│   ├── word/
│   │   └── vacation_plan.docx
│   ├── pdf/
│   │   └── recipe_chocolate_cake.pdf
│   └── other/
│       └── meeting_notes.txt
├── Pictures/
│   └── img_20230515_140322.jpg
├── Videos/
│   └── movie_night.mp4
├── Media/
│   ├── Music/
│   │   └── Nirvana/
│   │       └── Something.In.The.Way.mp3
│   └── Movies/
│       └── Iron.Man.2008/
│           └── Iron.Man.2008.mp4
├── Applications/
│   ├── windows/
│   │   └── installer.exe
│   └── android/
│       └── app-release.apk
└── Downloads/
    └── project_notes/
        └── notes.docx

14 directories, 11 files
```

### Core Features

- **Smart Sorting** — Organize images, videos, documents, movies, music, and more.  
- **Watchdog** — Monitor folders and auto-sort new files.  
- **Cleanup** — Remove empty files or folders, clean up temp files and system cache.  
- **Archive** — Move old files into an archive folder.  
- **Undo / Redo** — Roll back or reapply sort operations.  
- **Configurable** — Simple config file for full control.  
- **Dry Run** — Preview actions before applying changes.  
- **Fast & Lightweight** — Built in Rust for performance.

> Warning
> Do not run this in a codebase! obviously it will mess it up. this is primarily intended for keeping your huge downloads folder organized
