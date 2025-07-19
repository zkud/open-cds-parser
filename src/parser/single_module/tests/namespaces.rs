use super::utils::{get_mock_path, parse_single_file};
use crate::ast::*;
use crate::parser::ParseError;

#[test]
fn with_typical_namespace_declaration_it_parses() {
    let cdl = "namespace typical;";

    let result = parse_single_file(&cdl);

    expect_namespace_to_be(result, get_typical_namespace());
}

fn get_typical_namespace() -> NamespaceDeclarationTerm {
    NamespaceDeclarationTerm::new(
        Location::new(0, 18, &get_mock_path()),
        Box::new(KeywordTerm::new(
            Location::new(0, 9, &get_mock_path()),
            Keyword::Namespace,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(10, 17, &get_mock_path()),
            "typical",
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(17, 18, &get_mock_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

fn expect_namespace_to_be(
    parsed_module: Result<Box<ModuleTerm>, ParseError>,
    expected_namespace: NamespaceDeclarationTerm,
) {
    let module = parsed_module.expect("Parsing is failing");
    assert_eq!(module.definitions().len(), 1);
    let namespace = module
        .definitions()
        .get(0)
        .expect("Unable to retrieve namespace");
    let namespace = if let ModuleDefinition::Namespace(namespace) = namespace {
        namespace
    } else {
        panic!("Missed namespace")
    };
    assert_eq!(namespace, &expected_namespace);
}
