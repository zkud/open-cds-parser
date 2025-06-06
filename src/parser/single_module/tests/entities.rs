use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn with_basic_entity_declaration_it_parses() {
    let source = "entity Example {};";

    let result = parse_single_file(&source);

    expect_entity_declaration_to_be(result, build_basic_entity_declaration());
}

#[test]
fn with_define_entity_declaration_it_parses() {
    let source = "define entity Example {};";

    let result = parse_single_file(&source);

    expect_entity_declaration_to_be(result, build_define_entity_declaration());
}

#[test]
fn with_entity_declaration_with_single_aspect_it_parses() {
    let source = "entity Example : Aspect1 {};";

    let result = parse_single_file(&source);

    expect_entity_declaration_to_be(result, build_entity_with_single_aspect_declaration());
}

#[test]
fn with_entity_declaration_with_multiple_aspects_it_parses() {
    let source = "entity Example : Aspect1, Aspect2 {};";

    let result = parse_single_file(&source);

    expect_entity_declaration_to_be(result, build_entity_with_multiple_aspects_declaration());
}

#[test]
fn with_entity_declaration_with_structure_it_parses() {
    let source = "
        entity Example {
            field1 : String;
        };
    ";

    let result = parse_single_file(&source);

    expect_entity_declaration_to_be(result, build_entity_with_structure_declaration());
}

#[test]
fn with_entity_declaration_with_multiple_fields_it_parses() {
    let source = "
        entity Example {
            id : UUID;
            name : String;
            age : Integer;
            email : String;
            isActive : Boolean;
        };
    ";

    let result = parse_single_file(&source);

    expect_entity_declaration_to_be(result, build_entity_with_multiple_fields_declaration());
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
fn build_basic_entity_declaration() -> EntityDeclarationTerm {
    EntityDeclarationTerm::new(
        Location::new(0, 18, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(0, 6, &get_import_path()),
            Keyword::Entity,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(7, 14, &get_import_path()),
            "Example",
        )),
        None,
        Vec::new(),
        Box::new(StructureTerm::new(
            Location::new(15, 17, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(15, 16, &get_import_path()),
                PunctuationSign::OpenCurlyBrace,
            )),
            Vec::new(),
            Box::new(PunctuationSignTerm::new(
                Location::new(16, 17, &get_import_path()),
                PunctuationSign::CloseCurlyBrace,
            )),
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(17, 18, &get_import_path()),
            PunctuationSign::Semicolumn,
        ))),
    )
}

#[inline]
fn build_define_entity_declaration() -> EntityDeclarationTerm {
    EntityDeclarationTerm::new(
        Location::new(0, 25, &get_import_path()),
        Some(Box::new(KeywordTerm::new(
            Location::new(0, 6, &get_import_path()),
            Keyword::Define,
        ))),
        Box::new(KeywordTerm::new(
            Location::new(7, 13, &get_import_path()),
            Keyword::Entity,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(14, 21, &get_import_path()),
            "Example",
        )),
        None,
        Vec::new(),
        Box::new(StructureTerm::new(
            Location::new(22, 24, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(22, 23, &get_import_path()),
                PunctuationSign::OpenCurlyBrace,
            )),
            Vec::new(),
            Box::new(PunctuationSignTerm::new(
                Location::new(23, 24, &get_import_path()),
                PunctuationSign::CloseCurlyBrace,
            )),
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(24, 25, &get_import_path()),
            PunctuationSign::Semicolumn,
        ))),
    )
}

#[inline]
fn build_entity_with_single_aspect_declaration() -> EntityDeclarationTerm {
    EntityDeclarationTerm::new(
        Location::new(0, 28, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(0, 6, &get_import_path()),
            Keyword::Entity,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(7, 14, &get_import_path()),
            "Example",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(15, 16, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        vec![AppliedAspectSegment::Aspect(IdentifierTerm::new_basic(
            Location::new(17, 24, &get_import_path()),
            "Aspect1",
        ))],
        Box::new(StructureTerm::new(
            Location::new(25, 27, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(25, 26, &get_import_path()),
                PunctuationSign::OpenCurlyBrace,
            )),
            Vec::new(),
            Box::new(PunctuationSignTerm::new(
                Location::new(26, 27, &get_import_path()),
                PunctuationSign::CloseCurlyBrace,
            )),
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(27, 28, &get_import_path()),
            PunctuationSign::Semicolumn,
        ))),
    )
}

#[inline]
fn build_entity_with_multiple_aspects_declaration() -> EntityDeclarationTerm {
    EntityDeclarationTerm::new(
        Location::new(0, 37, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(0, 6, &get_import_path()),
            Keyword::Entity,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(7, 14, &get_import_path()),
            "Example",
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(15, 16, &get_import_path()),
            PunctuationSign::Colon,
        ))),
        vec![
            AppliedAspectSegment::Aspect(IdentifierTerm::new_basic(
                Location::new(17, 24, &get_import_path()),
                "Aspect1",
            )),
            AppliedAspectSegment::Comma(PunctuationSignTerm::new(
                Location::new(24, 25, &get_import_path()),
                PunctuationSign::Comma,
            )),
            AppliedAspectSegment::Aspect(IdentifierTerm::new_basic(
                Location::new(26, 33, &get_import_path()),
                "Aspect2",
            )),
        ],
        Box::new(StructureTerm::new(
            Location::new(34, 36, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(34, 35, &get_import_path()),
                PunctuationSign::OpenCurlyBrace,
            )),
            Vec::new(),
            Box::new(PunctuationSignTerm::new(
                Location::new(35, 36, &get_import_path()),
                PunctuationSign::CloseCurlyBrace,
            )),
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(36, 37, &get_import_path()),
            PunctuationSign::Semicolumn,
        ))),
    )
}

