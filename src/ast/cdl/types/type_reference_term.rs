use super::super::IdentifierTerm;
use ast_term_derive::ASTTerm;

// Temporaral solution to remove ReturnsTerm
#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeReferenceTerm {
    #[subnode_prop]
    type_name: Box<IdentifierTerm>,
    #[prop]
    is_arrayed: bool,
}

impl TypeReferenceTerm {
    pub fn new_scalar(type_name: Box<IdentifierTerm>) -> TypeReferenceTerm {
        TypeReferenceTerm {
            type_name,
            is_arrayed: false,
        }
    }

    pub fn new_arrayed(type_name: Box<IdentifierTerm>) -> TypeReferenceTerm {
        TypeReferenceTerm {
            type_name,
            is_arrayed: true,
        }
    }
}
