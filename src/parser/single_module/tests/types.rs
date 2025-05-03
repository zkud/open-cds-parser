use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn with_basic_type_declaration_it_parses() {
    let source = "
        type Example : String;
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_basic_type_declaration());
}

#[test]
fn with_define_type_declaration_it_parses() {
    let source = "
        define type Example : String;
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_define_type_declaration());
}

#[test]
fn with_array_type_reference_it_parses() {
    let source = "
        type ArrayExample : array of String;
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_array_type_declaration());
}

#[test]
fn with_many_type_reference_it_parses() {
    let source = "
        type ManyExample : many String;
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_many_type_declaration());
}

#[test]
fn with_structured_type_it_parses() {
    let source = "
        type StructExample : {};
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_structured_type_declaration());
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
fn build_basic_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 31, &get_import_path()),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 21, &get_import_path()),
            "Example",
        )),
        Box::new(TypeReferenceTerm::new(
            Location::new(24, 30, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Simple(SimpleTypeDetailsTerm::new(
                Location::new(24, 30, &get_import_path()),
                Box::new(IdentifierTerm::new_basic(
                    Location::new(24, 30, &get_import_path()),
                    "String",
                )),
            ))),
        )),
    )
}

#[inline]
fn build_define_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 38, &get_import_path()),
        Box::new(IdentifierTerm::new_basic(
            Location::new(21, 28, &get_import_path()),
            "Example",
        )),
        Box::new(TypeReferenceTerm::new(
            Location::new(31, 37, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Simple(SimpleTypeDetailsTerm::new(
                Location::new(31, 37, &get_import_path()),
                Box::new(IdentifierTerm::new_basic(
                    Location::new(31, 37, &get_import_path()),
                    "String",
                )),
            ))),
        )),
    )
}

#[inline]
fn build_array_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 45, &get_import_path()),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 26, &get_import_path()),
            "ArrayExample",
        )),
        Box::new(TypeReferenceTerm::new(
            Location::new(29, 44, &get_import_path()),
            Some(ArrayPrefix::ArrayOf {
                array: Box::new(KeywordTerm::new(
                    Location::new(29, 34, &get_import_path()),
                    Keyword::Array,
                )),
                of: Box::new(OfTerm::new(Location::new(35, 37, &get_import_path()))),
            }),
            Box::new(TypeDetailsVariant::Simple(SimpleTypeDetailsTerm::new(
                Location::new(38, 44, &get_import_path()),
                Box::new(IdentifierTerm::new_basic(
                    Location::new(38, 44, &get_import_path()),
                    "String",
                )),
            ))),
        )),
    )
}

#[inline]
fn build_many_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 40, &get_import_path()),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 25, &get_import_path()),
            "ManyExample",
        )),
        Box::new(TypeReferenceTerm::new(
            Location::new(28, 39, &get_import_path()),
            Some(ArrayPrefix::Many(Box::new(ManyTerm::new(Location::new(
                28,
                32,
                &get_import_path(),
            ))))),
            Box::new(TypeDetailsVariant::Simple(SimpleTypeDetailsTerm::new(
                Location::new(33, 39, &get_import_path()),
                Box::new(IdentifierTerm::new_basic(
                    Location::new(33, 39, &get_import_path()),
                    "String",
                )),
            ))),
        )),
    )
}

#[inline]
fn build_structured_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 33, &get_import_path()),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 27, &get_import_path()),
            "StructExample",
        )),
        Box::new(TypeReferenceTerm::new(
            Location::new(30, 32, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Structured(
                StructuredTypeDetailsTerm::new(Location::new(30, 32, &get_import_path())),
            )),
        )),
    )
}

#[inline]
fn expect_type_declaration_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    expected_type: TypeDeclarationTerm,
) {
    let module_to_check = module_to_check.expect("Unexpected Error");
    assert_eq!(module_to_check.definitions().len(), 1);
    let type_decl = module_to_check
        .definitions()
        .get(0)
        .expect("Unable to retrieve type declaration");
    let type_decl = if let ModuleDefinition::Type(type_decl) = type_decl {
        type_decl
    } else {
        panic!("Unable to retrieve type declaration")
    };
    assert_eq!(type_decl, &expected_type);
}
