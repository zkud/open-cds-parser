use std::collections::HashMap;

use super::multi_module::*;
use super::single_module::*;

use super::super::super::ast::ModuleTerm;
use super::super::parse_error::ParseError;

pub struct Parser {
    single_module_parser: Box<dyn SingleModuleParser>,
    multi_module_parser: Box<dyn MultiModuleParser>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            single_module_parser: Box::new(SingleModuleParserImpl::new()),
            multi_module_parser: Box::new(MultiModuleParserImpl::new(Box::new(
                SingleModuleParserImpl::new(),
            ))),
        }
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
