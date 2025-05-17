use open_cds_parser::ast::*;

#[inline]
pub fn get_type_name(field: &FieldTerm) -> String {
    if let TypeDetailsVariant::Simple(type_name) = field.type_name().type_details().as_ref() {
        type_name.full_name()
    } else {
        panic!("Cannot extract the field");
    }
}
