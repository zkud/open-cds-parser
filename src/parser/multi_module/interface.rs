use std::collections::HashMap;
use std::path::PathBuf;

use super::super::super::ast::ModuleTerm;
use super::super::error::ParseError;

pub trait MultiModuleParser {
    fn parse(&self, paths: Vec<PathBuf>) -> Result<HashMap<PathBuf, ModuleTerm>, ParseError>;
}
