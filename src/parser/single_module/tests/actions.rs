use crate::ast::*;
use crate::parser::fs::MockInMemoryFileSystem;
use crate::parser::single_module::{SingleModuleParser, SingleModuleParserImpl};
use crate::parser::ParseError;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[test]
fn with_action_with_one_param_and_no_returns_it_parses() {
    let source = "
        service Example {
            action example(param: Example);
        }
    ";

    let result = parse_single_file(&source);

    expect_action_to_be(
        result,
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
        ),
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
