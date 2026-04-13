# Quick Start Guide

Get Circle Explorer running on your machine in under 5 minutes.

---

## Option A: Run the Pre-Built App (macOS ARM only)

If you already have the `.dmg` file:

1. Open `Circle Explorer_0.1.0_aarch64.dmg`
2. Drag **Circle Explorer** to your Applications folder
3. Double-click to launch

> **Note:** macOS may block the app because it's unsigned. Right-click the app → "Open" → click "Open" in the dialog to bypass Gatekeeper.

---

## Option B: Build from Source

### 1. Install Prerequisites

You need three things: **Node.js**, **Rust**, and **Tauri system dependencies**.

**Node.js** (>= 18):
```bash
# Check if installed
node -v

# If not: https://nodejs.org/ or use nvm:
# curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
# nvm install 22
```

**Rust** (latest stable):
```bash
# Check if installed
rustc --version

# If not:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

**Tauri system dependencies:**

- **macOS:** Xcode Command Line Tools (you probably already have them)
  ```bash
  xcode-select --install
  ```
- **Linux (Debian/Ubuntu):**
  ```bash
  sudo apt update
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
  ```
- **Windows:** [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 10+)

Full details: https://v2.tauri.app/start/prerequisites/

### 2. Clone and Install

```bash
cd circle-explorer
npm install
```

### 3. Run in Development Mode (recommended for trying it out)

```bash
npm run tauri dev
```

This launches the app with **hot reload** — edit the frontend code and see changes instantly.

- First run will compile the Rust backend (~2-3 minutes)
- Subsequent runs start in ~5 seconds

### 4. Build for Production

```bash
npm run tauri build
```

This creates:
- **macOS:** `.app` bundle + `.dmg` installer in `src-tauri/target/release/bundle/`
- **Linux:** `.deb` + `.AppImage` in `src-tauri/target/release/bundle/`
- **Windows:** `.msi` + `.exe` installer in `src-tauri/target/release/bundle/`

---

## Using the App

- **Center circle** = your current directory. Click it to go up to the parent.
- **First ring** = direct children. Click a folder to navigate into it.
- **Second ring** = grandchildren (2 levels deep).
- **Hover** over any segment to see details in the bottom panel.
- **Double-click** a file to open it with your default app.
- **Toggle button** (top-right) switches between circle view and list view.
- **Breadcrumbs** (top bar) let you jump to any parent directory.
- Segments are **color-coded by file type** and **sized by file/folder size**.

---

## Troubleshooting

**"npm run tauri dev" fails immediately:**
- Make sure Rust is installed: `rustc --version`
- Make sure Node deps are installed: `npm install`
- On macOS, ensure Xcode CLI tools: `xcode-select --install`

**Compilation is slow:**
- First build downloads and compiles all Rust dependencies (~2-3 min). This is normal.
- Subsequent builds use the cache and are much faster.

**App won't open on macOS (Gatekeeper):**
- Right-click the `.app` → "Open" → click "Open" in the dialog
- Or: `xattr -cr "/path/to/Circle Explorer.app"`

**Blank window:**
- Check the terminal for errors. Common cause: the frontend build failed.
- Run `npm run build` separately to check for frontend errors.
