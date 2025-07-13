use super::super::*;
use crate::ast::*;
use crate::parser::error::ParseError;
use crate::parser::single_module::SingleModuleParser;
use crate::parser::ErrorCode;
use crate::parser::FileSystem;
use std::path::{Path, PathBuf};
use std::sync::Arc;

struct MockSingleModuleParser;

impl SingleModuleParser for MockSingleModuleParser {
    fn parse(&self, _: &Path) -> Result<Box<ModuleTerm>, ParseError> {
        panic!("The method is not expected to be called");
    }
}

struct MockSymlinkFileSystem;

impl FileSystem for MockSymlinkFileSystem {
    fn read_content(&self, _: &Path) -> Result<String, crate::parser::FileSystemError> {
        todo!()
    }

    fn file_exists(&self, _: &Path) -> bool {
        todo!()
    }

    fn path_is_file(&self, _: &Path) -> bool {
        false
    }

    fn path_is_directory(&self, _: &Path) -> bool {
        false
    }

    fn get_all_cds_files_in_dir(
        &self,
        _: &Path,
    ) -> Result<Vec<PathBuf>, crate::parser::FileSystemError> {
        todo!()
    }

    fn get_parent_dir(&self, _: &Path) -> Result<PathBuf, crate::parser::FileSystemError> {
        todo!()
    }

    fn join_paths(&self, _: &Path, _: &Path) -> Result<PathBuf, crate::parser::FileSystemError> {
        todo!()
    }

    fn to_absolute(&self, _: &Path) -> Result<PathBuf, crate::parser::FileSystemError> {
        todo!()
    }
}

#[test]
fn with_symlinks_it_fails_with_an_error() {
    let file_system = Arc::new(MockSymlinkFileSystem);
    let single_module_parser = Arc::new(MockSingleModuleParser);
    let multi_module_parser = MultiModuleParserImpl::new(single_module_parser, file_system);

    let result = multi_module_parser.parse(vec![PathBuf::from("/anypath.cds")]);

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.message().starts_with("Cannot open file by the path"));
    assert_eq!(error.error_code(), ErrorCode::FileIOError);
}
