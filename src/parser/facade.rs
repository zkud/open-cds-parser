use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use super::fs::FileSystem;
use super::fs::NativeFileSystem;
use super::multi_module::*;
use super::single_module::*;

use super::super::ast::ModuleTerm;
use super::error::ParseError;

pub struct Parser {
    single_module_parser: Arc<dyn SingleModuleParser>,
    multi_module_parser: Arc<dyn MultiModuleParser>,
}

impl Parser {
    pub fn new(file_system: Arc<dyn FileSystem>) -> Self {
        let single_module_parser = Arc::new(SingleModuleParserImpl::new(file_system.clone()));
        Self {
            single_module_parser: single_module_parser.clone(),
            multi_module_parser: Arc::new(MultiModuleParserImpl::new(
                single_module_parser.clone(),
                file_system.clone(),
            )),
        }
    }

    pub fn new_with_native_fs() -> Self {
        let file_system = Arc::new(NativeFileSystem::new());
        Parser::new(file_system)
    }

    pub fn parse_single_module(&self, path: &Path) -> Result<Box<ModuleTerm>, ParseError> {
        self.single_module_parser.parse(path)
    }

    pub fn parse_multiple_modules(
        &self,
        paths: Vec<PathBuf>,
    ) -> Result<HashMap<PathBuf, ModuleTerm>, ParseError> {
        self.multi_module_parser.parse(paths)
    }
}
