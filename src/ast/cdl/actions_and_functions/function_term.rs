use crate::ast::Location;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct FunctionTerm {
    #[prop]
    location: Location,
}
