use std::path::PathBuf;

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

#[inline]
fn get_service_path() -> PathBuf {
    PathBuf::from("./tests/projects/trivial/srv/cat-service.cds")
}

#[inline]
fn parse() -> Box<ModuleTerm> {
    let parser = Parser::new_with_native_fs();
    parser.parse_single_module(&get_service_path()).unwrap()
}

#[test]
fn test_parser_checks_for_grammar_at_least() {
    let parser = Parser::new_with_native_fs();
    let path = get_service_path();

    let ast = parser.parse_single_module(&path);

    assert!(ast.is_ok());
}

#[test]
fn test_module_structure() {
    let ast = parse();

    assert_eq!(ast.definitions().len(), 1);
    assert!(matches!(ast.definitions()[0], ModuleDefinition::Service(_)));
}

#[test]
fn test_service_definition() {
    let ast = parse();

    if let ModuleDefinition::Service(service) = &ast.definitions()[0] {
        assert_eq!(service.identifier().full_name(), "CatalogService");
        assert_eq!(service.definitions().len(), 2);
    } else {
        panic!("Expected service definition");
    }
}

#[test]
fn test_entity_structure() {
    let ast = parse();

    if let ModuleDefinition::Service(service) = &ast.definitions()[0] {
        if let ServiceDefinition::Entity(entity) = &service.definitions()[0] {
            assert_eq!(entity.identifier().full_name(), "UserScopes");
            assert_eq!(entity.fields().len(), 2);

            // Test first field
            let username_field = &entity.fields()[0];
            assert_eq!(username_field.name().full_name(), "username");
            assert_eq!(username_field.type_name().full_name(), "String");

            // Test second field
            let scope_field = &entity.fields()[1];
            assert_eq!(scope_field.name().full_name(), "scope");
            assert_eq!(scope_field.type_name().full_name(), "String");
        } else {
            panic!("Expected entity definition");
        }
    }
}

#[test]
fn test_function_declaration() {
    let ast = parse();

    if let ModuleDefinition::Service(service) = &ast.definitions()[0] {
        if let ServiceDefinition::Function(function) = &service.definitions()[1] {
            assert_eq!(function.identifier().full_name(), "getUserScopesCount");
            assert!(function.parameters().parameters_and_commas().is_empty());

            // Test return type
            let return_decl = function.returns();
            let return_type = return_decl.type_reference();
            let return_type_name =
                if let TypeDetailsVariant::Simple(simple) = return_type.type_details().as_ref() {
                    simple.identifier().full_name()
                } else {
                    panic!("Unexpected type variant!");
                };
            assert_eq!(return_type_name, "Integer");
            assert!(!return_type.is_arrayed());
        } else {
            panic!("Expected function definition");
        }
    }
}
