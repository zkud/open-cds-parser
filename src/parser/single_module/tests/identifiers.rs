use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn with_basic_service_name_it_parses() {
    let source = "
        service SimpleService {
        }
    ";

    let result = parse_single_file(&source);

    expect_identifier_to_be(result, build_basic_identifier());
}

#[inline]
fn parse_single_file(file_content: &str) -> Result<Box<ModuleTerm>, ParseError> {
    let mut files = HashMap::new();
    files.insert(get_import_path(), file_content.to_string());

    let file_system = Arc::new(MockInMemoryFileSystem::new(HashMap::new(), files));
    let parser = SingleModuleParserImpl::new(file_system);

    parser.parse(&get_import_path())
}

#[inline]
fn get_import_path() -> PathBuf {
    PathBuf::from("/import.cds")
}

#[test]
fn with_namespaced_service_name_it_parses() {
    let source = "
        service My.Namespaced.Service {
        }
    ";

    let result = parse_single_file(&source);

    expect_identifier_to_be(result, build_namespaced_identifier());
}

#[inline]
fn expect_identifier_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    expected_identifier: IdentifierTerm,
) {
    let parsed_module = module_to_check.unwrap();
    if let ModuleDefinition::Service(service) = &parsed_module.definitions()[0] {
        assert_eq!(service.identifier().deref(), &expected_identifier);
        return;
    }
    panic!("Service not found!");
}

#[inline]
fn build_basic_identifier() -> IdentifierTerm {
    IdentifierTerm::new_basic(Location::new(17, 30, &get_import_path()), "SimpleService")
}

#[inline]
fn build_namespaced_identifier() -> IdentifierTerm {
    IdentifierTerm::new(
        Location::new(17, 38, &get_import_path()),
        vec![
            IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                Location::new(17, 19, &get_import_path()),
                "My".to_string(),
            )),
            IdentifierSegment::Dot(PunctuationSignTerm::new(
                Location::new(19, 20, &get_import_path()),
                PunctuationSign::Dot,
            )),
            IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                Location::new(20, 30, &get_import_path()),
                "Namespaced".to_string(),
            )),
            IdentifierSegment::Dot(PunctuationSignTerm::new(
                Location::new(30, 31, &get_import_path()),
                PunctuationSign::Dot,
            )),
            IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                Location::new(31, 38, &get_import_path()),
                "Service".to_string(),
            )),
        ],
    )
}
