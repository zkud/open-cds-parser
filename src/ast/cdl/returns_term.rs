use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ReturnsTerm {
    #[subnode_prop]
    type_name: Box<NameTerm>,
    #[prop]
    is_arrayed: bool,
}

impl ReturnsTerm {
    pub fn new_scalar(type_name: Box<NameTerm>) -> ReturnsTerm {
        ReturnsTerm {
            type_name,
            is_arrayed: false,
        }
    }

    pub fn new_arrayed(type_name: Box<NameTerm>) -> ReturnsTerm {
        ReturnsTerm {
            type_name,
            is_arrayed: true,
        }
    }
}
