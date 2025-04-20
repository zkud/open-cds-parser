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

    expect_service_to_be(result, build_basic_service());
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

    expect_service_to_be(result, build_namespaced_service());
}

#[inline]
fn expect_service_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    expected_service: ServiceTerm,
) {
    let module_to_check = module_to_check.expect("Unexpected Error");
    assert_eq!(module_to_check.definitions().len(), 1);
    let service = module_to_check
        .definitions()
        .get(0)
        .expect("Unable to retrieve service");
    let service = if let ModuleDefinition::Service(service) = service {
        service
    } else {
        panic!("Unable to retrieve service")
    };
    assert_eq!(service, &expected_service);
}

#[inline]
fn build_basic_service() -> ServiceTerm {
    ServiceTerm::new(
        Location::new(9, 42, &get_import_path()),
        Box::new(IdentifierTerm::new_basic(
            Location::new(17, 30, &get_import_path()),
            "SimpleService",
        )),
        vec![],
    )
}

#[inline]
fn build_namespaced_service() -> ServiceTerm {
    ServiceTerm::new(
        Location::new(9, 50, &get_import_path()),
        Box::new(IdentifierTerm::new(
            Location::new(17, 38, &get_import_path()),
            vec![
                IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                    Location::new(17, 19, &get_import_path()),
                    "My".to_string(),
                )),
                IdentifierSegment::Dot(DotTerm::new(Location::new(19, 20, &get_import_path()))),
                IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                    Location::new(20, 30, &get_import_path()),
                    "Namespaced".to_string(),
                )),
                IdentifierSegment::Dot(DotTerm::new(Location::new(30, 31, &get_import_path()))),
                IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                    Location::new(31, 38, &get_import_path()),
                    "Service".to_string(),
                )),
            ],
        )),
        vec![],
    )
}
