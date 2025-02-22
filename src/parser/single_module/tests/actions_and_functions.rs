use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn with_action_with_one_param_and_no_returns_it_parses() {
    let source = "
        service Example {
            action example(param: Example);
        }
    ";

    let result = parse_single_file(&source);

    expect_action_to_be(result, build_basic_action());
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
fn build_basic_action() -> ActionDeclarationTerm {
    ActionDeclarationTerm::new(
        Location::new(39, 70, &get_import_path()),
        Box::new(ActionTerm::new(Location::new(39, 45, &get_import_path()))),
        Box::new(NameTerm::new("example".to_string())),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 69, &get_import_path()),
            Box::new(OpenRoundBracketTerm::new(Location::new(
                53,
                54,
                &get_import_path(),
            ))),
            vec![ParameterOrComma::Parameter(ParamTerm::new(
                Location::new(54, 68, &get_import_path()),
                Box::new(NameTerm::new("param".to_string())),
                Box::new(ColonTerm::new(Location::new(59, 60, &get_import_path()))),
                Box::new(TypeReferenceTerm::new_scalar(Box::new(NameTerm::new(
                    "Example".to_string(),
                )))),
            ))],
            Box::new(CloseRoundBracketTerm::new(Location::new(
                68,
                69,
                &get_import_path(),
            ))),
        )),
        None,
        Box::new(SemicolumnTerm::new(Location::new(
            69,
            70,
            &get_import_path(),
        ))),
    )
}

#[inline]
fn expect_action_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    action: ActionDeclarationTerm,
) {
    let module_to_check = module_to_check.expect("Unexpected Error");
    assert_eq!(
        module_to_check.deref(),
        &ModuleTerm::new(vec![ModuleDefinition::Service(ServiceTerm::new(
            Box::new(NameTerm::new("Example".to_string())),
            vec![ServiceDefinition::Action(action)]
        ))])
    );
}

#[test]
fn with_action_with_one_param_and_one_return_it_parses() {
    let source = "
        service Example {
            action example(param: Example) returns Example;
        }
    ";

    let result = parse_single_file(&source);

    expect_action_to_be(result, build_basic_action_plus_return());
}

#[inline]
fn build_basic_action_plus_return() -> ActionDeclarationTerm {
    ActionDeclarationTerm::new(
        Location::new(39, 86, &get_import_path()),
        Box::new(ActionTerm::new(Location::new(39, 45, &get_import_path()))),
        Box::new(NameTerm::new("example".to_string())),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 69, &get_import_path()),
            Box::new(OpenRoundBracketTerm::new(Location::new(
                53,
                54,
                &get_import_path(),
            ))),
            vec![ParameterOrComma::Parameter(ParamTerm::new(
                Location::new(54, 68, &get_import_path()),
                Box::new(NameTerm::new("param".to_string())),
                Box::new(ColonTerm::new(Location::new(59, 60, &get_import_path()))),
                Box::new(TypeReferenceTerm::new_scalar(Box::new(NameTerm::new(
                    "Example".to_string(),
                )))),
            ))],
            Box::new(CloseRoundBracketTerm::new(Location::new(
                68,
                69,
                &get_import_path(),
            ))),
        )),
        Some(Box::new(ReturnsDeclarationTerm::new(
            Location::new(70, 85, &get_import_path()),
            Box::new(ReturnsTerm::new(Location::new(70, 77, &get_import_path()))),
            Box::new(TypeReferenceTerm::new_scalar(Box::new(NameTerm::new(
                "Example".to_string(),
            )))),
        ))),
        Box::new(SemicolumnTerm::new(Location::new(
            85,
            86,
            &get_import_path(),
        ))),
    )
}

#[test]
fn with_action_with_several_params_it_parses() {
    let source = "
        service Example {
            action example(param1: Example1, param2: Example2);
        }
    ";

    let result = parse_single_file(&source);

    expect_action_to_be(result, build_basic_action_with_several_args());
}

