use std::collections::HashMap;
use std::sync::Arc;

use super::fs::FileSystem;
use super::fs::NativeFileSystem;
use super::multi_module::*;
use super::single_module::*;

use super::super::ast::ModuleTerm;
use super::parse_error::ParseError;

pub struct Parser {
    single_module_parser: Box<dyn SingleModuleParser>,
    multi_module_parser: Box<dyn MultiModuleParser>,
}

impl Parser {
    pub fn new(file_system: Arc<dyn FileSystem>) -> Self {
        Self {
            single_module_parser: Box::new(SingleModuleParserImpl::new(file_system.clone())),
            multi_module_parser: Box::new(MultiModuleParserImpl::new(
                Box::new(SingleModuleParserImpl::new(file_system.clone())),
                file_system.clone(),
            )),
        }
    }

    pub fn new_with_native_fs() -> Self {
        let file_system = Arc::new(NativeFileSystem::new());
        Parser::new(file_system)
    }

    pub fn parse_single_module(&self, path: &str) -> Result<Box<ModuleTerm>, ParseError> {
        self.single_module_parser.parse(path)
    }

    pub fn parse_multiple_module(
        &self,
        paths: Vec<String>,
    ) -> Result<HashMap<String, ModuleTerm>, ParseError> {
        self.multi_module_parser.parse(paths)
    }
}
