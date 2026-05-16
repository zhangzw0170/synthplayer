use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "wav", "ogg", "m4a", "aac", "wma", "ape", "alac", "mp4",
];

pub fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn scan_directory(root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_audio_file(entry.path()))
        .map(|entry| entry.into_path())
        .collect();

    files.sort();
    files
}
