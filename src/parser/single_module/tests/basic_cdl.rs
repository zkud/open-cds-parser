use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;

use crate::parser::fs::NativeFileSystem;

use super::super::{SingleModuleParser, SingleModuleParserImpl};

#[test]
fn with_correct_input_it_translates() {
    let path = Path::new("test_correct.cds");
    let mut test_file = File::create(path).unwrap();
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
    let native_file_system = Arc::new(NativeFileSystem::new());

    let _result = SingleModuleParserImpl::new(native_file_system)
        .parse(path)
        .unwrap();

    remove_file(path).unwrap();
}

#[test]
fn with_unexisting_file_it_fails() {
    let native_file_system = Arc::new(NativeFileSystem::new());
    let path = Path::new("test.cds");

    let result = SingleModuleParserImpl::new(native_file_system).parse(path);

    assert!(result.is_err());
}

#[test]
fn with_syntactically_incorrect_it_fails() {
    let path = Path::new("test_incorrect.cds");
    let mut test_file = File::create(path).unwrap();
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
    let native_file_system = Arc::new(NativeFileSystem::new());

    let result = SingleModuleParserImpl::new(native_file_system).parse(path);

    remove_file(path).unwrap();

    assert!(result.is_err());
}