#[inline]
fn build_entity_with_structure_declaration() -> EntityDeclarationTerm {
    EntityDeclarationTerm::new(
        Location::new(9, 65, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 15, &get_import_path()),
            Keyword::Entity,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(16, 23, &get_import_path()),
            "Example",
        )),
        None,
        Vec::new(),
        Box::new(StructureTerm::new(
            Location::new(24, 64, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(24, 25, &get_import_path()),
                PunctuationSign::OpenCurlyBrace,
            )),
            vec![FieldTerm::new(
                Location::new(38, 54, &get_import_path()),
                Box::new(IdentifierTerm::new_basic(
                    Location::new(38, 44, &get_import_path()),
                    "field1",
                )),
                Box::new(PunctuationSignTerm::new(
                    Location::new(45, 46, &get_import_path()),
                    PunctuationSign::Colon,
                )),
                Box::new(TypeReferenceTerm::new(
                    Location::new(47, 53, &get_import_path()),
                    None,
                    Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                        Location::new(47, 53, &get_import_path()),
                        "String",
                    ))),
                )),
                Box::new(PunctuationSignTerm::new(
                    Location::new(53, 54, &get_import_path()),
                    PunctuationSign::Semicolumn,
                )),
            )],
            Box::new(PunctuationSignTerm::new(
                Location::new(63, 64, &get_import_path()),
                PunctuationSign::CloseCurlyBrace,
            )),
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(64, 65, &get_import_path()),
            PunctuationSign::Semicolumn,
        ))),
    )
}

#[inline]
fn build_entity_with_multiple_fields_declaration() -> EntityDeclarationTerm {
    EntityDeclarationTerm::new(
        Location::new(9, 173, &get_import_path()),
        None,
        Box::new(KeywordTerm::new(
            Location::new(9, 15, &get_import_path()),
            Keyword::Entity,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(16, 23, &get_import_path()),
            "Example",
        )),
        None,
        Vec::new(),
        Box::new(StructureTerm::new(
            Location::new(24, 172, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(24, 25, &get_import_path()),
                PunctuationSign::OpenCurlyBrace,
            )),
            vec![
                FieldTerm::new(
                    Location::new(38, 48, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(38, 40, &get_import_path()),
                        "id",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(41, 42, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(43, 47, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(43, 47, &get_import_path()),
                            "UUID",
                        ))),
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(47, 48, &get_import_path()),
                        PunctuationSign::Semicolumn,
                    )),
                ),
                FieldTerm::new(
                    Location::new(61, 75, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(61, 65, &get_import_path()),
                        "name",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(66, 67, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(68, 74, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(68, 74, &get_import_path()),
                            "String",
                        ))),
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(74, 75, &get_import_path()),
                        PunctuationSign::Semicolumn,
                    )),
                ),
                FieldTerm::new(
                    Location::new(88, 102, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(88, 91, &get_import_path()),
                        "age",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(92, 93, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(94, 101, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(94, 101, &get_import_path()),
                            "Integer",
                        ))),
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(101, 102, &get_import_path()),
                        PunctuationSign::Semicolumn,
                    )),
                ),
                FieldTerm::new(
                    Location::new(115, 130, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(115, 120, &get_import_path()),
                        "email",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(121, 122, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(123, 129, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(123, 129, &get_import_path()),
                            "String",
                        ))),
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(129, 130, &get_import_path()),
                        PunctuationSign::Semicolumn,
                    )),
                ),
                FieldTerm::new(
                    Location::new(143, 162, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(143, 151, &get_import_path()),
                        "isActive",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(152, 153, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(154, 161, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(154, 161, &get_import_path()),
                            "Boolean",
                        ))),
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(161, 162, &get_import_path()),
                        PunctuationSign::Semicolumn,
                    )),
                ),
            ],
            Box::new(PunctuationSignTerm::new(
                Location::new(171, 172, &get_import_path()),
                PunctuationSign::CloseCurlyBrace,
            )),
        )),
        Some(Box::new(PunctuationSignTerm::new(
            Location::new(172, 173, &get_import_path()),
            PunctuationSign::Semicolumn,
        ))),
    )
}

#[inline]
fn expect_entity_declaration_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    expected_entity: EntityDeclarationTerm,
) {
    let module_to_check = module_to_check.expect("Unexpected Error");
    assert_eq!(module_to_check.definitions().len(), 1);
    let entity_decl = module_to_check
        .definitions()
        .first()
        .expect("Unable to retrieve entity declaration");
    let entity_decl = if let ModuleDefinition::Entity(entity_decl) = entity_decl {
        entity_decl
    } else {
        panic!("Expected entity declaration but got something else");
    };
    assert_eq!(entity_decl, &expected_entity);
}