#[inline]
fn build_basic_action_with_several_args() -> ActionDeclarationTerm {
    ActionDeclarationTerm::new(
        Location::new(39, 90, &get_import_path()),
        Box::new(ActionTerm::new(Location::new(39, 45, &get_import_path()))),
        Box::new(NameTerm::new("example".to_string())),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 89, &get_import_path()),
            Box::new(OpenRoundBracketTerm::new(Location::new(
                53,
                54,
                &get_import_path(),
            ))),
            vec![
                ParameterOrComma::Parameter(ParamTerm::new(
                    Location::new(54, 70, &get_import_path()),
                    Box::new(NameTerm::new("param1".to_string())),
                    Box::new(ColonTerm::new(Location::new(60, 61, &get_import_path()))),
                    Box::new(TypeReferenceTerm::new_scalar(Box::new(NameTerm::new(
                        "Example1".to_string(),
                    )))),
                )),
                ParameterOrComma::Comma(CommaTerm::new(Location::new(70, 71, &get_import_path()))),
                ParameterOrComma::Parameter(ParamTerm::new(
                    Location::new(72, 88, &get_import_path()),
                    Box::new(NameTerm::new("param2".to_string())),
                    Box::new(ColonTerm::new(Location::new(78, 79, &get_import_path()))),
                    Box::new(TypeReferenceTerm::new_scalar(Box::new(NameTerm::new(
                        "Example2".to_string(),
                    )))),
                )),
            ],
            Box::new(CloseRoundBracketTerm::new(Location::new(
                88,
                89,
                &get_import_path(),
            ))),
        )),
        None,
        Box::new(SemicolumnTerm::new(Location::new(
            89,
            90,
            &get_import_path(),
        ))),
    )
}

#[test]
fn with_action_with_no_params_it_parses() {
    let source = "
        service Example {
            action example();
        }
    ";

    let result = parse_single_file(&source);

    expect_action_to_be(result, build_basic_action_with_no_args());
}

#[inline]
fn build_basic_action_with_no_args() -> ActionDeclarationTerm {
    ActionDeclarationTerm::new(
        Location::new(39, 56, &get_import_path()),
        Box::new(ActionTerm::new(Location::new(39, 45, &get_import_path()))),
        Box::new(NameTerm::new("example".to_string())),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 55, &get_import_path()),
            Box::new(OpenRoundBracketTerm::new(Location::new(
                53,
                54,
                &get_import_path(),
            ))),
            vec![],
            Box::new(CloseRoundBracketTerm::new(Location::new(
                54,
                55,
                &get_import_path(),
            ))),
        )),
        None,
        Box::new(SemicolumnTerm::new(Location::new(
            55,
            56,
            &get_import_path(),
        ))),
    )
}

#[test]
fn with_most_basic_function_it_parses() {
    let source = "
        service Example {
            function example() returns Example;
        }
    ";

    let result = parse_single_file(&source);

    expect_function_to_be(result, build_the_most_simple_function());
}

#[inline]
fn build_the_most_simple_function() -> FunctionDeclarationTerm {
    FunctionDeclarationTerm::new(
        Location::new(39, 74, &get_import_path()),
        Box::new(FunctionTerm::new(Location::new(39, 47, &get_import_path()))),
        Box::new(NameTerm::new("example".to_string())),
        Box::new(ParametersBlockTerm::new(
            Location::new(55, 57, &get_import_path()),
            Box::new(OpenRoundBracketTerm::new(Location::new(
                55,
                56,
                &get_import_path(),
            ))),
            vec![],
            Box::new(CloseRoundBracketTerm::new(Location::new(
                56,
                57,
                &get_import_path(),
            ))),
        )),
        Box::new(ReturnsDeclarationTerm::new(
            Location::new(58, 73, &get_import_path()),
            Box::new(ReturnsTerm::new(Location::new(58, 65, &get_import_path()))),
            Box::new(TypeReferenceTerm::new_scalar(Box::new(NameTerm::new(
                "Example".to_string(),
            )))),
        )),
        Box::new(SemicolumnTerm::new(Location::new(
            73,
            74,
            &get_import_path(),
        ))),
    )
}

#[inline]
fn expect_function_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    function: FunctionDeclarationTerm,
) {
    let module_to_check = module_to_check.expect("Unexpected Error");
    assert_eq!(
        module_to_check.deref(),
        &ModuleTerm::new(vec![ModuleDefinition::Service(ServiceTerm::new(
            Box::new(NameTerm::new("Example".to_string())),
            vec![ServiceDefinition::Function(function)]
        ))])
    );
}
