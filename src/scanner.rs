use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|s| s == "mkv").unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect()
}
