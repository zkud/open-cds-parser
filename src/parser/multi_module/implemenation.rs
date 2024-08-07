use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::*;
use crate::parser::fs::FileSystem;
use crate::parser::single_module::SingleModuleParser;
use crate::visitor::Visitor;

use super::super::super::ast::ModuleTerm;
use super::super::parse_error::ParseError;
use super::super::parse_error::ParseErrorType;

use super::MultiModuleParser;

pub struct MultiModuleParserImpl {
    single_module_parser: Box<dyn SingleModuleParser>,
    file_system: Arc<dyn FileSystem>,
}

impl MultiModuleParserImpl {
    pub fn new(
        single_module_parser: Box<dyn SingleModuleParser>,
        file_system: Arc<dyn FileSystem>,
    ) -> Self {
        MultiModuleParserImpl {
            single_module_parser,
            file_system,
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
        if self.file_system.path_is_file(path) {
            return self.parse_single_file_wrapper(path, result);
        }

        if self.file_system.path_is_directory(path) {
            let file_paths = self.file_system.get_all_cds_files_in_dir(path)?;
            for file_path in file_paths {
                self.parse_single_file_wrapper(&file_path, result)?;
            }
            return Ok(());
        }

        Err(ParseError::new(
            "Invalid path: ".to_string() + path,
            ParseErrorType::FileIOError,
        ))
    }

    fn parse_single_file_wrapper(
        &self,
        path: &str,
        result: &mut HashMap<String, ModuleTerm>,
    ) -> Result<(), ParseError> {
        let absolute_path = self.file_system.to_absolute(path)?;
        if result.contains_key(&absolute_path) {
            return Ok(());
        }

        let module_term = self.single_module_parser.parse(&path)?;
        result.insert(absolute_path, (*module_term).clone());

        let parent_dir = self.file_system.get_parent_dir(path)?;

        struct UsingVisitor<'a> {
            parser: &'a MultiModuleParserImpl,
            result: &'a mut HashMap<String, ModuleTerm>,
            current_dir: String,
        }

        impl<'a> Visitor for UsingVisitor<'a> {
            type Error = ParseError;

            fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ParseError> {
                if let Some(term) = term.try_convert::<ImportTerm>() {
                    let using_path = term.path().value();
                    let path_to_dependency = self
                        .parser
                        .file_system
                        .join_paths(&self.current_dir, &using_path)?;
                    self.parser
                        .parse_path(&(path_to_dependency + &".cds"), self.result)?;
                }
                Ok(())
            }
        }

        let mut using_visitor = UsingVisitor {
            parser: self,
            result,
            current_dir: parent_dir,
        };

        module_term.accept(&mut using_visitor)
    }
}
