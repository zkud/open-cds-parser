use std::collections::HashMap;

use super::super::{FileSystem, FileSystemError};

pub struct MockInMemoryFileSystem {
    directories: HashMap<String, Vec<String>>,
    files: HashMap<String, String>,
}

impl MockInMemoryFileSystem {
    pub fn new(directories: HashMap<String, Vec<String>>, files: HashMap<String, String>) -> Self {
        Self { directories, files }
    }
}

impl FileSystem for MockInMemoryFileSystem {
    fn read_content(&self, path: &str) -> Result<String, FileSystemError> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| FileSystemError::new(format!("{} not found", path)))
    }

    fn path_is_file(&self, path: &str) -> bool {
        !self.path_is_directory(path)
    }

    fn path_is_directory(&self, path: &str) -> bool {
        path.ends_with('/')
    }

    fn get_all_cds_files_in_dir(&self, dir_path: &str) -> Result<Vec<String>, FileSystemError> {
        self.directories
            .get(dir_path)
            .cloned()
            .ok_or_else(|| FileSystemError::new(format!("{} not found", dir_path)))
    }

    fn get_parent_dir(&self, path: &str) -> Result<String, FileSystemError> {
        // don't care for performance in mocks
        self.directories
            .iter()
            .find(|(_, entries)| entries.iter().find(|e| e == &path).is_some())
            .map(|entry| entry.0.clone())
            .ok_or_else(|| FileSystemError::new(format!("cannot find parent dir for {}", path)))
    }

    fn join_paths(&self, _: &str, path_b: &str) -> Result<String, FileSystemError> {
        Ok(path_b.to_owned()) // paths always absolute
    }

    fn to_absolute(&self, path: &str) -> Result<String, FileSystemError> {
        Ok(path.to_owned()) // paths always absolute
    }
}
