use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::super::{FileSystem, FileSystemError};

pub struct MockInMemoryFileSystem {
    directories: HashMap<PathBuf, Vec<PathBuf>>,
    files: HashMap<PathBuf, String>,
}

impl MockInMemoryFileSystem {
    pub fn new(
        directories: HashMap<PathBuf, Vec<PathBuf>>,
        files: HashMap<PathBuf, String>,
    ) -> Self {
        Self { directories, files }
    }
}

impl FileSystem for MockInMemoryFileSystem {
    fn read_content(&self, path: &Path) -> Result<String, FileSystemError> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| FileSystemError::new(format!("{} not found", path.to_string_lossy())))
    }

    fn path_is_file(&self, path: &Path) -> bool {
        !self.path_is_directory(path)
    }

    fn path_is_directory(&self, path: &Path) -> bool {
        path.ends_with("/")
    }

    fn get_all_cds_files_in_dir(&self, dir_path: &Path) -> Result<Vec<PathBuf>, FileSystemError> {
        self.directories.get(dir_path).cloned().ok_or_else(|| {
            FileSystemError::new(format!("{} not found", dir_path.to_string_lossy()))
        })
    }

    fn get_parent_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        // don't care for performance in mocks
        self.directories
            .iter()
            .find(|(_, entries)| entries.iter().find(|e| e == &path).is_some())
            .map(|entry| entry.0.clone())
            .ok_or_else(|| {
                FileSystemError::new(format!(
                    "cannot find parent dir for {}",
                    path.to_string_lossy()
                ))
            })
    }

    fn join_paths(&self, _: &Path, path_b: &Path) -> Result<PathBuf, FileSystemError> {
        Ok(path_b.to_path_buf()) // paths always absolute
    }

    fn to_absolute(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        Ok(path.to_path_buf()) // paths always absolute
    }

    fn file_exists(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }
}
