use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

/// Maximum number of direct children to include in a directory's children vec.
/// Directories with more entries will be truncated and `truncated` set to true.
const MAX_CHILDREN: usize = 500;

/// When at max scan depth, if a directory has more than this many entries,
/// we estimate total size from a sample rather than iterating every entry.
const MAX_DEPTH_ENTRY_LIMIT: usize = 5000;

/// Number of entries to sample when estimating size for very large directories.
const SIZE_SAMPLE_COUNT: usize = 100;

#[derive(Debug, Serialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: Option<u64>,
    pub extension: Option<String>,
    pub children: Vec<FileEntry>,
    /// The actual number of direct children in this directory (before truncation).
    /// For files, this is always 0.
    pub child_count: u32,
    /// Whether the children list was capped at MAX_CHILDREN.
    /// If true, the frontend should indicate that not all items are shown.
    pub truncated: bool,
}

/// Scan a directory to the given depth, returning a tree of FileEntry nodes.
/// Handles permission errors gracefully by skipping unreadable entries.
/// Uses symlink_metadata to avoid following symlinks and prevent infinite loops.
fn scan_dir(path: &PathBuf, current_depth: u32, max_depth: u32) -> Option<FileEntry> {
    // Use symlink_metadata to avoid following symlinks (prevents infinite loops)
    let metadata = fs::symlink_metadata(path).ok()?;
    let file_type = metadata.file_type();

    // Skip symlinks entirely to avoid cycles
    if file_type.is_symlink() {
        return None;
    }

    let is_dir = file_type.is_dir();

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    let extension = if !is_dir {
        path.extension()
            .map(|e| e.to_string_lossy().to_string().to_lowercase())
    } else {
        None
    };

    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());

    if !is_dir {
        return Some(FileEntry {
            name,
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            is_dir: false,
            modified,
            extension,
            children: vec![],
            child_count: 0,
            truncated: false,
        });
    }

    // For directories, scan children if we haven't exceeded depth
    let mut children = Vec::new();
    let mut total_size: u64 = 0;
    let mut actual_child_count: u32 = 0;

    if current_depth < max_depth {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let child_path = entry.path();

                // Skip hidden files/dirs (starting with .) for cleaner display
                if let Some(fname) = child_path.file_name() {
                    if fname.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }

                // Skip symlinks at the child level too
                if let Ok(child_meta) = fs::symlink_metadata(&child_path) {
                    if child_meta.file_type().is_symlink() {
                        continue;
                    }
                }

                if let Some(child) = scan_dir(&child_path, current_depth + 1, max_depth) {
                    total_size += child.size;
                    actual_child_count += 1;
                    children.push(child);
                }
            }
        }

        // Sort children: directories first, then by size descending
        children.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(b.size.cmp(&a.size)));

        // Cap children at MAX_CHILDREN
        let truncated = children.len() > MAX_CHILDREN;
        if truncated {
            children.truncate(MAX_CHILDREN);
        }

        Some(FileEntry {
            name,
            path: path.to_string_lossy().to_string(),
            size: total_size,
            is_dir: true,
            modified,
            extension: None,
            children,
            child_count: actual_child_count,
            truncated,
        })
    } else {
        // At max depth: calculate size without recursing into children.
        // For very large directories (>5000 entries), estimate size from the
        // first 100 entries rather than reading metadata for every file.
        if let Ok(entries) = fs::read_dir(path) {
            let mut entry_count: usize = 0;
            let mut sampled_size: u64 = 0;
            let mut sampled_count: usize = 0;
            let mut exact_size: u64 = 0;
            let mut exceeded_limit = false;

            for entry in entries.flatten() {
                entry_count += 1;

                if entry_count <= SIZE_SAMPLE_COUNT {
                    // First N entries: read metadata for the size sample
                    if let Ok(meta) = entry.path().symlink_metadata() {
                        if !meta.file_type().is_symlink() {
                            sampled_size += meta.len();
                            sampled_count += 1;
                        }
                    }
                } else if entry_count <= MAX_DEPTH_ENTRY_LIMIT {
                    // Beyond sample but within limit: still read metadata for exact size
                    if let Ok(meta) = entry.path().symlink_metadata() {
                        if !meta.file_type().is_symlink() {
                            exact_size += meta.len();
                        }
                    }
                } else {
                    // Exceeded the entry limit -- stop reading metadata entirely.
                    // We will estimate the size from the sample instead.
                    exceeded_limit = true;
                    break;
                }
            }

            if exceeded_limit {
                // We broke out early. Do a second lightweight pass just to count
                // remaining entries (no metadata reads, just directory listing).
                if let Ok(remaining_entries) = fs::read_dir(path) {
                    let full_count = remaining_entries.into_iter().filter_map(|e| e.ok()).count();
                    entry_count = full_count;
                }
                // Estimate total size from sample average
                if sampled_count > 0 {
                    let avg = sampled_size / sampled_count as u64;
                    total_size = avg * entry_count as u64;
                }
            } else {
                // We read all entries exactly
                total_size = sampled_size + exact_size;
            }

            actual_child_count = entry_count as u32;
        }

        Some(FileEntry {
            name,
            path: path.to_string_lossy().to_string(),
            size: total_size,
            is_dir: true,
            modified,
            extension: None,
            children: vec![],
            child_count: actual_child_count,
            truncated: false,
        })
    }
}

#[tauri::command]
fn scan_directory(path: Option<String>) -> Result<FileEntry, String> {
    let dir_path = match path {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
    };

    if !dir_path.exists() {
        return Err(format!("Path does not exist: {}", dir_path.display()));
    }

    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", dir_path.display()));
    }

    // Scan 2 levels deep (current + children + grandchildren)
    scan_dir(&dir_path, 0, 2).ok_or_else(|| "Failed to scan directory".to_string())
}

#[tauri::command]
fn get_home_dir() -> Result<String, String> {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not determine home directory".to_string())
}

#[tauri::command]
fn get_parent_dir(path: String) -> Result<String, String> {
    let p = PathBuf::from(&path);
    p.parent()
        .map(|parent| parent.to_string_lossy().to_string())
        .ok_or_else(|| "No parent directory".to_string())
}

#[tauri::command]
fn open_file(path: String) -> Result<String, String> {
    let file_path = PathBuf::from(&path);

    if !file_path.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    // Use the OS default application to open the file
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    Ok(format!("Opened: {}", path))
}

#[tauri::command]
fn reveal_in_finder(path: String) -> Result<String, String> {
    let file_path = PathBuf::from(&path);

    if !file_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // If it's a file, reveal it in its parent folder; if it's a directory, open it
    #[cfg(target_os = "macos")]
    {
        if file_path.is_dir() {
            Command::new("open")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to reveal in Finder: {}", e))?;
        } else {
            Command::new("open")
                .arg("-R")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to reveal in Finder: {}", e))?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        let dir = if file_path.is_dir() {
            path.clone()
        } else {
            file_path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or(path.clone())
        };
        Command::new("xdg-open")
            .arg(&dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        if file_path.is_dir() {
            Command::new("explorer")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        } else {
            Command::new("explorer")
                .args(["/select,", &path])
                .spawn()
                .map_err(|e| format!("Failed to reveal in Explorer: {}", e))?;
        }
    }

    Ok(format!("Revealed: {}", path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_directory,
            get_home_dir,
            get_parent_dir,
            open_file,
            reveal_in_finder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
