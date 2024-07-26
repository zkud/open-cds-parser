use std::collections::HashMap;
use std::io::prelude::*;

use std::fs;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

use crate::ast::{ASTTerm, UsingTerm};
use crate::visitor::{Visitor, VisitorError};

use super::super::ast::ModuleTerm;
use super::parse_error::ParseError;
use super::parse_error::ParseErrorType;

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse_single_file(&self, path: &str) -> Result<Box<ModuleTerm>, ParseError> {
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
            fn process_using(&mut self, term: &UsingTerm) -> Result<(), VisitorError> {
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

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::prelude::*;

    use super::Parser;

    #[test]
    fn with_correct_input_it_translates() {
        let mut test_file = File::create("test_correct.cds").unwrap();
        test_file
            .write_all(
                b"
                type test : Testtype;

                entity Test {
                    field : TestType;
                    field2    : TestType2;
                }

                entity Test2 : Astpect {
                    field : TestType;
                    field2    : TestType2;
                }
            
                service TestService {
                    type test : Test;
            
                    entity Test2 : Aspect1 {
                    field3 : Test3;
                    }
            
                    action atest(arg1: Test);
                    action atest1(arg1: Test) returns Test;
                    action atest2(arg1: Test) returns array of Test;
            
                    function ftest0() returns Test;
                    function ftest1(arg1: Test) returns Test;
                    function ftest2(arg1: Test) returns array of Test;
                }
                ",
            )
            .unwrap();

        let _result = Parser::new()
            .parse_single_file(&"test_correct.cds")
            .unwrap();

        remove_file("test_correct.cds").unwrap();
    }

    #[test]
    fn with_unexisting_file_it_fails() {
        let result = Parser::new().parse_single_file(&"test.cds");

        assert!(result.is_err());
    }

    #[test]
    fn with_syntactically_incorrect_it_fails() {
        let mut test_file = File::create("test_incorrect.cds").unwrap();
        test_file
            .write_all(
                b"
                    service TestService {
                        function ftest0() returns Test;
                        function ftest1(turns Test;
                        function ftest2(arg1: Test) returns array of Test;
                    }
                ",
            )
            .unwrap();

        let result = Parser::new().parse_single_file(&"test_incorrect.cds");

        remove_file("test_incorrect.cds").unwrap();

        assert!(result.is_err());
    }
}
