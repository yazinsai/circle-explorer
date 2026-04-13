# Contributing to Circle Explorer

Thanks for your interest in contributing! This is an early-stage experiment to see if concentric circle navigation can work as a daily file browser.

## How to Contribute

### Reporting Issues

- **Bugs**: Use the [Bug Report template](https://github.com/circle-explorer/circle-explorer/issues/new?template=bug_report.yml)
- **UX Feedback**: Use the [Feedback template](https://github.com/circle-explorer/circle-explorer/issues/new?template=feedback.yml) — this is the most valuable contribution right now
- **Feature Requests**: Use the [Feature Request template](https://github.com/circle-explorer/circle-explorer/issues/new?template=feature_request.yml)

### Code Contributions

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-improvement`)
3. Make your changes
4. Test locally with `npm run tauri dev`
5. Ensure frontend builds cleanly: `npm run build`
6. Ensure Rust checks pass: `cd src-tauri && cargo check && cargo clippy`
7. Submit a pull request

### Development Setup

**Prerequisites:**
- Node.js >= 18
- Rust (latest stable)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS

```bash
# Clone and install
git clone https://github.com/circle-explorer/circle-explorer.git
cd circle-explorer
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure

```
src/                     # Svelte frontend
  lib/
    components/
      Sunburst.svelte    # D3 concentric ring visualization (the core)
      ListView.svelte    # Table view (escape hatch)
      Breadcrumb.svelte  # Clickable path navigation
      InfoPanel.svelte   # Hover/selection detail panel
      FeedbackButton.svelte  # Feedback & GitHub links
      Toast.svelte       # Notification component
    colors.ts            # File type color mappings
    types.ts             # TypeScript interfaces
    utils.ts             # Formatting helpers
  routes/
    +page.svelte         # Main app shell
src-tauri/               # Rust backend
  src/
    lib.rs               # Directory scanner + file opener commands
    main.rs              # Tauri entry point
```

### What We're Looking For

Priority contributions right now:

1. **UX improvements to the circle view** — smoother animations, better hover states, more intuitive navigation
2. **Performance optimizations** — especially for large directories (1000+ files)
3. **Accessibility** — keyboard navigation, screen reader support
4. **Bug fixes** — edge cases with different file systems, permissions, symlinks

### What We're NOT Adding Yet

Per the [strategy](strategy.md), these are intentionally excluded from v1:

- File operations (copy, move, rename, delete)
- Search / find
- Bookmarks / favorites
- Custom themes
- Mobile / touch support

If you want to work on these, please open an issue first to discuss.

## Code Style

- **Rust**: Follow `cargo clippy` recommendations
- **TypeScript/Svelte**: Use Svelte 5 runes (`$state`, `$derived`, `$effect`)
- **CSS**: Dark theme, use existing color variables from the codebase

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
