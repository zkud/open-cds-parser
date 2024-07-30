use std::collections::HashMap;

use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::{ASTTerm, ImportTerm};
use crate::visitor::{Visitor, VisitorError};

use super::super::super::super::ast::ModuleTerm;
use super::super::super::parse_error::ParseError;
use super::super::super::parse_error::ParseErrorType;

use super::super::Parser;

impl Parser {
    pub fn parse(&self, paths: Vec<String>) -> Result<HashMap<String, ModuleTerm>, ParseError> {
        let mut result = HashMap::new();

        for path in paths {
            self.parse_path(&path, &mut result)?;
        }

        Ok(result)
    }

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

        let module_term = self.parse_single_file(&path_str)?;
        result.insert(path_str.clone(), (*module_term).clone());

        let parent_dir = path.parent().ok_or_else(|| {
            ParseError::new(
                "Unable to get parent directory".to_string() + &path_str.clone(),
                ParseErrorType::FileIOError,
            )
        })?;

        struct UsingVisitor<'a> {
            parser: &'a Parser,
            result: &'a mut HashMap<String, ModuleTerm>,
            current_dir: PathBuf,
        }

        impl<'a> Visitor for UsingVisitor<'a> {
            fn process_import(&mut self, term: &ImportTerm) -> Result<(), VisitorError> {
                let using_path = term.path().value();
                let full_path = self.current_dir.join(using_path);
                let formatted_path = &full_path.to_string_lossy().to_owned();
                self.parser
                    .parse_path(&(formatted_path.to_string() + &".cds"), self.result)
                    .map_err(|e| VisitorError::new(e.to_string()))?;
                Ok(())
            }
        }

        let mut using_visitor = UsingVisitor {
            parser: self,
            result,
            current_dir: parent_dir.to_path_buf(),
        };

        module_term
            .accept(&mut using_visitor)
            .map_err(|e| ParseError::new(e.to_string(), ParseErrorType::SyntaxError))?;

        Ok(())
    }
}