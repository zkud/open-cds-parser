use std::path::Path;

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

#[test]
fn with_correct_and_trivial_cds_it_buids_a_tree() {
    let parser = Parser::new_with_native_fs();

    let path = Path::new("./tests/projects/trivial/srv/cat-service.cds");
    let ast = parser.parse_single_module(path).unwrap();

    let expected_ast = Box::new(ModuleTerm::new(
        Location::new(0, 0, &path),
        vec![ModuleDefinition::Service(ServiceTerm::new(
            Location::new(0, 0, &path),
            Box::new(IdentifierTerm::new_basic(
                Location::new(8, 22, &path),
                "CatalogService",
            )),
            vec![
                ServiceDefinition::Entity(EntityTerm::new(
                    Location::new(0, 0, &path),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(35, 45, &path),
                        "UserScopes",
                    )),
                    vec![],
                    vec![
                        FieldTerm::new(
                            Location::new(0, 0, &path),
                            Box::new(IdentifierTerm::new_basic(
                                Location::new(53, 61, &path),
                                "username",
                            )),
                            Box::new(IdentifierTerm::new_basic(
                                Location::new(64, 70, &path),
                                "String",
                            )),
                        ),
                        FieldTerm::new(
                            Location::new(0, 0, &path),
                            Box::new(IdentifierTerm::new_basic(
                                Location::new(77, 82, &path),
                                "scope",
                            )),
                            Box::new(IdentifierTerm::new_basic(
                                Location::new(88, 94, &path),
                                "String",
                            )),
                        ),
                    ],
                )),
                ServiceDefinition::Function(FunctionDeclarationTerm::new(
                    Location::new(106, 152, path),
                    Box::new(FunctionTerm::new(Location::new(106, 114, path))),
                    Box::new(IdentifierTerm::new_basic(
                        Location::new(115, 133, &path),
                        "getUserScopesCount",
                    )),
                    Box::new(ParametersBlockTerm::new(
                        Location::new(133, 135, path),
                        Box::new(OpenRoundBracketTerm::new(Location::new(133, 134, path))),
                        vec![],
                        Box::new(CloseRoundBracketTerm::new(Location::new(134, 135, path))),
                    )),
                    Box::new(ReturnsDeclarationTerm::new(
                        Location::new(136, 151, path),
                        Box::new(ReturnsTerm::new(Location::new(136, 143, path))),
                        Box::new(TypeReferenceTerm::new_scalar(
                            Location::new(144, 151, &path),
                            Box::new(IdentifierTerm::new_basic(
                                Location::new(144, 151, &path),
                                "Integer",
                            )),
                        )),
                    )),
                    Box::new(SemicolumnTerm::new(Location::new(151, 152, path))),
                )),
            ],
        ))],
    ));
    assert_eq!(ast, expected_ast);
}
