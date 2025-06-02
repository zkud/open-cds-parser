use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn with_empty_service_declaration_it_parses() {
    let source = "service Example {}";

    let result = parse_single_file(&source);

    expect_service_declaration_to_be(result, build_empty_service_declaration());
}

#[test]
fn with_service_declaration_with_empty_entity_it_parses() {
    let source = "service Example { entity TestEntity {}; }";

    let result = parse_single_file(&source);

    expect_service_declaration_to_be(result, build_service_with_empty_entity_declaration());
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

#[inline]
fn build_empty_service_declaration() -> ServiceDeclarationTerm {
    ServiceDeclarationTerm::new(
        Location::new(0, 18, &get_import_path()),
        Box::new(KeywordTerm::new(
            Location::new(0, 7, &get_import_path()),
            Keyword::Service,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(8, 15, &get_import_path()),
            "Example",
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(16, 17, &get_import_path()),
            PunctuationSign::OpenCurlyBrace,
        )),
        Vec::new(),
        Box::new(PunctuationSignTerm::new(
            Location::new(17, 18, &get_import_path()),
            PunctuationSign::CloseCurlyBrace,
        )),
    )
}

#[inline]
fn build_service_with_empty_entity_declaration() -> ServiceDeclarationTerm {
    let entity_decl = EntityDeclarationTerm::new(
        Location::new(18, 39, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(18, 24, &get_import_path()),
            Keyword::Entity,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(25, 35, &get_import_path()),
            "TestEntity",
        )),
        None,
        Vec::new(),
        Box::new(StructureTerm::new(
            Location::new(36, 38, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(36, 37, &get_import_path()),
                PunctuationSign::OpenCurlyBrace,
            )),
            Vec::new(),
            Box::new(PunctuationSignTerm::new(
                Location::new(37, 38, &get_import_path()),
                PunctuationSign::CloseCurlyBrace,
            )),
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(38, 39, &get_import_path()),
            PunctuationSign::Semicolumn,
        ))),
    );

    ServiceDeclarationTerm::new(
        Location::new(0, 41, &get_import_path()),
        Box::new(KeywordTerm::new(
            Location::new(0, 7, &get_import_path()),
            Keyword::Service,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(8, 15, &get_import_path()),
            "Example",
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(16, 17, &get_import_path()),
            PunctuationSign::OpenCurlyBrace,
        )),
        vec![ServiceDefinition::Entity(entity_decl)],
        Box::new(PunctuationSignTerm::new(
            Location::new(40, 41, &get_import_path()),
            PunctuationSign::CloseCurlyBrace,
        )),
    )
}

#[inline]
fn expect_service_declaration_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    expected_service: ServiceDeclarationTerm,
) {
    let module_to_check = module_to_check.expect("Unexpected Error");
    assert_eq!(module_to_check.definitions().len(), 1);
    let service_decl = module_to_check
        .definitions()
        .first()
        .expect("Unable to retrieve service declaration");
    let service_decl = if let ModuleDefinition::Service(service_decl) = service_decl {
        service_decl
    } else {
        panic!("Expected service declaration but got something else");
    };
    assert_eq!(service_decl, &expected_service);
}
