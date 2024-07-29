use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_open_figure_bracket")]
pub struct OpenFigureBracketTerm {}
