use crate::ast::*;
use std::path::PathBuf;

#[inline]
pub fn get_file_1_path() -> PathBuf {
    PathBuf::from("/file1.cds")
}

#[inline]
pub fn get_mock_location() -> Location {
    Location::new(0, 0, &PathBuf::new())
}
