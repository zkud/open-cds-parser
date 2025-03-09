use std::path::PathBuf;

use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct PathTerm {
    #[prop]
    location: Location,
    #[prop]
    value: PathBuf,
}
