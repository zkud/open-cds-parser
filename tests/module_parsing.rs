use std::collections::HashMap;
use std::path::{Path, PathBuf};

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

/// Helper functions
#[inline]
fn get_schema_path() -> PathBuf {
    PathBuf::from("./tests/projects/modules/db/schema.cds")
        .canonicalize()
        .unwrap()
}

#[inline]
fn get_service_path() -> PathBuf {
    PathBuf::from("./tests/projects/modules/srv/books.cds")
        .canonicalize()
        .unwrap()
}

#[inline]
fn setup_parser() -> Parser {
    Parser::new_with_native_fs()
}

#[inline]
fn parse_test_modules() -> HashMap<PathBuf, ModuleTerm> {
    setup_parser()
        .parse_multiple_modules(vec![PathBuf::from("./tests/projects/modules/srv")])
        .unwrap()
}

// Top level module structure tests
#[test]
fn test_module_presence() {
    let ast = parse_test_modules();

    // Verify both modules exist
    assert!(ast.contains_key(&get_schema_path()));
    assert!(ast.contains_key(&get_service_path()));
}

#[test]
fn test_module_definition_counts() {
    let ast = parse_test_modules();

    let schema_module = ast.get(&get_schema_path()).unwrap();
    let service_module = ast.get(&get_service_path()).unwrap();

    // Schema module must have one entity definition
    assert_eq!(schema_module.definitions().len(), 1);
    // Service module must have import and service definitions
    assert_eq!(service_module.definitions().len(), 2);
}

#[test]
fn test_module_definition_types() {
    let ast = parse_test_modules();

    let schema_module = ast.get(&get_schema_path()).unwrap();
    let service_module = ast.get(&get_service_path()).unwrap();

    // Schema module should contain an Entity
    assert!(matches!(
        schema_module.definitions()[0],
        ModuleDefinition::Entity(_)
    ));

    // Service module should contain Import followed by Service
    assert!(matches!(
        service_module.definitions()[0],
        ModuleDefinition::Import(_)
    ));
    assert!(matches!(
        service_module.definitions()[1],
        ModuleDefinition::Service(_)
    ));
}

// Service level tests
#[test]
fn test_service_term() {
    let ast = parse_test_modules();
    let service_module = ast.get(&get_service_path()).unwrap();

    if let ModuleDefinition::Service(service) = &service_module.definitions()[1] {
        assert_eq!(service.identifier().full_name(), "BooksService");
        assert!(service.definitions().is_empty());

        assert_eq!(service.location().start(), 36);
        assert_eq!(service.location().end(), 61);
    } else {
        panic!("Expected Service definition");
    }
}

#[test]
fn test_module_imports() {
    let ast = parse_test_modules();
    let service_module = ast.get(&get_service_path()).unwrap();

    if let ModuleDefinition::Import(import) = &service_module.definitions()[0] {
        assert_import_identifier_and_path_are_correct(import);
        assert_selection_block_is_correct(import);
    } else {
        panic!("Expected Import definition");
    }
}

fn assert_import_identifier_and_path_are_correct(import: &ImportTerm) {
    assert_eq!(import.location().start(), 0);
    assert_eq!(import.location().end(), 32);
    assert_eq!(import.path().value(), &Path::new("../db/schema"));
}

fn assert_selection_block_is_correct(import: &ImportTerm) {
    let selection_block = import.selection_block();
    let selectors = selection_block.selectors();
    assert_eq!(selectors.len(), 1);
    if let SelectionBlockSegment::Selector(selector) = &selectors[0] {
        if let ImportIdentifierVariant::IdentifierOnly(ident) =
            selector.import_identifier().variant().as_ref()
        {
            assert_eq!(ident.full_name(), "Books");
        } else {
            panic!("Expected IdentifierOnly variant");
        }
    } else {
        panic!("Expected Selector segment");
    }
}

// Entity level tests
#[test]
fn test_entity_basic_properties() {
    let ast = parse_test_modules();
    let schema_module = ast.get(&get_schema_path()).unwrap();

    if let ModuleDefinition::Entity(entity) = &schema_module.definitions()[0] {
        assert_eq!(entity.identifier().full_name(), "Books");
        assert_eq!(entity.fields().len(), 2);
    } else {
        panic!("Expected Entity definition");
    }
}

#[test]
fn test_entity_fields() {
    let ast = parse_test_modules();
    let schema_module = ast.get(&get_schema_path()).unwrap();

    if let ModuleDefinition::Entity(entity) = &schema_module.definitions()[0] {
        let fields = entity.fields();

        // Test id field
        assert_eq!(fields[0].name().full_name(), "id");
        assert_eq!(fields[0].type_name().full_name(), "UUID");

        // Test name field
        assert_eq!(fields[1].name().full_name(), "name");
        assert_eq!(fields[1].type_name().full_name(), "String");
    } else {
        panic!("Expected Entity definition");
    }
}
