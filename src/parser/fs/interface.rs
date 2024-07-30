use super::FileSystemError;

pub trait FileSystem {
    fn read_content(&self, path: &str) -> Result<String, FileSystemError>;
    fn path_is_file(&self, path: &str) -> bool;
    fn path_is_directory(&self, path: &str) -> bool;
    fn get_all_cds_files_in_dir(&self, dir_path: &str) -> Result<Vec<String>, FileSystemError>;
    fn get_parent_dir(&self, path: &str) -> Result<String, FileSystemError>;
    fn join_paths(&self, path_a: &str, path_b: &str) -> Result<String, FileSystemError>;
    fn to_absolute(&self, path: &str) -> Result<String, FileSystemError>;
}
