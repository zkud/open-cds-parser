use std::fs::{self, read_dir};

use std::path::{Path, PathBuf};

use super::super::{FileSystem, FileSystemError};

pub struct NativeFileSystem {}

impl NativeFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for NativeFileSystem {
    fn read_content(&self, path: &Path) -> Result<String, FileSystemError> {
        Ok(fs::read_to_string(path)?)
    }

    fn path_is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn path_is_directory(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn get_all_cds_files_in_dir(&self, path: &Path) -> Result<Vec<PathBuf>, FileSystemError> {
        let mut files = vec![];
        for entry in read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("cds") {
                files.push(file_path);
            }
        }
        Ok(files)
    }

    fn get_parent_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        path.parent().map(|p| p.to_path_buf()).ok_or_else(|| {
            FileSystemError::new(
                "Unable to get parent directory".to_string() + &path.to_string_lossy().to_string(),
            )
        })
    }

    fn join_paths(&self, path_a: &Path, path_b: &Path) -> Result<PathBuf, FileSystemError> {
        Ok(path_a.join(path_b))
    }

    fn to_absolute(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        Ok(path.canonicalize()?)
    }

    fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }
}
