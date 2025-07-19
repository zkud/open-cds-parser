use std::path::Path;

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

mod util;

use util::get_type_name;

struct SQLVisitor {
    pub tables: Vec<Table>,
}

struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

struct Field {
    pub name: String,
    pub field_type: String,
}

impl Visitor for SQLVisitor {
    type Error = ();

    fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ()> {
        if let Some(term) = term.try_convert::<EntityDeclarationTerm>() {
            let name = term.identifier().full_name();
            let fields = term
                .structure()
                .fields()
                .iter()
                .map(|f| Field {
                    name: f.name().full_name(),
                    field_type: get_type_name(&f),
                })
                .collect();
            self.tables.push(Table { name, fields });
        }
        Ok(())
    }
}

#[test]
fn with_correct_and_trivial_cds_it_buids_a_tree() {
    let parser = Parser::new_with_native_fs();

    let path = Path::new("./tests/projects/trivial/srv/cat-service.cds");
    let ast = parser.parse_single_module(path).unwrap();
    let mut sql_visitor = SQLVisitor { tables: vec![] };
    ast.accept(&mut sql_visitor).unwrap();
}
