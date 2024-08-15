use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use crate::parser::fs::NativeFileSystem;

use super::super::{SingleModuleParser, SingleModuleParserImpl};

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
    let native_file_system = Arc::new(NativeFileSystem::new());

    let _result = SingleModuleParserImpl::new(native_file_system)
        .parse(&"test_correct.cds")
        .unwrap();

    remove_file("test_correct.cds").unwrap();
}

#[test]
fn with_unexisting_file_it_fails() {
    let native_file_system = Arc::new(NativeFileSystem::new());

    let result = SingleModuleParserImpl::new(native_file_system).parse(&"test.cds");

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
    let native_file_system = Arc::new(NativeFileSystem::new());

    let result = SingleModuleParserImpl::new(native_file_system).parse(&"test_incorrect.cds");

    remove_file("test_incorrect.cds").unwrap();

    assert!(result.is_err());
}
