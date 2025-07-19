use std::path::Path;

use open_cds_parser::ast::*;
use open_cds_parser::parser::Parser;

mod util;

use util::get_type_name;

struct CounterVisitor {
    pub namespaces_count: u8,
    pub types_count: u8,
}

impl Visitor for CounterVisitor {
    type Error = ();

    fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ()> {
        if let Some(term) = term.try_convert::<NamespaceDeclarationTerm>() {
            self.namespaces_count += 1;
        }
        if let Some(term) = term.try_convert::<TypeDeclarationTerm>() {
            self.types_count += 1;
        }
        Ok(())
    }
}

#[test]
fn with_correct_and_trivial_cds_it_visits_it_correctly() {
    let parser = Parser::new_with_native_fs();
    let path = Path::new("./tests/projects/trivial/srv/books-service.cds");

    let ast = parser.parse_single_module(path).unwrap();
    let mut visitor = CounterVisitor {
        namespaces_count: 0,
        types_count: 0,
    };
    ast.accept(&mut visitor).unwrap();

    assert_eq!(visitor.namespaces_count, 1);
    assert_eq!(visitor.types_count, 1);
}
