use std::collections::HashMap;

use super::super::super::ast::ModuleTerm;
use super::super::parse_error::ParseError;

pub trait MultiModuleParser {
    fn parse(&self, paths: Vec<String>) -> Result<HashMap<String, ModuleTerm>, ParseError>;
}
