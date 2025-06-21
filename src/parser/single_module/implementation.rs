use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crate::parser::fs::FileSystem;

use super::super::super::ast::ModuleTerm;
use super::super::parse_error::ErrorCode;
use super::super::parse_error::ParseError;
use super::SingleModuleParser;

pub struct SingleModuleParserImpl {
    file_system: Arc<dyn FileSystem>,
}

impl SingleModuleParserImpl {
    pub fn new(file_system: Arc<dyn FileSystem>) -> Self {
        Self { file_system }
    }
}

impl SingleModuleParser for SingleModuleParserImpl {
    fn parse(&self, path: &Path) -> Result<Box<ModuleTerm>, ParseError> {
        let content = self.file_system.read_content(&path)?;

        let module = match super::cds::ModuleParser::new().parse(&path, &content) {
            Ok(module_ast) => module_ast,
            Err(lalrpop_auto_generated_error) => return Err(ParseError::new(
                format!(
                    "Failed to parse the file \"{}\".\nPlease see the internal parser error: \"{}\"",
                    path.to_string_lossy().to_string(),
                    lalrpop_auto_generated_error
                ),
                ErrorCode::SyntaxError,
            )),
        };

        Ok(Box::new(module))
    }
}
