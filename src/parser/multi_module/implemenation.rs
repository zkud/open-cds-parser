use std::collections::HashMap;

use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::{ASTTerm, ImportTerm};
use crate::parser::single_module::SingleModuleParser;
use crate::visitor::Visitor;

use super::super::super::ast::ModuleTerm;
use super::super::parse_error::ParseError;
use super::super::parse_error::ParseErrorType;

use super::MultiModuleParser;

pub struct MultiModuleParserImpl {
    single_module_parser: Box<dyn SingleModuleParser>,
}

impl MultiModuleParserImpl {
    pub fn new(single_module_parser: Box<dyn SingleModuleParser>) -> Self {
        MultiModuleParserImpl {
            single_module_parser,
        }
    }
}

impl MultiModuleParser for MultiModuleParserImpl {
    fn parse(&self, paths: Vec<String>) -> Result<HashMap<String, ModuleTerm>, ParseError> {
        let mut result = HashMap::new();

        for path in paths {
            self.parse_path(&path, &mut result)?;
        }

        Ok(result)
    }
}

impl MultiModuleParserImpl {
    fn parse_path(
        &self,
        path: &str,
        result: &mut HashMap<String, ModuleTerm>,
    ) -> Result<(), ParseError> {
        let path = Path::new(path);

        if path.is_file() {
            self.parse_single_file_wrapper(path, result)?;
        } else if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();
                if file_path.extension().and_then(|s| s.to_str()) == Some("cds") {
                    self.parse_single_file_wrapper(&file_path, result)?;
                }
            }
        } else {
            return Err(ParseError::new(
                "Invalid path".to_string() + &path.to_string_lossy(),
                ParseErrorType::FileIOError,
            ));
        }

        Ok(())
    }

    fn parse_single_file_wrapper(
        &self,
        path: &Path,
        result: &mut HashMap<String, ModuleTerm>,
    ) -> Result<(), ParseError> {
        let path_str = path.canonicalize()?.to_string_lossy().into_owned();

        if result.contains_key(&path_str) {
            return Ok(());
        }

        let module_term = self.single_module_parser.parse(&path_str)?;
        result.insert(path_str.clone(), (*module_term).clone());

        let parent_dir = path.parent().ok_or_else(|| {
            ParseError::new(
                "Unable to get parent directory".to_string() + &path_str.clone(),
                ParseErrorType::FileIOError,
            )
        })?;

        struct UsingVisitor<'a> {
            parser: &'a MultiModuleParserImpl,
            result: &'a mut HashMap<String, ModuleTerm>,
            current_dir: PathBuf,
        }

        impl<'a> Visitor<ParseError> for UsingVisitor<'a> {
            fn process_import(&mut self, term: &ImportTerm) -> Result<(), ParseError> {
                let using_path = term.path().value();
                let full_path = self.current_dir.join(using_path);
                let formatted_path = &full_path.to_string_lossy().to_owned();
                self.parser
                    .parse_path(&(formatted_path.to_string() + &".cds"), self.result)?;
                Ok(())
            }
        }

        let mut using_visitor = UsingVisitor {
            parser: self,
            result,
            current_dir: parent_dir.to_path_buf(),
        };

        module_term.accept(&mut using_visitor)
    }
}
