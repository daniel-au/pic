use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn _get_image_extensions() -> HashSet<String> {
    HashSet::from(["cr2", "gif", "jpg", "jpeg", "mov", "mp4", "nef", "png"].map(String::from))
}

pub fn _get_file_extension(filename: &str) -> String {
    match Path::new(filename).extension() {
        Some(ext) => ext.to_str().unwrap_or("").to_string(),
        None => String::new(),
    }
}

pub fn _get_images(img_extensions: &HashSet<String>) -> Vec<String> {
    let mut image_files: Vec<String> = Vec::new();
    let entries: fs::ReadDir = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(e) => panic!("Error: failed to read directory: {}", e),
    };
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => panic!("Error: failed to read directory entry: {}", e),
        };
        let entry_md: fs::Metadata = match entry.metadata() {
            Ok(md) => md,
            Err(e) => panic!("Error: failed to read directory entry metadata: {}", e),
        };
        if entry_md.is_dir() {
            continue;
        }
        let filename: String = entry.file_name().into_string().unwrap();
        let extension: String = _get_file_extension(&filename);
        if img_extensions.contains(&extension.to_lowercase()) {
            image_files.push(filename);
        }
    }
    image_files.sort();
    image_files
}
