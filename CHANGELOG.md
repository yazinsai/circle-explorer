# Changelog

All notable changes to Circle Explorer will be documented in this file.

## [0.1.0] - 2026-02-16

### Added
- Concentric ring visualization using D3.js sunburst layout
- Click-to-navigate: click folders to drill in, click center to go up to parent
- Color-coded segments by file type (60+ extensions mapped)
- Hover details with persistent info panel (name, size, date, path)
- Double-click files to open in default OS application
- Breadcrumb navigation bar with clickable path segments
- List view toggle (sortable table with name, size, type, modified columns)
- Flat-folder grouping: directories with >20 items show top 15 + "N other items"
- Dark theme UI
- View preference saved in localStorage
- Loading spinner during directory scans
- Error states with "Go Home" recovery button
- Toast notifications for file open confirmations
- Feedback button linking to GitHub Issues
- GitHub Actions CI/CD pipeline (build + release workflows)
- Cross-platform support: macOS (primary), Linux, Windows
- Symlink-safe directory scanning (skips symlinks to avoid cycles)
- Large directory handling: caps at 500 children per directory for performance

### Known Limitations
- Read-only: no file operations (copy, move, rename, delete)
- No search functionality
- No bookmarks or favorites
- No keyboard navigation
- 2-level scan depth only
- Hidden files (dotfiles) excluded
