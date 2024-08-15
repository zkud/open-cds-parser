use super::super::super::ast::ModuleTerm;
use super::super::parse_error::ParseError;

pub trait SingleModuleParser {
    fn parse(&self, path: &str) -> Result<Box<ModuleTerm>, ParseError>;
}
