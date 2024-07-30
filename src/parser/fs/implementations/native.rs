use std::fs::{self, read_dir};

use std::path::Path;

use super::super::{FileSystem, FileSystemError};

pub struct NativeFileSystem {}

impl NativeFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for NativeFileSystem {
    fn read_content(&self, path: &str) -> Result<String, FileSystemError> {
        let path = Path::new(path);
        Ok(fs::read_to_string(path)?)
    }

    fn path_is_file(&self, path: &str) -> bool {
        let path = Path::new(path);
        path.is_file()
    }

    fn path_is_directory(&self, path: &str) -> bool {
        let path = Path::new(path);
        path.is_dir()
    }

    fn get_all_cds_files_in_dir(&self, dir_path: &str) -> Result<Vec<String>, FileSystemError> {
        let path = Path::new(dir_path);
        let mut files = vec![];
        for entry in read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("cds") {
                files.push(file_path.to_string_lossy().to_string());
            }
        }
        Ok(files)
    }

    fn get_parent_dir(&self, path: &str) -> Result<String, FileSystemError> {
        let path = Path::new(path);
        path.parent()
            .map(|p| p.to_string_lossy().to_string())
            .ok_or_else(|| {
                FileSystemError::new(
                    "Unable to get parent directory".to_string()
                        + &path.to_string_lossy().to_string(),
                )
            })
    }

    fn join_paths(&self, path_a: &str, path_b: &str) -> Result<String, FileSystemError> {
        let path_a = Path::new(path_a);
        Ok(path_a.join(path_b).to_string_lossy().to_string())
    }

    fn to_absolute(&self, path: &str) -> Result<String, FileSystemError> {
        let path = Path::new(path);
        Ok(path.canonicalize()?.to_string_lossy().to_string())
    }
}
