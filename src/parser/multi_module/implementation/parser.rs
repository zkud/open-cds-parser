use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crate::ast::*;
use crate::parser::fs::FileSystem;
use crate::parser::single_module::SingleModuleParser;

use super::super::super::parse_error::ParseError;
use super::super::super::parse_error::ParseErrorType;
use crate::ast::ModuleTerm;

use super::super::MultiModuleParser;
use super::PathCollectorVisitor;

pub struct MultiModuleParserImpl {
    single_module_parser: Arc<dyn SingleModuleParser>,
    file_system: Arc<dyn FileSystem>,
}

impl MultiModuleParserImpl {
    pub fn new(
        single_module_parser: Arc<dyn SingleModuleParser>,
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
        if self.file_system.path_is_file(&Path::new(path)) {
            return self.parse_single_file(path, result);
        }

        if self.file_system.path_is_directory(&Path::new(path)) {
            return self.parse_directory(path, result);
        }

        Err(ParseError::new(
            "Invalid path: ".to_string() + path,
            ParseErrorType::FileIOError,
        ))
    }

    fn parse_directory(
        &self,
        path: &str,
        result: &mut HashMap<String, ModuleTerm>,
    ) -> Result<(), ParseError> {
        let file_paths = self.file_system.get_all_cds_files_in_dir(path)?;
        for file_path in file_paths {
            self.parse_single_file(&file_path, result)?;
        }
        Ok(())
    }

    fn parse_single_file(
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

        let abstract_paths = self.collect_abstract_paths(path, &module_term)?;
        let concrete_paths = self.resolve_paths(&abstract_paths)?;
        for path in concrete_paths {
            self.parse_single_file(&path, result)?;
        }

        Ok(())
    }

    fn collect_abstract_paths(
        &self,
        path: &str,
        module_term: &ModuleTerm,
    ) -> Result<Vec<String>, ParseError> {
        let parent_dir = self.file_system.get_parent_dir(path)?;
        let mut using_visitor = PathCollectorVisitor::new(self.file_system.clone(), &parent_dir);
        module_term.accept(&mut using_visitor)?;
        Ok(using_visitor.modules_to_parse())
    }

    fn resolve_paths(&self, abstract_paths: &[String]) -> Result<Vec<String>, ParseError> {
        let mut concrete_paths = vec![];

        for path in abstract_paths {
            let direct_dependency = path.clone() + &".cds";
            let direct_dependency_path = PathBuf::from(path.clone() + &".cds");
            let direct_exists = self.file_system.file_exists(&direct_dependency_path);
            let module_dependency = path.clone() + &"/index.cds";
            let module_dependency_path = PathBuf::from(path.clone() + &"/index.cds");
            let module_exists = self.file_system.file_exists(&module_dependency_path);
            match (direct_exists, module_exists) {
                (true, false) => {
                    concrete_paths.push(direct_dependency);
                }
                (false, true) => {
                    concrete_paths.push(module_dependency);
                }
                (true, true) => {
                    return Err(ParseError::new(
                        format!(
                            "Unexpected duplication {}, both file and dir/index.cds are present",
                            path
                        ),
                        ParseErrorType::FileIOError,
                    ))
                }
                _ => {
                    return Err(ParseError::new(
                        format!("Cannot find import {}", path),
                        ParseErrorType::FileIOError,
                    ))
                }
            };
        }

        Ok(concrete_paths)
    }
}
