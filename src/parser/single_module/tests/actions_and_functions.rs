use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
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
        Box::new(KeywordTerm::new(
            Location::new(39, 45, &get_import_path()),
            Keyword::Action,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(46, 53, &get_import_path()),
            "example",
        )),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 69, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(53, 54, &get_import_path()),
                PunctuationSign::OpenRoundBracket,
            )),
            vec![ParameterOrComma::Parameter(ParamTerm::new(
                Location::new(54, 68, &get_import_path()),
                Box::new(IdentifierTerm::new_basic(
                    Location::new(54, 59, &get_import_path()),
                    "param",
                )),
                Box::new(PunctuationSignTerm::new(
                    Location::new(59, 60, &get_import_path()),
                    PunctuationSign::Colon,
                )),
                Box::new(TypeReferenceTerm::new(
                    Location::new(61, 68, &get_import_path()),
                    None,
                    Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                        Location::new(61, 68, &get_import_path()),
                        "Example",
                    ))),
                )),
            ))],
            Box::new(PunctuationSignTerm::new(
                Location::new(68, 69, &get_import_path()),
                PunctuationSign::CloseRoundBracket,
            )),
        )),
        None,
        Box::new(PunctuationSignTerm::new(
            Location::new(69, 70, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn expect_action_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    expected_action: ActionDeclarationTerm,
) {
    let parsed_module = module_to_check.unwrap();
    if let ModuleDefinition::Service(service) = &parsed_module.definitions()[0] {
        if let ServiceDefinition::Action(action) = &service.definitions()[0] {
            assert_eq!(*action, expected_action);
            return;
        }
    }
    panic!("Action not found!");
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
        Box::new(KeywordTerm::new(
            Location::new(39, 45, &get_import_path()),
            Keyword::Action,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(46, 53, &get_import_path()),
            "example",
        )),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 69, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(53, 54, &get_import_path()),
                PunctuationSign::OpenRoundBracket,
            )),
            vec![ParameterOrComma::Parameter(ParamTerm::new(
                Location::new(54, 68, &get_import_path()),
                Box::new(IdentifierTerm::new_basic(
                    Location::new(54, 59, &get_import_path()),
                    "param",
                )),
                Box::new(PunctuationSignTerm::new(
                    Location::new(59, 60, &get_import_path()),
                    PunctuationSign::Colon,
                )),
                Box::new(TypeReferenceTerm::new(
                    Location::new(61, 68, &get_import_path()),
                    None,
                    Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                        Location::new(61, 68, &get_import_path()),
                        "Example",
                    ))),
                )),
            ))],
            Box::new(PunctuationSignTerm::new(
                Location::new(68, 69, &get_import_path()),
                PunctuationSign::CloseRoundBracket,
            )),
        )),
        Some(Box::new(ReturnsDeclarationTerm::new(
            Location::new(70, 85, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(70, 77, &get_import_path()),
                Keyword::Returns,
            )),
            Box::new(TypeReferenceTerm::new(
                Location::new(78, 85, &get_import_path()),
                None,
                Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                    Location::new(78, 85, &get_import_path()),
                    "Example",
                ))),
            )),
        ))),
        Box::new(PunctuationSignTerm::new(
            Location::new(85, 86, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
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
        Box::new(KeywordTerm::new(
            Location::new(39, 45, &get_import_path()),
            Keyword::Action,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(46, 53, &get_import_path()),
            "example",
        )),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 89, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(53, 54, &get_import_path()),
                PunctuationSign::OpenRoundBracket,
            )),
            vec![
                ParameterOrComma::Parameter(ParamTerm::new(
                    Location::new(54, 70, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(54, 60, &get_import_path()),
                        "param1",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(60, 61, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(62, 70, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(62, 70, &get_import_path()),
                            "Example1",
                        ))),
                    )),
                )),
                ParameterOrComma::Comma(PunctuationSignTerm::new(
                    Location::new(70, 71, &get_import_path()),
                    PunctuationSign::Comma,
                )),
                ParameterOrComma::Parameter(ParamTerm::new(
                    Location::new(72, 88, &get_import_path()),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(72, 78, &get_import_path()),
                        "param2",
                    )),
                    Box::new(PunctuationSignTerm::new(
                        Location::new(78, 79, &get_import_path()),
                        PunctuationSign::Colon,
                    )),
                    Box::new(TypeReferenceTerm::new(
                        Location::new(80, 88, &get_import_path()),
                        None,
                        Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                            Location::new(80, 88, &get_import_path()),
                            "Example2",
                        ))),
                    )),
                )),
            ],
            Box::new(PunctuationSignTerm::new(
                Location::new(88, 89, &get_import_path()),
                PunctuationSign::CloseRoundBracket,
            )),
        )),
        None,
        Box::new(PunctuationSignTerm::new(
            Location::new(89, 90, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
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
        Box::new(KeywordTerm::new(
            Location::new(39, 45, &get_import_path()),
            Keyword::Action,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(46, 53, &get_import_path()),
            "example",
        )),
        Box::new(ParametersBlockTerm::new(
            Location::new(53, 55, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(53, 54, &get_import_path()),
                PunctuationSign::OpenRoundBracket,
            )),
            vec![],
            Box::new(PunctuationSignTerm::new(
                Location::new(54, 55, &get_import_path()),
                PunctuationSign::CloseRoundBracket,
            )),
        )),
        None,
        Box::new(PunctuationSignTerm::new(
            Location::new(55, 56, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
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
        Box::new(KeywordTerm::new(
            Location::new(39, 47, &get_import_path()),
            Keyword::Function,
        )),
        Box::new(IdentifierTerm::new_basic(
            Location::new(48, 55, &get_import_path()),
            "example",
        )),
        Box::new(ParametersBlockTerm::new(
            Location::new(55, 57, &get_import_path()),
            Box::new(PunctuationSignTerm::new(
                Location::new(55, 56, &get_import_path()),
                PunctuationSign::OpenRoundBracket,
            )),
            vec![],
            Box::new(PunctuationSignTerm::new(
                Location::new(56, 57, &get_import_path()),
                PunctuationSign::CloseRoundBracket,
            )),
        )),
        Box::new(ReturnsDeclarationTerm::new(
            Location::new(58, 73, &get_import_path()),
            Box::new(KeywordTerm::new(
                Location::new(58, 65, &get_import_path()),
                Keyword::Returns,
            )),
            Box::new(TypeReferenceTerm::new(
                Location::new(66, 73, &get_import_path()),
                None,
                Box::new(TypeDetailsVariant::Simple(IdentifierTerm::new_basic(
                    Location::new(66, 73, &get_import_path()),
                    "Example",
                ))),
            )),
        )),
        Box::new(PunctuationSignTerm::new(
            Location::new(73, 74, &get_import_path()),
            PunctuationSign::Semicolumn,
        )),
    )
}

#[inline]
fn expect_function_to_be(
    module_to_check: Result<Box<ModuleTerm>, ParseError>,
    expected_function: FunctionDeclarationTerm,
) {
    let parsed_module = module_to_check.unwrap();
    if let ModuleDefinition::Service(service) = &parsed_module.definitions()[0] {
        if let ServiceDefinition::Function(function) = &service.definitions()[0] {
            assert_eq!(*function, expected_function);
            return;
        }
    }
    panic!("Function not found!");
}

#[test]
fn with_function_without_returns_it_fails() {
    let source = "
        service Example {
            function example();
        }
    ";

    let result = parse_single_file(&source);

    assert!(result.is_err());
}

struct ParamCaptureVisitor {
    params: Vec<(String, String)>,
    comma_count: i32,
}

impl Visitor for ParamCaptureVisitor {
    type Error = ();

    fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), Self::Error> {
        if let Some(term) = term.try_convert::<ParamTerm>() {
            let name = term.name().full_name();
            let param_type = Self::extract_full_name(term);
            self.params.push((name.clone(), param_type.clone()));
        }
        if let Some(term) = term.try_convert::<PunctuationSignTerm>() {
            if term.sign() != &PunctuationSign::Comma {
                return Ok(());
            }
            if self.params.is_empty() {
                panic!("Unexpected comma order!!!");
            }
            self.comma_count += 1;
        }
        Ok(())
    }
}

impl ParamCaptureVisitor {
    fn extract_full_name(param: &ParamTerm) -> String {
        if let TypeDetailsVariant::Simple(type_name) =
            param.type_reference().type_details().as_ref()
        {
            type_name.full_name()
        } else {
            panic!("Unexpected TypeDetailsVariant");
        }
    }
}

#[test]
fn with_visiting_it_visits_correctly() {
    let source = "
        service Example {
            function example(param1: Type1, param2: Type2) returns Example;
        }
    ";
    let mut visitor = ParamCaptureVisitor {
        params: vec![],
        comma_count: 0,
    };

    let result = parse_single_file(&source);
    result
        .expect("Expected success")
        .accept(&mut visitor)
        .expect("Expected success");

    assert_eq!(
        visitor.params,
        vec![
            ("param1".to_string(), "Type1".to_string()),
            ("param2".to_string(), "Type2".to_string())
        ]
    );
}
