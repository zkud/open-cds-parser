use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn with_empty_structure_it_parses() {
    let source = "
        type Empty : {};
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_empty_structure());
}

#[test]
fn with_single_field_structure_it_parses() {
    let source = "
        type Person : {
            name: String;
        };
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_single_field_structure());
}

#[test]
fn with_multiple_fields_structure_it_parses() {
    let source = "
        type Person : {
            name: String;
            age: Integer;
            email: String;
            active: Boolean;
        };
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(result, build_multiple_fields_structure());
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
fn build_empty_structure() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 25, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 13, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 19, &get_import_path()),
            "Empty",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(20, 21, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(22, 24, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Structured(StructureTerm::new(
                Location::new(22, 24, &get_import_path()),
                Box::new(PunctuationSignTerm::new(
                    Location::new(22, 23, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                )),
                vec![],
                Box::new(PunctuationSignTerm::new(
                    Location::new(23, 24, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                )),
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(24, 25, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn build_single_field_structure() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 61, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 13, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 20, &get_import_path()),
            "Person",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(21, 22, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(23, 60, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Structured(StructureTerm::new(
                Location::new(23, 60, &get_import_path()),
                Box::new(PunctuationSignTerm::new(
                    Location::new(23, 24, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                )),
                vec![FieldTerm::new(
                    Location::new(37, 50, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(37, 41, &get_import_path()),
                        "name",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(41, 42, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(43, 49, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(43, 49, &get_import_path()),
                            "String",
                        ))),
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(49, 50, &get_import_path()),
                        PunctuationSign::Semicolumn,
                    )),
                )],
                Box::new(PunctuationSignTerm::new(
                    Location::new(59, 60, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                )),
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(60, 61, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn build_multiple_fields_structure() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 143, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 13, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 20, &get_import_path()),
            "Person",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(21, 22, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(23, 142, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Structured(StructureTerm::new(
                Location::new(23, 142, &get_import_path()),
                Box::new(PunctuationSignTerm::new(
                    Location::new(23, 24, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                )),
                vec![
                    FieldTerm::new(
                        Location::new(37, 50, &get_import_path()),
                        Box::new(IdentifierTerm::new_basic(
                            Location::new(37, 41, &get_import_path()),
                            "name",
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(41, 42, &get_import_path()),
                            PunctuationSign::Colon,
                        )),
                        Box::new(TypeReferenceTerm::new(
                            Location::new(43, 49, &get_import_path()),
                            None,
                            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                                Location::new(43, 49, &get_import_path()),
                                "String",
                            ))),
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(49, 50, &get_import_path()),
                            PunctuationSign::Semicolumn,
                        )),
                    ),
                    FieldTerm::new(
                        Location::new(63, 76, &get_import_path()),
                        Box::new(IdentifierTerm::new_basic(
                            Location::new(63, 66, &get_import_path()),
                            "age",
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(66, 67, &get_import_path()),
                            PunctuationSign::Colon,
                        )),
                        Box::new(TypeReferenceTerm::new(
                            Location::new(68, 75, &get_import_path()),
                            None,
                            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                                Location::new(68, 75, &get_import_path()),
                                "Integer",
                            ))),
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(75, 76, &get_import_path()),
                            PunctuationSign::Semicolumn,
                        )),
                    ),
                    FieldTerm::new(
                        Location::new(89, 103, &get_import_path()),
                        Box::new(IdentifierTerm::new_basic(
                            Location::new(89, 94, &get_import_path()),
                            "email",
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(94, 95, &get_import_path()),
                            PunctuationSign::Colon,
                        )),
                        Box::new(TypeReferenceTerm::new(
                            Location::new(96, 102, &get_import_path()),
                            None,
                            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                                Location::new(96, 102, &get_import_path()),
                                "String",
                            ))),
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(102, 103, &get_import_path()),
                            PunctuationSign::Semicolumn,
                        )),
                    ),
                    FieldTerm::new(
                        Location::new(116, 132, &get_import_path()),
                        Box::new(IdentifierTerm::new_basic(
                            Location::new(116, 122, &get_import_path()),
                            "active",
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(122, 123, &get_import_path()),
                            PunctuationSign::Colon,
                        )),
                        Box::new(TypeReferenceTerm::new(
                            Location::new(124, 131, &get_import_path()),
                            None,
                            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                                Location::new(124, 131, &get_import_path()),
                                "Boolean",
                            ))),
                        )),
                        Box::new(PunctuationSignTerm::new(
                            Location::new(131, 132, &get_import_path()),
                            PunctuationSign::Semicolumn,
                        )),
                    ),
                ],
                Box::new(PunctuationSignTerm::new(
                    Location::new(141, 142, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                )),
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(142, 143, &get_import_path()),
            PunctuationSign::Semicolumn,
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
