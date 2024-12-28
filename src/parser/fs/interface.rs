use std::path::{Path, PathBuf};

use super::FileSystemError;

pub trait FileSystem {
    fn read_content(&self, path: &Path) -> Result<String, FileSystemError>;
    fn file_exists(&self, path: &Path) -> bool;
    fn path_is_file(&self, path: &Path) -> bool;
    fn path_is_directory(&self, path: &Path) -> bool;
    fn get_all_cds_files_in_dir(&self, dir_path: &Path) -> Result<Vec<PathBuf>, FileSystemError>;
    fn get_parent_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError>;
    fn join_paths(&self, path_a: &Path, path_b: &Path) -> Result<PathBuf, FileSystemError>;
    fn to_absolute(&self, path: &Path) -> Result<PathBuf, FileSystemError>;
}
