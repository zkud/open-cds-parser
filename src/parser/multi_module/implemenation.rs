use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::*;
use crate::parser::fs::FileSystem;
use crate::parser::single_module::SingleModuleParser;

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
        let mut using_visitor = PathCollectorVisitor::new(self.file_system.clone(), &parent_dir);
        module_term.accept(&mut using_visitor)?;

        for module in using_visitor.modules_to_parse().into_iter() {
            self.parse_single_file_wrapper(&module, result)?;
        }

        Ok(())
    }
}

struct PathCollectorVisitor {
    file_system: Arc<dyn FileSystem>,
    current_dir: String,
    modules_to_parse: Vec<String>,
}

impl PathCollectorVisitor {
    pub fn new(file_system: Arc<dyn FileSystem>, current_dir: &str) -> Self {
        Self {
            file_system: file_system,
            current_dir: current_dir.to_string(),
            modules_to_parse: vec![],
        }
    }

    pub fn modules_to_parse(self) -> Vec<String> {
        self.modules_to_parse
    }
}

impl Visitor for PathCollectorVisitor {
    type Error = ParseError;

    fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ParseError> {
        if let Some(term) = term.try_convert::<ImportTerm>() {
            let using_path = term.path().value();
            let path_to_dependency = self
                .file_system
                .join_paths(&self.current_dir, &using_path)?;
            let direct_dependency = path_to_dependency.clone() + &".cds";
            let module_dependency = path_to_dependency.clone() + &"/index.cds";
            let direct_exists = self.file_system.file_exists(&direct_dependency);
            let module_exists = self.file_system.file_exists(&module_dependency);
            return match (direct_exists, module_exists) {
                (true, false) => {
                    self.modules_to_parse.push(direct_dependency);
                    Ok(())
                }
                (false, true) => {
                    self.modules_to_parse.push(module_dependency);
                    Ok(())
                }
                (true, true) => Err(ParseError::new(
                    format!(
                        "Unexpected duplication {}, both file and dir/index.cds are present",
                        path_to_dependency
                    ),
                    ParseErrorType::FileIOError,
                )),
                _ => Err(ParseError::new(
                    format!("Cannot find import {}", path_to_dependency),
                    ParseErrorType::FileIOError,
                )),
            };
        }
        Ok(())
    }
}
