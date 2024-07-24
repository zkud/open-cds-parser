use std::io::prelude::*;

use std::fs::File;
use std::path::Path;

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
                    format!("{}", lalrpop_auto_generated_error),
                    ParseErrorType::SyntaxError,
                ))
            }
        };

        Ok(Box::new(module))
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

        let _result = Parser::new().parse_single_file(&"test_correct.cds").unwrap();

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
