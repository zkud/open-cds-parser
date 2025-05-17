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

#[test]
fn with_structured_type_with_single_field_it_parses() {
    let source = "
        type Person : {
            name: String;
        };
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(
        result,
        build_structured_type_with_single_field_declaration(),
    );
}

#[test]
fn with_structured_type_with_multiple_fields_it_parses() {
    let source = "
        type Person : {
            name: String;
            age: Integer;
            email: String;
        };
    ";

    let result = parse_single_file(&source);
    expect_type_declaration_to_be(
        result,
        build_structured_type_with_multiple_fields_declaration(),
    );
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
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 13, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 21, &get_import_path()),
            "Example",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(22, 23, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(24, 30, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                Location::new(24, 30, &get_import_path()),
                "String",
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(30, 31, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn build_define_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 38, &get_import_path()),
        Some(Box::new(KeywordTerm::new(
            Location::new(9, 15, &get_import_path()),
            Keyword::Define,
        ))),
        Box::new(KeywordTerm::new(
            Location::new(16, 20, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(21, 28, &get_import_path()),
            "Example",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(29, 30, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(31, 37, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                Location::new(31, 37, &get_import_path()),
                "String",
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(37, 38, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn build_array_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 45, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 13, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 26, &get_import_path()),
            "ArrayExample",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(27, 28, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(29, 44, &get_import_path()),
            Some(ArrayPrefix::ArrayOf {
                array: Box::new(KeywordTerm::new(
                    Location::new(29, 34, &get_import_path()),
                    Keyword::Array,
                )),
                of: Box::new(KeywordTerm::new(
                    Location::new(35, 37, &get_import_path()),
                    Keyword::Of,
                )),
            }),
            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                Location::new(38, 44, &get_import_path()),
                "String",
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(44, 45, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn build_many_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 40, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 13, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 25, &get_import_path()),
            "ManyExample",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(26, 27, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(28, 39, &get_import_path()),
            Some(ArrayPrefix::Many(Box::new(KeywordTerm::new(
                Location::new(28, 32, &get_import_path()),
                Keyword::Many,
            )))),
            Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                Location::new(33, 39, &get_import_path()),
                "String",
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(39, 40, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn build_structured_type_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 33, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 13, &get_import_path()),
            Keyword::Type,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 27, &get_import_path()),
            "StructExample",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(28, 29, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        Box::new(TypeReferenceTerm::new(
            Location::new(30, 32, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Structured(StructureTerm::new(
                Location::new(30, 32, &get_import_path()),
                Box::new(PunctuationSignTerm::new(
                    Location::new(30, 31, &get_import_path()),
                    PunctuationSign::OpenCurlyBrace,
                )),
                vec![],
                Box::new(PunctuationSignTerm::new(
                    Location::new(31, 32, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                )),
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(32, 33, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn build_structured_type_with_single_field_declaration() -> TypeDeclarationTerm {
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
fn build_structured_type_with_multiple_fields_declaration() -> TypeDeclarationTerm {
    TypeDeclarationTerm::new(
        Location::new(9, 114, &get_import_path()),
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
            Location::new(23, 113, &get_import_path()),
            None,
            Box::new(TypeDetailsVariant::Structured(StructureTerm::new(
                Location::new(23, 113, &get_import_path()),
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
                ],
                Box::new(PunctuationSignTerm::new(
                    Location::new(112, 113, &get_import_path()),
                    PunctuationSign::CloseCurlyBrace,
                )),
            ))),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(113, 114, &get_import_path()),
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
