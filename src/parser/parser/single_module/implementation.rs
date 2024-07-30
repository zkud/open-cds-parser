use std::io::prelude::*;

use std::fs::File;
use std::path::Path;

use super::super::super::super::ast::ModuleTerm;
use super::super::super::parse_error::ParseError;
use super::super::super::parse_error::ParseErrorType;
use super::SingleModuleParser;

pub struct SingleModuleParserImpl {}

impl SingleModuleParserImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl SingleModuleParser for SingleModuleParserImpl {
    fn parse(&self, path: &str) -> Result<Box<ModuleTerm>, ParseError> {
        let path = Path::new(path);

        let mut file = File::open(path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let module = match super::cds::ModuleParser::new().parse(&content) {
            Ok(module_ast) => module_ast,
            Err(lalrpop_auto_generated_error) => {
                return Err(ParseError::new(
                    format!(
                        "File: {} Error: {}",
                        path.to_string_lossy(),
                        lalrpop_auto_generated_error
                    ),
                    ParseErrorType::SyntaxError,
                ))
            }
        };

        Ok(Box::new(module))
    }
}
