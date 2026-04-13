# Circle Explorer

**A file explorer that works like a map, not a list.**

[![Release](https://img.shields.io/github/v/release/yazinsai/circle-explorer?include_prereleases&sort=semver)](https://github.com/yazinsai/circle-explorer/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Website](https://img.shields.io/badge/website-circle--explorer.whhite.com-blue)](https://circle-explorer.whhite.com)

Circle Explorer uses concentric rings to visualize your file system. The center circle is your current directory, the first ring shows its children, and the second ring shows grandchildren. You see 2-3 levels of hierarchy at once, with segment sizes proportional to file/folder size.

## Download

Grab the latest signed & notarized build from the [**Releases page**](https://github.com/yazinsai/circle-explorer/releases/latest):

- **macOS (Apple Silicon)** — `Circle.Explorer_*_aarch64.dmg`
- **macOS (Intel)** — `Circle.Explorer_*_x64.dmg`
- **Linux** — `.AppImage` or `.deb`
- **Windows** — `.msi` or `.exe`

> This is an experiment. The question we're trying to answer: **is concentric circle navigation usable enough that someone would come back to it a second time?**

## Why This Exists

Every file explorer since 1984 uses lists and trees. Every sunburst/radial visualization tool (DaisyDisk, Filelight, Baobab) is a disk *analyzer*, not a file *browser*. One tool tried to bridge this gap -- Spyglass (Windows, ~2013) -- and was called "not really practicable."

Circle Explorer is a second attempt, with three specific improvements over Spyglass:

1. **Flat-folder grouping** -- Directories with >20 items show the 15 largest and collapse the rest into a single "N other items" segment. This directly addresses the #1 reason Spyglass was unusable.
2. **Smooth animations** -- 350ms transitions with cubic easing on every navigation. The "feel" is the product.
3. **List view escape hatch** -- One click to switch to a traditional sortable table. No shame.

## Features

- **Concentric ring visualization** -- D3.js sunburst with 2 rings of hierarchy
- **Click to navigate** -- Click folders to drill in, click center to go up
- **Color-coded by file type** -- 60+ extensions mapped to colors (code=green, images=blue, docs=yellow, media=purple)
- **Hover details** -- Persistent info panel shows name, size, date, path of hovered item
- **Double-click to open** -- Opens files in your default OS application
- **Breadcrumb navigation** -- Clickable path bar at the top
- **List view toggle** -- Switch to a sortable table view anytime (preference saved)
- **Flat-folder grouping** -- Handles directories with hundreds of files gracefully
- **Dark theme** -- Because it's 2026
- **Cross-platform** -- macOS, Linux, Windows (macOS-first)
- **Lightweight** -- ~8MB binary, ~3MB DMG (Tauri, not Electron)
- **Read-only** -- Cannot modify, delete, or move your files. Zero risk.

## Tech Stack

- **[Tauri v2](https://tauri.app)** -- Rust backend + web frontend, tiny binary
- **[Svelte 5](https://svelte.dev)** -- Reactive UI with runes
- **[D3.js v7](https://d3js.org)** -- Sunburst visualization
- **[SvelteKit](https://kit.svelte.dev)** -- Static adapter for SPA mode
- **TypeScript** -- Throughout the frontend

## Getting Started

**See [QUICKSTART.md](QUICKSTART.md) for a detailed step-by-step guide with troubleshooting.**

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) (latest stable) -- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [Tauri system dependencies](https://v2.tauri.app/start/prerequisites/) for your OS
  - macOS: `xcode-select --install`
  - Linux: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`

### Run It

```bash
# Install dependencies
npm install

# Run in development mode (hot reload) -- first build takes ~2-3 min
npm run tauri dev

# Build for production (.app/.dmg on macOS, .deb/.AppImage on Linux)
npm run tauri build
```

### Build Status

| Component | Status |
|-----------|--------|
| Frontend (Svelte + D3.js) | **Builds successfully** -- 0 errors, 0 warnings |
| Backend (Rust + Tauri) | **Compiles cleanly** -- 0 warnings |
| Production build | **Produces .app + .dmg** on macOS (8MB binary, 3MB DMG) |
| TypeScript checks | **Passes** -- `npm run check` clean |

### Project Structure

```
circle-explorer/
  src/                    # Svelte frontend
    lib/
      components/
        Sunburst.svelte   # D3 concentric ring visualization
        ListView.svelte   # Table view (escape hatch)
        Breadcrumb.svelte # Clickable path navigation
        InfoPanel.svelte  # Hover/selection detail panel
        Toast.svelte      # Notification component
      colors.ts           # 60+ file type color mappings
      types.ts            # TypeScript interfaces
      utils.ts            # Formatting helpers
    routes/
      +page.svelte        # Main app shell
  src-tauri/              # Rust backend
    src/
      lib.rs              # Directory scanner + file opener commands
      main.rs             # Tauri entry point
```

## Known Limitations

- **Read-only** -- No file operations (copy, move, rename, delete). Intentional for v1.
- **No search** -- Navigate visually or switch to list view.
- **No bookmarks** -- No favorites or pinned folders yet.
- **No keyboard navigation** -- Mouse/trackpad only for now.
- **2-level scan depth** -- Only shows children and grandchildren to keep scans fast.
- **Hidden files excluded** -- Dotfiles are not shown (design choice for cleaner display).

## License

MIT
