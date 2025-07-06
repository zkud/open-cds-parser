use crate::ast::*;
use std::path::PathBuf;

#[inline]
pub fn get_file_1_path() -> PathBuf {
    PathBuf::from("/file1.cds")
}

#[inline]
pub fn get_file_2_path() -> PathBuf {
    PathBuf::from("/file2.cds")
}

#[inline]
pub fn get_subdir_file_3_path() -> PathBuf {
    PathBuf::from("/subdir/file3.cds")
}

#[inline]
pub fn get_subdir_file_4_path() -> PathBuf {
    PathBuf::from("/subdir/file4.cds")
}

#[inline]
pub fn get_subdir_subdir_index_path() -> PathBuf {
    PathBuf::from("/subdir/subdir/index.cds")
}

#[inline]
pub fn get_failure_no_file_present_path() -> PathBuf {
    PathBuf::from("/failure_no_file_present.cds")
}

#[inline]
pub fn get_mock_location() -> Location {
    Location::new(0, 0, &PathBuf::new())
}
