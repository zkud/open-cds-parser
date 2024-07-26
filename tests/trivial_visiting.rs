use open_cds_parser::ast::{
    ASTTerm, EntityTerm, FieldTerm, ModuleDefinition, ModuleTerm, NameTerm, ServiceDefinition,
    ServiceTerm,
};
use open_cds_parser::parser::Parser;
use open_cds_parser::visitor::{Visitor, VisitorError};

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
    fn process_entity(&mut self, term: &EntityTerm) -> Result<(), VisitorError> {
        let name = term.name().value().to_string();
        let fields = term
            .fields()
            .iter()
            .map(|f| Field {
                name: f.name().value().to_string(),
                field_type: f.type_name().value().to_string(),
            })
            .collect();
        self.tables.push(Table { name, fields });
        Ok(())
    }
}

#[test]
fn with_correct_and_trivial_cds_it_buids_a_tree() {
    let parser = Parser::new();

    let ast = parser
        .parse_single_file(&"./tests/projects/trivial/srv/cat-service.cds")
        .unwrap();
    let mut sql_visitor = SQLVisitor { tables: vec![] };
    ast.accept(&mut sql_visitor).unwrap();
}
