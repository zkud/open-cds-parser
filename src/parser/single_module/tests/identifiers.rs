use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

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

#[inline]
fn expect_service_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    service: ServiceTerm,
) {
    let module_to_check = module_to_check.expect("Unexpected Error");

    //find the correct start
    let start = match &service.name().segments()[0] {
        IdentifierSegment::SubIdentifier(sub_identifier) => sub_identifier.location().start(),
        _ => panic!("unsupported"),
    };

    let end = service.name().location().end();

    assert_eq!(
        module_to_check.deref(),
        &ModuleTerm::new(vec![ModuleDefinition::Service(ServiceTerm::new(
            Box::new(IdentifierTerm::new(
                Location::new(start, end, &get_import_path()),
                service.name().segments().clone()
            )),
            service.definitions().to_vec()
        ))])
    );
}

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
fn build_basic_service() -> ServiceTerm {
    ServiceTerm::new(
        Box::new(IdentifierTerm::new_basic(
            Location::new(17, 30, &get_import_path()),
            "SimpleService",
        )),
        vec![],
    )
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
fn build_namespaced_service() -> ServiceTerm {
    ServiceTerm::new(
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
#[test]
fn with_basic_entity_name_it_parses() {
    let source = "
        service SimpleService {
            entity SimpleEntity {}
        }
    ";

    let result = parse_single_file(&source);

    expect_service_to_be(result, build_service_with_entity());
}

#[inline]
fn build_service_with_entity() -> ServiceTerm {
    ServiceTerm::new(
        Box::new(IdentifierTerm::new_basic(
            Location::new(17, 30, &get_import_path()),
            "SimpleService",
        )),
        vec![ServiceDefinition::Entity(EntityTerm::new(
            Box::new(IdentifierTerm::new_basic(
                Location::new(52, 64, &get_import_path()),
                "SimpleEntity",
            )),
            vec![],
            vec![],
        ))],
    )
}

#[test]
fn with_namespaced_entity_name_it_parses() {
    let source = "
        service SimpleService {
            entity My.Namespaced.Entity {}
        }
    ";

    let result = parse_single_file(&source);

    expect_service_to_be(result, build_service_with_namespaced_entity());
}

#[inline]
fn build_service_with_namespaced_entity() -> ServiceTerm {
    ServiceTerm::new(
        Box::new(IdentifierTerm::new_basic(
            Location::new(17, 30, &get_import_path()),
            "SimpleService",
        )),
        vec![ServiceDefinition::Entity(EntityTerm::new(
            Box::new(IdentifierTerm::new(
                Location::new(52, 72, &get_import_path()),
                vec![
                    IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                        Location::new(52, 54, &get_import_path()),
                        "My".to_string(),
                    )),
                    IdentifierSegment::Dot(DotTerm::new(Location::new(54, 55, &get_import_path()))),
                    IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                        Location::new(55, 65, &get_import_path()),
                        "Namespaced".to_string(),
                    )),
                    IdentifierSegment::Dot(DotTerm::new(Location::new(65, 66, &get_import_path()))),
                    IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                        Location::new(66, 72, &get_import_path()),
                        "Entity".to_string(),
                    )),
                ],
            )),
            vec![],
            vec![],
        ))],
    )
}

#[test]
fn with_basic_type_declaration_name_it_parses() {
    let source = "
        service SimpleService {
            type SimpleType : String;
        }
    ";

    let result = parse_single_file(&source);

    expect_service_to_be(result, build_service_with_type_declaration());
}

#[inline]
fn build_service_with_type_declaration() -> ServiceTerm {
    ServiceTerm::new(
        Box::new(IdentifierTerm::new_basic(
            Location::new(17, 30, &get_import_path()),
            "SimpleService",
        )),
        vec![ServiceDefinition::Type(TypeDeclarationTerm::new(
            Box::new(IdentifierTerm::new_basic(
                Location::new(50, 60, &get_import_path()),
                "SimpleType",
            )),
            Box::new(IdentifierTerm::new_basic(
                Location::new(63, 69, &get_import_path()),
                "String",
            )),
        ))],
    )
}

#[test]
fn with_namespaced_type_declaration_name_it_parses() {
    let source = "
        service SimpleService {
            type My.Namespaced.Type : String;
        }
    ";

    let result = parse_single_file(&source);

    expect_service_to_be(result, build_service_with_namespaced_type_declaration());
}
#[inline]
fn build_service_with_namespaced_type_declaration() -> ServiceTerm {
    ServiceTerm::new(
        Box::new(IdentifierTerm::new_basic(
            Location::new(17, 30, &get_import_path()),
            "SimpleService",
        )),
        vec![ServiceDefinition::Type(TypeDeclarationTerm::new(
            Box::new(IdentifierTerm::new(
                Location::new(50, 68, &get_import_path()),
                vec![
                    IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                        Location::new(50, 52, &get_import_path()),
                        "My".to_string(),
                    )),
                    IdentifierSegment::Dot(DotTerm::new(Location::new(52, 53, &get_import_path()))),
                    IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                        Location::new(53, 63, &get_import_path()),
                        "Namespaced".to_string(),
                    )),
                    IdentifierSegment::Dot(DotTerm::new(Location::new(63, 64, &get_import_path()))),
                    IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                        Location::new(64, 68, &get_import_path()),
                        "Type".to_string(),
                    )),
                ],
            )),
            Box::new(IdentifierTerm::new_basic(
                Location::new(71, 77, &get_import_path()),
                "String",
            )),
        ))],
    )
}
