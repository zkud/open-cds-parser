#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod ast {
    mod cdl {
        pub mod action_term {
            use super::name_term::NameTerm;
            use super::param_term::ParamTerm;
            use super::returns_term::ReturnsTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_action")]
            pub struct ActionTerm {
                #[subnode_prop]
                name: Box<NameTerm>,
                #[subnode_prop]
                params: Vec<Box<ParamTerm>>,
                #[subnode_prop]
                returned_type: Option<Box<ReturnsTerm>>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ActionTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ActionTerm {
                #[inline]
                fn eq(&self, other: &ActionTerm) -> bool {
                    self.name == other.name && self.params == other.params
                        && self.returned_type == other.returned_type
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for ActionTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ActionTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Vec<Box<ParamTerm>>>;
                    let _: ::core::cmp::AssertParamIsEq<Option<Box<ReturnsTerm>>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ActionTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ActionTerm",
                        "name",
                        &&self.name,
                        "params",
                        &&self.params,
                        "returned_type",
                        &&self.returned_type,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for ActionTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_action(self)?;
                    self.name.accept(visitor)?;
                    self.params.accept(visitor)?;
                    self.returned_type.accept(visitor)?;
                    Ok(())
                }
            }
            impl ActionTerm {
                pub fn name(&self) -> &Box<NameTerm> {
                    &self.name
                }
            }
            impl ActionTerm {
                pub fn name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.name
                }
            }
            impl ActionTerm {
                pub fn set_name(&mut self, value: Box<NameTerm>) {
                    self.name = value;
                }
            }
            impl ActionTerm {
                pub fn params(&self) -> &Vec<Box<ParamTerm>> {
                    &self.params
                }
            }
            impl ActionTerm {
                pub fn params_mut(&mut self) -> &mut Vec<Box<ParamTerm>> {
                    &mut self.params
                }
            }
            impl ActionTerm {
                pub fn set_params(&mut self, value: Vec<Box<ParamTerm>>) {
                    self.params = value;
                }
            }
            impl ActionTerm {
                pub fn returned_type(&self) -> &Option<Box<ReturnsTerm>> {
                    &self.returned_type
                }
            }
            impl ActionTerm {
                pub fn returned_type_mut(&mut self) -> &mut Option<Box<ReturnsTerm>> {
                    &mut self.returned_type
                }
            }
            impl ActionTerm {
                pub fn set_returned_type(&mut self, value: Option<Box<ReturnsTerm>>) {
                    self.returned_type = value;
                }
            }
            impl ActionTerm {
                pub fn new_boxed(
                    name: Box<NameTerm>,
                    params: Vec<Box<ParamTerm>>,
                    returned_type: Option<Box<ReturnsTerm>>,
                ) -> ActionTerm {
                    Box::new(ActionTerm {
                        name,
                        params,
                        returned_type,
                    })
                }
                pub fn new(
                    name: Box<NameTerm>,
                    params: Vec<Box<ParamTerm>>,
                    returned_type: Option<Box<ReturnsTerm>>,
                ) -> ActionTerm {
                    ActionTerm {
                        name,
                        params,
                        returned_type,
                    }
                }
            }
        }
        pub mod entity_term {
            use super::super::common::reference::Reference;
            use super::field_term::FieldTerm;
            use super::name_term::NameTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_entity")]
            pub struct EntityTerm {
                #[subnode_prop]
                name: Box<NameTerm>,
                #[subnode_prop]
                applied_aspects: Vec<Reference<Box<NameTerm>>>,
                #[subnode_prop]
                fields: Vec<Box<FieldTerm>>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for EntityTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for EntityTerm {
                #[inline]
                fn eq(&self, other: &EntityTerm) -> bool {
                    self.name == other.name
                        && self.applied_aspects == other.applied_aspects
                        && self.fields == other.fields
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for EntityTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for EntityTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Vec<Reference<Box<NameTerm>>>>;
                    let _: ::core::cmp::AssertParamIsEq<Vec<Box<FieldTerm>>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for EntityTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "EntityTerm",
                        "name",
                        &&self.name,
                        "applied_aspects",
                        &&self.applied_aspects,
                        "fields",
                        &&self.fields,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for EntityTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_entity(self)?;
                    self.name.accept(visitor)?;
                    self.applied_aspects.accept(visitor)?;
                    self.fields.accept(visitor)?;
                    Ok(())
                }
            }
            impl EntityTerm {
                pub fn name(&self) -> &Box<NameTerm> {
                    &self.name
                }
            }
            impl EntityTerm {
                pub fn name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.name
                }
            }
            impl EntityTerm {
                pub fn set_name(&mut self, value: Box<NameTerm>) {
                    self.name = value;
                }
            }
            impl EntityTerm {
                pub fn applied_aspects(&self) -> &Vec<Reference<Box<NameTerm>>> {
                    &self.applied_aspects
                }
            }
            impl EntityTerm {
                pub fn applied_aspects_mut(
                    &mut self,
                ) -> &mut Vec<Reference<Box<NameTerm>>> {
                    &mut self.applied_aspects
                }
            }
            impl EntityTerm {
                pub fn set_applied_aspects(
                    &mut self,
                    value: Vec<Reference<Box<NameTerm>>>,
                ) {
                    self.applied_aspects = value;
                }
            }
            impl EntityTerm {
                pub fn fields(&self) -> &Vec<Box<FieldTerm>> {
                    &self.fields
                }
            }
            impl EntityTerm {
                pub fn fields_mut(&mut self) -> &mut Vec<Box<FieldTerm>> {
                    &mut self.fields
                }
            }
            impl EntityTerm {
                pub fn set_fields(&mut self, value: Vec<Box<FieldTerm>>) {
                    self.fields = value;
                }
            }
            impl EntityTerm {
                pub fn new_boxed(
                    name: Box<NameTerm>,
                    applied_aspects: Vec<Reference<Box<NameTerm>>>,
                    fields: Vec<Box<FieldTerm>>,
                ) -> EntityTerm {
                    Box::new(EntityTerm {
                        name,
                        applied_aspects,
                        fields,
                    })
                }
                pub fn new(
                    name: Box<NameTerm>,
                    applied_aspects: Vec<Reference<Box<NameTerm>>>,
                    fields: Vec<Box<FieldTerm>>,
                ) -> EntityTerm {
                    EntityTerm {
                        name,
                        applied_aspects,
                        fields,
                    }
                }
            }
        }
        pub mod field_term {
            use super::name_term::NameTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_field")]
            pub struct FieldTerm {
                #[subnode_prop]
                name: Box<NameTerm>,
                #[subnode_prop]
                type_name: Box<NameTerm>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for FieldTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for FieldTerm {
                #[inline]
                fn eq(&self, other: &FieldTerm) -> bool {
                    self.name == other.name && self.type_name == other.type_name
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for FieldTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for FieldTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FieldTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "FieldTerm",
                        "name",
                        &&self.name,
                        "type_name",
                        &&self.type_name,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for FieldTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_field(self)?;
                    self.name.accept(visitor)?;
                    self.type_name.accept(visitor)?;
                    Ok(())
                }
            }
            impl FieldTerm {
                pub fn name(&self) -> &Box<NameTerm> {
                    &self.name
                }
            }
            impl FieldTerm {
                pub fn name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.name
                }
            }
            impl FieldTerm {
                pub fn set_name(&mut self, value: Box<NameTerm>) {
                    self.name = value;
                }
            }
            impl FieldTerm {
                pub fn type_name(&self) -> &Box<NameTerm> {
                    &self.type_name
                }
            }
            impl FieldTerm {
                pub fn type_name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.type_name
                }
            }
            impl FieldTerm {
                pub fn set_type_name(&mut self, value: Box<NameTerm>) {
                    self.type_name = value;
                }
            }
            impl FieldTerm {
                pub fn new_boxed(
                    name: Box<NameTerm>,
                    type_name: Box<NameTerm>,
                ) -> FieldTerm {
                    Box::new(FieldTerm { name, type_name })
                }
                pub fn new(name: Box<NameTerm>, type_name: Box<NameTerm>) -> FieldTerm {
                    FieldTerm { name, type_name }
                }
            }
        }
        pub mod function_term {
            use super::name_term::NameTerm;
            use super::param_term::ParamTerm;
            use super::returns_term::ReturnsTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_function")]
            pub struct FunctionTerm {
                #[subnode_prop]
                name: Box<NameTerm>,
                #[subnode_prop]
                params: Vec<Box<ParamTerm>>,
                #[subnode_prop]
                returned_type: Box<ReturnsTerm>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for FunctionTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for FunctionTerm {
                #[inline]
                fn eq(&self, other: &FunctionTerm) -> bool {
                    self.name == other.name && self.params == other.params
                        && self.returned_type == other.returned_type
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for FunctionTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for FunctionTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Vec<Box<ParamTerm>>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<ReturnsTerm>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FunctionTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "FunctionTerm",
                        "name",
                        &&self.name,
                        "params",
                        &&self.params,
                        "returned_type",
                        &&self.returned_type,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for FunctionTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_function(self)?;
                    self.name.accept(visitor)?;
                    self.params.accept(visitor)?;
                    self.returned_type.accept(visitor)?;
                    Ok(())
                }
            }
            impl FunctionTerm {
                pub fn name(&self) -> &Box<NameTerm> {
                    &self.name
                }
            }
            impl FunctionTerm {
                pub fn name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.name
                }
            }
            impl FunctionTerm {
                pub fn set_name(&mut self, value: Box<NameTerm>) {
                    self.name = value;
                }
            }
            impl FunctionTerm {
                pub fn params(&self) -> &Vec<Box<ParamTerm>> {
                    &self.params
                }
            }
            impl FunctionTerm {
                pub fn params_mut(&mut self) -> &mut Vec<Box<ParamTerm>> {
                    &mut self.params
                }
            }
            impl FunctionTerm {
                pub fn set_params(&mut self, value: Vec<Box<ParamTerm>>) {
                    self.params = value;
                }
            }
            impl FunctionTerm {
                pub fn returned_type(&self) -> &Box<ReturnsTerm> {
                    &self.returned_type
                }
            }
            impl FunctionTerm {
                pub fn returned_type_mut(&mut self) -> &mut Box<ReturnsTerm> {
                    &mut self.returned_type
                }
            }
            impl FunctionTerm {
                pub fn set_returned_type(&mut self, value: Box<ReturnsTerm>) {
                    self.returned_type = value;
                }
            }
            impl FunctionTerm {
                pub fn new_boxed(
                    name: Box<NameTerm>,
                    params: Vec<Box<ParamTerm>>,
                    returned_type: Box<ReturnsTerm>,
                ) -> FunctionTerm {
                    Box::new(FunctionTerm {
                        name,
                        params,
                        returned_type,
                    })
                }
                pub fn new(
                    name: Box<NameTerm>,
                    params: Vec<Box<ParamTerm>>,
                    returned_type: Box<ReturnsTerm>,
                ) -> FunctionTerm {
                    FunctionTerm {
                        name,
                        params,
                        returned_type,
                    }
                }
            }
        }
        pub mod module_term {
            use super::super::super::visitor::{Visitor, VisitorError};
            use super::super::common::ast_term::ASTTerm;
            use super::entity_term::EntityTerm;
            use super::service_term::ServiceTerm;
            use super::type_term::TypeTerm;
            pub struct ModuleTerm {
                definitions: Vec<ModuleDefinition>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ModuleTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ModuleTerm {
                #[inline]
                fn eq(&self, other: &ModuleTerm) -> bool {
                    self.definitions == other.definitions
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for ModuleTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ModuleTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Vec<ModuleDefinition>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ModuleTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ModuleTerm",
                        "definitions",
                        &&self.definitions,
                    )
                }
            }
            pub enum ModuleDefinition {
                Entity(Box<EntityTerm>),
                Type(Box<TypeTerm>),
                Service(Box<ServiceTerm>),
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ModuleDefinition {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ModuleDefinition {
                #[inline]
                fn eq(&self, other: &ModuleDefinition) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                            (
                                ModuleDefinition::Entity(__self_0),
                                ModuleDefinition::Entity(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ModuleDefinition::Type(__self_0),
                                ModuleDefinition::Type(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ModuleDefinition::Service(__self_0),
                                ModuleDefinition::Service(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for ModuleDefinition {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ModuleDefinition {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<EntityTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<TypeTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<ServiceTerm>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ModuleDefinition {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ModuleDefinition::Entity(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Entity",
                                &__self_0,
                            )
                        }
                        ModuleDefinition::Type(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Type",
                                &__self_0,
                            )
                        }
                        ModuleDefinition::Service(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Service",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            impl ASTTerm for ModuleTerm {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    visitor.process_module(self)?;
                    for param in self.definitions.iter() {
                        match param {
                            ModuleDefinition::Entity(entity) => entity.accept(visitor)?,
                            ModuleDefinition::Type(type_declaration) => {
                                type_declaration.accept(visitor)?
                            }
                            ModuleDefinition::Service(service) => {
                                service.accept(visitor)?
                            }
                        };
                    }
                    Ok(())
                }
            }
            impl ModuleTerm {
                pub fn definitions(&self) -> &[ModuleDefinition] {
                    self.definitions.as_ref()
                }
                pub fn new_boxed(definitions: Vec<ModuleDefinition>) -> Box<ModuleTerm> {
                    Box::new(ModuleTerm::new(definitions))
                }
                pub fn new(definitions: Vec<ModuleDefinition>) -> ModuleTerm {
                    ModuleTerm { definitions }
                }
            }
        }
        pub mod name_term {
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_name")]
            pub struct NameTerm {
                #[prop]
                value: String,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for NameTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for NameTerm {
                #[inline]
                fn eq(&self, other: &NameTerm) -> bool {
                    self.value == other.value
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for NameTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for NameTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for NameTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "NameTerm",
                        "value",
                        &&self.value,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for NameTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_name(self)?;
                    Ok(())
                }
            }
            impl NameTerm {
                pub fn value(&self) -> &String {
                    &self.value
                }
            }
            impl NameTerm {
                pub fn value_mut(&mut self) -> &mut String {
                    &mut self.value
                }
            }
            impl NameTerm {
                pub fn set_value(&mut self, value: String) {
                    self.value = value;
                }
            }
            impl NameTerm {
                pub fn new_boxed(value: String) -> NameTerm {
                    Box::new(NameTerm { value })
                }
                pub fn new(value: String) -> NameTerm {
                    NameTerm { value }
                }
            }
        }
        pub mod param_term {
            use super::name_term::NameTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_param")]
            pub struct ParamTerm {
                #[subnode_prop]
                name: Box<NameTerm>,
                #[subnode_prop]
                type_name: Box<NameTerm>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ParamTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ParamTerm {
                #[inline]
                fn eq(&self, other: &ParamTerm) -> bool {
                    self.name == other.name && self.type_name == other.type_name
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for ParamTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ParamTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ParamTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ParamTerm",
                        "name",
                        &&self.name,
                        "type_name",
                        &&self.type_name,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for ParamTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_param(self)?;
                    self.name.accept(visitor)?;
                    self.type_name.accept(visitor)?;
                    Ok(())
                }
            }
            impl ParamTerm {
                pub fn name(&self) -> &Box<NameTerm> {
                    &self.name
                }
            }
            impl ParamTerm {
                pub fn name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.name
                }
            }
            impl ParamTerm {
                pub fn set_name(&mut self, value: Box<NameTerm>) {
                    self.name = value;
                }
            }
            impl ParamTerm {
                pub fn type_name(&self) -> &Box<NameTerm> {
                    &self.type_name
                }
            }
            impl ParamTerm {
                pub fn type_name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.type_name
                }
            }
            impl ParamTerm {
                pub fn set_type_name(&mut self, value: Box<NameTerm>) {
                    self.type_name = value;
                }
            }
            impl ParamTerm {
                pub fn new_boxed(
                    name: Box<NameTerm>,
                    type_name: Box<NameTerm>,
                ) -> ParamTerm {
                    Box::new(ParamTerm { name, type_name })
                }
                pub fn new(name: Box<NameTerm>, type_name: Box<NameTerm>) -> ParamTerm {
                    ParamTerm { name, type_name }
                }
            }
        }
        pub mod returns_term {
            use super::name_term::NameTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_returns", generate_news = false)]
            pub struct ReturnsTerm {
                #[subnode_prop]
                type_name: Box<NameTerm>,
                #[prop]
                is_arrayed: bool,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ReturnsTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ReturnsTerm {
                #[inline]
                fn eq(&self, other: &ReturnsTerm) -> bool {
                    self.type_name == other.type_name
                        && self.is_arrayed == other.is_arrayed
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for ReturnsTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ReturnsTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<bool>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ReturnsTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ReturnsTerm",
                        "type_name",
                        &&self.type_name,
                        "is_arrayed",
                        &&self.is_arrayed,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for ReturnsTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_returns(self)?;
                    self.type_name.accept(visitor)?;
                    Ok(())
                }
            }
            impl ReturnsTerm {
                pub fn type_name(&self) -> &Box<NameTerm> {
                    &self.type_name
                }
            }
            impl ReturnsTerm {
                pub fn type_name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.type_name
                }
            }
            impl ReturnsTerm {
                pub fn set_type_name(&mut self, value: Box<NameTerm>) {
                    self.type_name = value;
                }
            }
            impl ReturnsTerm {
                pub fn is_arrayed(&self) -> &bool {
                    &self.is_arrayed
                }
            }
            impl ReturnsTerm {
                pub fn is_arrayed_mut(&mut self) -> &mut bool {
                    &mut self.is_arrayed
                }
            }
            impl ReturnsTerm {
                pub fn set_is_arrayed(&mut self, value: bool) {
                    self.is_arrayed = value;
                }
            }
            impl ReturnsTerm {
                pub fn new_boxed(type_name: Box<NameTerm>) -> Box<ReturnsTerm> {
                    Box::new(ReturnsTerm::new(type_name))
                }
                pub fn new(type_name: Box<NameTerm>) -> ReturnsTerm {
                    ReturnsTerm {
                        type_name,
                        is_arrayed: false,
                    }
                }
                pub fn new_arrayed_boxed(type_name: Box<NameTerm>) -> Box<ReturnsTerm> {
                    Box::new(ReturnsTerm::new_arrayed(type_name))
                }
                pub fn new_arrayed(type_name: Box<NameTerm>) -> ReturnsTerm {
                    ReturnsTerm {
                        type_name,
                        is_arrayed: true,
                    }
                }
            }
        }
        pub mod service_term {
            use super::super::super::visitor::{Visitor, VisitorError};
            use super::super::common::ast_term::ASTTerm;
            use super::action_term::ActionTerm;
            use super::entity_term::EntityTerm;
            use super::function_term::FunctionTerm;
            use super::name_term::NameTerm;
            use super::type_term::TypeTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_service")]
            pub struct ServiceTerm {
                #[subnode_prop]
                name: Box<NameTerm>,
                #[subnode_prop]
                definitions: Vec<ServiceDefinition>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ServiceTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ServiceTerm {
                #[inline]
                fn eq(&self, other: &ServiceTerm) -> bool {
                    self.name == other.name && self.definitions == other.definitions
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for ServiceTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ServiceTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Vec<ServiceDefinition>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ServiceTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ServiceTerm",
                        "name",
                        &&self.name,
                        "definitions",
                        &&self.definitions,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for ServiceTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_service(self)?;
                    self.name.accept(visitor)?;
                    self.definitions.accept(visitor)?;
                    Ok(())
                }
            }
            impl ServiceTerm {
                pub fn name(&self) -> &Box<NameTerm> {
                    &self.name
                }
            }
            impl ServiceTerm {
                pub fn name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.name
                }
            }
            impl ServiceTerm {
                pub fn set_name(&mut self, value: Box<NameTerm>) {
                    self.name = value;
                }
            }
            impl ServiceTerm {
                pub fn definitions(&self) -> &Vec<ServiceDefinition> {
                    &self.definitions
                }
            }
            impl ServiceTerm {
                pub fn definitions_mut(&mut self) -> &mut Vec<ServiceDefinition> {
                    &mut self.definitions
                }
            }
            impl ServiceTerm {
                pub fn set_definitions(&mut self, value: Vec<ServiceDefinition>) {
                    self.definitions = value;
                }
            }
            impl ServiceTerm {
                pub fn new_boxed(
                    name: Box<NameTerm>,
                    definitions: Vec<ServiceDefinition>,
                ) -> ServiceTerm {
                    Box::new(ServiceTerm { name, definitions })
                }
                pub fn new(
                    name: Box<NameTerm>,
                    definitions: Vec<ServiceDefinition>,
                ) -> ServiceTerm {
                    ServiceTerm { name, definitions }
                }
            }
            pub enum ServiceDefinition {
                Entity(Box<EntityTerm>),
                Function(Box<FunctionTerm>),
                Action(Box<ActionTerm>),
                Type(Box<TypeTerm>),
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ServiceDefinition {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ServiceDefinition {
                #[inline]
                fn eq(&self, other: &ServiceDefinition) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                            (
                                ServiceDefinition::Entity(__self_0),
                                ServiceDefinition::Entity(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ServiceDefinition::Function(__self_0),
                                ServiceDefinition::Function(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ServiceDefinition::Action(__self_0),
                                ServiceDefinition::Action(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ServiceDefinition::Type(__self_0),
                                ServiceDefinition::Type(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for ServiceDefinition {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ServiceDefinition {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<EntityTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<FunctionTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<ActionTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<TypeTerm>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ServiceDefinition {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ServiceDefinition::Entity(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Entity",
                                &__self_0,
                            )
                        }
                        ServiceDefinition::Function(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Function",
                                &__self_0,
                            )
                        }
                        ServiceDefinition::Action(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Action",
                                &__self_0,
                            )
                        }
                        ServiceDefinition::Type(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Type",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            impl ASTTerm for ServiceDefinition {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    match self {
                        ServiceDefinition::Entity(entity) => entity.accept(visitor)?,
                        ServiceDefinition::Function(function) => {
                            function.accept(visitor)?
                        }
                        ServiceDefinition::Action(action) => action.accept(visitor)?,
                        ServiceDefinition::Type(type_declaration) => {
                            type_declaration.accept(visitor)?
                        }
                    };
                    Ok(())
                }
            }
        }
        pub mod type_term {
            use super::name_term::NameTerm;
            use ast_term_derive::ASTTerm;
            #[ast_term(process_path = "process_type")]
            pub struct TypeTerm {
                #[subnode_prop]
                name: Box<NameTerm>,
                #[subnode_prop]
                resolved_type_name: Box<NameTerm>,
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for TypeTerm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for TypeTerm {
                #[inline]
                fn eq(&self, other: &TypeTerm) -> bool {
                    self.name == other.name
                        && self.resolved_type_name == other.resolved_type_name
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for TypeTerm {}
            #[automatically_derived]
            impl ::core::cmp::Eq for TypeTerm {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<NameTerm>>;
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for TypeTerm {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "TypeTerm",
                        "name",
                        &&self.name,
                        "resolved_type_name",
                        &&self.resolved_type_name,
                    )
                }
            }
            impl crate::ast::common::ast_term::ASTTerm for TypeTerm {
                fn accept(
                    &self,
                    visitor: &mut dyn crate::visitor::Visitor,
                ) -> Result<(), crate::visitor::VisitorError> {
                    visitor.process_type(self)?;
                    self.name.accept(visitor)?;
                    self.resolved_type_name.accept(visitor)?;
                    Ok(())
                }
            }
            impl TypeTerm {
                pub fn name(&self) -> &Box<NameTerm> {
                    &self.name
                }
            }
            impl TypeTerm {
                pub fn name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.name
                }
            }
            impl TypeTerm {
                pub fn set_name(&mut self, value: Box<NameTerm>) {
                    self.name = value;
                }
            }
            impl TypeTerm {
                pub fn resolved_type_name(&self) -> &Box<NameTerm> {
                    &self.resolved_type_name
                }
            }
            impl TypeTerm {
                pub fn resolved_type_name_mut(&mut self) -> &mut Box<NameTerm> {
                    &mut self.resolved_type_name
                }
            }
            impl TypeTerm {
                pub fn set_resolved_type_name(&mut self, value: Box<NameTerm>) {
                    self.resolved_type_name = value;
                }
            }
            impl TypeTerm {
                pub fn new_boxed(
                    name: Box<NameTerm>,
                    resolved_type_name: Box<NameTerm>,
                ) -> TypeTerm {
                    Box::new(TypeTerm {
                        name,
                        resolved_type_name,
                    })
                }
                pub fn new(
                    name: Box<NameTerm>,
                    resolved_type_name: Box<NameTerm>,
                ) -> TypeTerm {
                    TypeTerm {
                        name,
                        resolved_type_name,
                    }
                }
            }
        }
        pub use action_term::ActionTerm;
        pub use entity_term::EntityTerm;
        pub use field_term::FieldTerm;
        pub use function_term::FunctionTerm;
        pub use module_term::{ModuleDefinition, ModuleTerm};
        pub use name_term::NameTerm;
        pub use param_term::ParamTerm;
        pub use returns_term::ReturnsTerm;
        pub use service_term::{ServiceDefinition, ServiceTerm};
        pub use type_term::TypeTerm;
    }
    mod common {
        pub mod ast_term {
            use super::super::super::visitor::{Visitor, VisitorError};
            use std::ops::Deref;
            use std::sync::Arc;
            pub trait ASTTerm {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError>;
            }
            impl<T: ASTTerm> ASTTerm for dyn Deref<Target = T> {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    self.deref().accept(visitor)?;
                    Ok(())
                }
            }
            impl<T: ASTTerm> ASTTerm for Option<T> {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    if let Some(variant) = self {
                        variant.accept(visitor)?;
                    }
                    Ok(())
                }
            }
            impl<T: ASTTerm> ASTTerm for Box<T> {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    self.deref().accept(visitor)?;
                    Ok(())
                }
            }
            impl<T: ASTTerm> ASTTerm for Arc<T> {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    self.deref().accept(visitor)?;
                    Ok(())
                }
            }
            impl<T: ASTTerm> ASTTerm for [T] {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    for term in self.iter() {
                        term.accept(visitor)?;
                    }
                    Ok(())
                }
            }
        }
        pub mod reference {
            use super::super::super::visitor::{Visitor, VisitorError};
            use super::ast_term::ASTTerm;
            use std::fmt::Debug;
            /// A Suitable wrapper around the term to ast linkable and processable in parallel
            pub enum Reference<T: PartialEq + Eq + Debug> {
                Fulfilled(T),
                Virtual { qualifier: String },
            }
            #[automatically_derived]
            impl<T: PartialEq + Eq + Debug> ::core::marker::StructuralPartialEq
            for Reference<T> {}
            #[automatically_derived]
            impl<
                T: ::core::cmp::PartialEq + PartialEq + Eq + Debug,
            > ::core::cmp::PartialEq for Reference<T> {
                #[inline]
                fn eq(&self, other: &Reference<T>) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                            (
                                Reference::Fulfilled(__self_0),
                                Reference::Fulfilled(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                Reference::Virtual { qualifier: __self_0 },
                                Reference::Virtual { qualifier: __arg1_0 },
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            #[automatically_derived]
            impl<T: PartialEq + Eq + Debug> ::core::marker::StructuralEq
            for Reference<T> {}
            #[automatically_derived]
            impl<T: ::core::cmp::Eq + PartialEq + Eq + Debug> ::core::cmp::Eq
            for Reference<T> {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<T>;
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
            #[automatically_derived]
            impl<T: ::core::fmt::Debug + PartialEq + Eq + Debug> ::core::fmt::Debug
            for Reference<T> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        Reference::Fulfilled(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Fulfilled",
                                &__self_0,
                            )
                        }
                        Reference::Virtual { qualifier: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Virtual",
                                "qualifier",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            impl<T: ASTTerm + PartialEq + Eq + Debug> ASTTerm for Reference<T> {
                fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
                    if let Reference::Fulfilled(value) = self {
                        value.accept(visitor)?;
                        return Ok(());
                    }
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Cannot accept visitor with a virtual reference"],
                            &[],
                        ),
                    )
                }
            }
            impl<T: PartialEq + Eq + Debug> Reference<T> {
                pub fn fulfill(&mut self, storage: &dyn ReferenceStorage<T>) {
                    if let Reference::Virtual { qualifier } = self {
                        *self = Reference::new_fulfilled(storage.get_value(qualifier));
                    }
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Already fulfilled reference cannot be fulfilled"],
                            &[],
                        ),
                    )
                }
            }
            pub trait ReferenceStorage<T: PartialEq + Eq + Debug> {
                fn get_value(&self, qualifier: &str) -> T;
            }
            impl<T: PartialEq + Eq + Debug> Reference<T> {
                pub fn value(self) -> T {
                    if let Reference::Fulfilled(value) = self {
                        return value;
                    }
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Cannot get value of a virtual reference"],
                            &[],
                        ),
                    )
                }
                pub fn value_ref(&self) -> &T {
                    if let Reference::Fulfilled(value) = self {
                        return value;
                    }
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Cannot get value of a virtual reference"],
                            &[],
                        ),
                    )
                }
                pub fn value_ref_mut(&mut self) -> &mut T {
                    if let Reference::Fulfilled(value) = self {
                        return value;
                    }
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Cannot get value of a virtual reference"],
                            &[],
                        ),
                    )
                }
            }
            impl<T: PartialEq + Eq + Debug> From<T> for Reference<T> {
                fn from(value: T) -> Reference<T> {
                    Reference::Fulfilled(value)
                }
            }
            impl<T: PartialEq + Eq + Debug> Reference<T> {
                pub fn new_fulfilled(value: T) -> Reference<T> {
                    Reference::Fulfilled(value)
                }
                pub fn new_virtual(qualifier: String) -> Reference<T> {
                    Reference::Virtual { qualifier }
                }
            }
        }
        pub mod term_iter {
            use super::reference::Reference;
            use std::fmt::Debug;
            use std::ops::Deref;
            pub struct TermIter<'i, T> {
                iter: Box<dyn Iterator<Item = &'i T> + 'i>,
            }
            impl<'i, T: PartialEq + Eq + Debug> Iterator for TermIter<'i, T> {
                type Item = &'i T;
                fn next(&mut self) -> Option<Self::Item> {
                    self.iter.next()
                }
            }
            impl<'i, T: PartialEq + Eq + Debug> TermIter<'i, T> {
                pub fn new_from_deref_vec<D: Deref<Target = T>>(
                    iterable: &'i Vec<D>,
                ) -> Self {
                    let iterator = iterable.iter();
                    let iterator = iterator.map(|x| x.deref());
                    TermIter {
                        iter: Box::new(iterator),
                    }
                }
                pub fn new_from_referenced_deref_vec<
                    D: Deref<Target = T> + PartialEq + Eq + Debug,
                >(iterable: &'i Vec<Reference<D>>) -> Self {
                    let iterator = iterable.iter();
                    let iterator = iterator.map(|x| x.value_ref().deref());
                    TermIter {
                        iter: Box::new(iterator),
                    }
                }
            }
        }
        pub use ast_term::*;
        pub use reference::*;
        pub use term_iter::*;
    }
    pub use cdl::*;
    pub use common::*;
}
pub mod parser {
    mod parse_error {
        use std::fmt;
        use std::io;
        pub struct ParseError {
            error_type: ParseErrorType,
            message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ParseError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ParseError",
                    "error_type",
                    &&self.error_type,
                    "message",
                    &&self.message,
                )
            }
        }
        pub enum ParseErrorType {
            FileIOError,
            SyntaxError,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ParseErrorType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ParseErrorType::FileIOError => {
                        ::core::fmt::Formatter::write_str(f, "FileIOError")
                    }
                    ParseErrorType::SyntaxError => {
                        ::core::fmt::Formatter::write_str(f, "SyntaxError")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParseErrorType {
            #[inline]
            fn clone(&self) -> ParseErrorType {
                match self {
                    ParseErrorType::FileIOError => ParseErrorType::FileIOError,
                    ParseErrorType::SyntaxError => ParseErrorType::SyntaxError,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParseErrorType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParseErrorType {
            #[inline]
            fn eq(&self, other: &ParseErrorType) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        impl fmt::Display for ParseErrorType {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    ParseErrorType::FileIOError => {
                        formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(&["FileIOError"], &[]),
                            )
                    }
                    ParseErrorType::SyntaxError => {
                        formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(&["SyntaxError"], &[]),
                            )
                    }
                }
            }
        }
        impl ParseError {
            pub fn new(message: String, error_type: ParseErrorType) -> ParseError {
                ParseError { message, error_type }
            }
            pub fn get_message(&self) -> String {
                self.message.clone()
            }
            pub fn get_error_type(&self) -> ParseErrorType {
                self.error_type.clone()
            }
        }
        impl fmt::Display for ParseError {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["parse error ", ", reason: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&self.error_type),
                                ::core::fmt::ArgumentV1::new_display(&self.message),
                            ],
                        ),
                    )
            }
        }
        impl From<io::Error> for ParseError {
            fn from(error: io::Error) -> ParseError {
                ParseError::new(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&error)],
                            ),
                        );
                        res
                    },
                    ParseErrorType::FileIOError,
                )
            }
        }
    }
    mod parser {
        use std::io::prelude::*;
        use std::fs::File;
        use std::path::Path;
        use super::super::ast::ModuleTerm;
        use super::parse_error::ParseError;
        use super::parse_error::ParseErrorType;
        pub struct Parser {
            path: String,
        }
        impl Parser {
            pub fn new(path: String) -> Parser {
                Parser { path }
            }
            pub fn parse(&self) -> Result<Box<ModuleTerm>, ParseError> {
                let path = Path::new(&self.path);
                let mut file = File::open(path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let module = match super::cds::ModuleParser::new().parse(&content) {
                    Ok(module_ast) => module_ast,
                    Err(lalrpop_auto_generated_error) => {
                        return Err(
                            ParseError::new(
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &[""],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &lalrpop_auto_generated_error,
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                },
                                ParseErrorType::SyntaxError,
                            ),
                        );
                    }
                };
                Ok(module)
            }
        }
    }
    use lalrpop_util::lalrpop_mod;
    mod cds {
        use crate::ast::{
            ActionTerm, EntityTerm, FieldTerm, FunctionTerm, ModuleDefinition,
            ModuleTerm, NameTerm, ParamTerm, ReturnsTerm, ServiceDefinition, ServiceTerm,
            TypeTerm,
        };
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        mod __parse__Module {
            #![allow(
                non_snake_case,
                non_camel_case_types,
                unused_mut,
                unused_variables,
                unused_imports,
                unused_parens,
                clippy::all
            )]
            use crate::ast::{
                ActionTerm, EntityTerm, FieldTerm, FunctionTerm, ModuleDefinition,
                ModuleTerm, NameTerm, ParamTerm, ReturnsTerm, ServiceDefinition,
                ServiceTerm, TypeTerm,
            };
            #[allow(unused_extern_crates)]
            extern crate lalrpop_util as __lalrpop_util;
            #[allow(unused_imports)]
            use self::__lalrpop_util::state_machine as __state_machine;
            extern crate core;
            extern crate alloc;
            use self::__lalrpop_util::lexer::Token;
            #[allow(dead_code)]
            pub(crate) enum __Symbol<'input> {
                Variant0(&'input str),
                Variant1(Box<ActionTerm>),
                Variant2(Vec<Box<NameTerm>>),
                Variant3(Box<EntityTerm>),
                Variant4(Box<FieldTerm>),
                Variant5(Vec<Box<FieldTerm>>),
                Variant6(Box<FunctionTerm>),
                Variant7(Box<ModuleTerm>),
                Variant8(Vec<ModuleDefinition>),
                Variant9(Box<NameTerm>),
                Variant10(Box<ParamTerm>),
                Variant11(Vec<Box<ParamTerm>>),
                Variant12(Box<ReturnsTerm>),
                Variant13(Box<ServiceTerm>),
                Variant14(Vec<ServiceDefinition>),
                Variant15(Box<TypeTerm>),
            }
            const __ACTION: &[i8] = &[
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                33,
                3,
                0,
                0,
                0,
                4,
                5,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                33,
                3,
                0,
                0,
                0,
                4,
                5,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                46,
                38,
                0,
                0,
                0,
                0,
                0,
                14,
                0,
                33,
                3,
                15,
                0,
                0,
                0,
                5,
                0,
                51,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                55,
                38,
                0,
                0,
                0,
                0,
                0,
                14,
                0,
                33,
                3,
                15,
                0,
                0,
                0,
                5,
                0,
                60,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                66,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                68,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                72,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                74,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                75,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                80,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                80,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                80,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                38,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -15,
                -15,
                0,
                0,
                0,
                -15,
                -15,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -16,
                -16,
                0,
                0,
                0,
                -16,
                -16,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -17,
                -17,
                0,
                0,
                0,
                -17,
                -17,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                6,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -18,
                -18,
                0,
                0,
                0,
                -18,
                -18,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -19,
                -19,
                0,
                0,
                0,
                -19,
                -19,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -20,
                -20,
                0,
                0,
                0,
                -20,
                -20,
                0,
                0,
                0,
                0,
                0,
                0,
                7,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                8,
                0,
                0,
                -21,
                -21,
                -21,
                -21,
                -21,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -21,
                0,
                -21,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                9,
                0,
                0,
                0,
                0,
                0,
                10,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                11,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                16,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                17,
                0,
                0,
                0,
                0,
                -3,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -3,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -10,
                -10,
                0,
                0,
                0,
                18,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -5,
                0,
                -5,
                -5,
                -5,
                0,
                0,
                -5,
                -5,
                0,
                -5,
                0,
                0,
                0,
                0,
                0,
                0,
                -30,
                0,
                -30,
                -30,
                -30,
                0,
                0,
                0,
                -30,
                0,
                -30,
                0,
                0,
                0,
                0,
                0,
                0,
                -29,
                0,
                -29,
                -29,
                -29,
                0,
                0,
                0,
                -29,
                0,
                -29,
                0,
                0,
                0,
                0,
                0,
                0,
                -31,
                0,
                -31,
                -31,
                -31,
                0,
                0,
                0,
                -31,
                0,
                -31,
                0,
                0,
                0,
                0,
                0,
                0,
                -32,
                0,
                -32,
                -32,
                -32,
                0,
                0,
                0,
                -32,
                0,
                -32,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -27,
                -27,
                0,
                0,
                0,
                -27,
                -27,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                63,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -11,
                -11,
                0,
                0,
                0,
                0,
                0,
                -6,
                0,
                -6,
                -6,
                -6,
                0,
                0,
                -6,
                -6,
                0,
                -6,
                0,
                0,
                0,
                0,
                0,
                0,
                -34,
                0,
                -34,
                -34,
                -34,
                0,
                0,
                0,
                -34,
                0,
                -34,
                0,
                0,
                0,
                0,
                0,
                0,
                -33,
                0,
                -33,
                -33,
                -33,
                0,
                0,
                0,
                -33,
                0,
                -33,
                0,
                0,
                0,
                0,
                0,
                0,
                -35,
                0,
                -35,
                -35,
                -35,
                0,
                0,
                0,
                -35,
                0,
                -35,
                0,
                0,
                0,
                0,
                0,
                0,
                -36,
                0,
                -36,
                -36,
                -36,
                0,
                0,
                0,
                -36,
                0,
                -36,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -28,
                -28,
                0,
                0,
                0,
                -28,
                -28,
                0,
                0,
                0,
                20,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                21,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -38,
                0,
                -38,
                -38,
                -38,
                0,
                0,
                -38,
                -38,
                0,
                -38,
                0,
                0,
                0,
                0,
                0,
                0,
                -37,
                0,
                -37,
                -37,
                -37,
                0,
                0,
                -37,
                -37,
                0,
                -37,
                0,
                0,
                0,
                -4,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -4,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -7,
                0,
                -7,
                -7,
                -7,
                0,
                0,
                -7,
                -7,
                0,
                -7,
                0,
                0,
                0,
                0,
                0,
                69,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -8,
                0,
                -8,
                -8,
                -8,
                0,
                0,
                -8,
                -8,
                0,
                -8,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -9,
                -9,
                0,
                0,
                0,
                24,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -23,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -23,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                25,
                0,
                0,
                0,
                0,
                0,
                0,
                -24,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -24,
                0,
                0,
                0,
                0,
                77,
                0,
                0,
                0,
                0,
                0,
                0,
                26,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                27,
                0,
                0,
                0,
                0,
                0,
                0,
                -22,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -22,
                0,
                0,
                0,
                0,
                0,
                -1,
                0,
                -1,
                -1,
                -1,
                0,
                0,
                0,
                -1,
                0,
                -1,
                0,
                0,
                0,
                0,
                0,
                -25,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                83,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                28,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                84,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                85,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -12,
                0,
                -12,
                -12,
                -12,
                0,
                0,
                0,
                -12,
                0,
                -12,
                0,
                0,
                0,
                0,
                0,
                0,
                -2,
                0,
                -2,
                -2,
                -2,
                0,
                0,
                0,
                -2,
                0,
                -2,
                0,
                0,
                0,
                0,
                0,
                0,
                -13,
                0,
                -13,
                -13,
                -13,
                0,
                0,
                0,
                -13,
                0,
                -13,
                0,
                0,
                0,
                0,
                0,
                -26,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            fn __action(state: i8, integer: usize) -> i8 {
                __ACTION[(state as usize) * 17 + integer]
            }
            const __EOF_ACTION: &[i8] = &[
                0,
                -14,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -15,
                -39,
                -16,
                -17,
                0,
                -18,
                -19,
                -20,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -5,
                0,
                0,
                0,
                0,
                -27,
                0,
                0,
                0,
                -6,
                0,
                0,
                0,
                0,
                -28,
                0,
                0,
                -38,
                -37,
                0,
                -7,
                0,
                -8,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            fn __goto(state: i8, nt: usize) -> i8 {
                match nt {
                    0 => {
                        match state {
                            12 => 55,
                            _ => 46,
                        }
                    }
                    1 => 41,
                    2 => {
                        match state {
                            1 => 33,
                            8 => 47,
                            12 => 56,
                            _ => 28,
                        }
                    }
                    3 => {
                        match state {
                            11 | 18 => 53,
                            _ => 43,
                        }
                    }
                    4 => {
                        match state {
                            16 => 18,
                            _ => 11,
                        }
                    }
                    5 => {
                        match state {
                            12 => 57,
                            _ => 48,
                        }
                    }
                    6 => 29,
                    7 => 1,
                    8 => {
                        match state {
                            2 => 36,
                            3 => 38,
                            4 => 39,
                            5 => 40,
                            6 => 42,
                            9 => 51,
                            10 => 52,
                            13 => 60,
                            14 => 61,
                            15 => 64,
                            17 => 66,
                            19..=22 => 69,
                            23 => 75,
                            24..=26 => 77,
                            27 => 85,
                            _ => 44,
                        }
                    }
                    9 => {
                        match state {
                            21..=22 => 72,
                            _ => 70,
                        }
                    }
                    10 => {
                        match state {
                            20 => 22,
                            _ => 21,
                        }
                    }
                    11 => {
                        match state {
                            25 => 80,
                            26 => 81,
                            _ => 78,
                        }
                    }
                    12 => {
                        match state {
                            1 => 34,
                            _ => 30,
                        }
                    }
                    13 => 12,
                    14 => {
                        match state {
                            1 => 35,
                            8 => 49,
                            12 => 58,
                            _ => 31,
                        }
                    }
                    _ => 0,
                }
            }
            fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
                const __TERMINAL: &[&str] = &[
                    r###""(""###,
                    r###"")""###,
                    r###"",""###,
                    r###"":""###,
                    r###"";""###,
                    r###""action""###,
                    r###""array""###,
                    r###""define""###,
                    r###""entity""###,
                    r###""function""###,
                    r###""of""###,
                    r###""returns""###,
                    r###""service""###,
                    r###""type""###,
                    r###""{""###,
                    r###""}""###,
                    r###"r#"[a-zA-Z0-9-]+"#"###,
                ];
                __TERMINAL
                    .iter()
                    .enumerate()
                    .filter_map(|(index, terminal)| {
                        let next_state = __action(__state, index);
                        if next_state == 0 {
                            None
                        } else {
                            Some(alloc::string::ToString::to_string(terminal))
                        }
                    })
                    .collect()
            }
            pub(crate) struct __StateMachine<'input> {
                input: &'input str,
                __phantom: core::marker::PhantomData<(&'input ())>,
            }
            impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
                type Location = usize;
                type Error = &'static str;
                type Token = Token<'input>;
                type TokenIndex = usize;
                type Symbol = __Symbol<'input>;
                type Success = Box<ModuleTerm>;
                type StateIndex = i8;
                type Action = i8;
                type ReduceIndex = i8;
                type NonterminalIndex = usize;
                #[inline]
                fn start_location(&self) -> Self::Location {
                    Default::default()
                }
                #[inline]
                fn start_state(&self) -> Self::StateIndex {
                    0
                }
                #[inline]
                fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
                    __token_to_integer(token, core::marker::PhantomData::<(&())>)
                }
                #[inline]
                fn action(&self, state: i8, integer: usize) -> i8 {
                    __action(state, integer)
                }
                #[inline]
                fn error_action(&self, state: i8) -> i8 {
                    __action(state, 17 - 1)
                }
                #[inline]
                fn eof_action(&self, state: i8) -> i8 {
                    __EOF_ACTION[state as usize]
                }
                #[inline]
                fn goto(&self, state: i8, nt: usize) -> i8 {
                    __goto(state, nt)
                }
                fn token_to_symbol(
                    &self,
                    token_index: usize,
                    token: Self::Token,
                ) -> Self::Symbol {
                    __token_to_symbol(
                        token_index,
                        token,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                fn expected_tokens(
                    &self,
                    state: i8,
                ) -> alloc::vec::Vec<alloc::string::String> {
                    __expected_tokens(state)
                }
                #[inline]
                fn uses_error_recovery(&self) -> bool {
                    false
                }
                #[inline]
                fn error_recovery_symbol(
                    &self,
                    recovery: __state_machine::ErrorRecovery<Self>,
                ) -> Self::Symbol {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["error recovery not enabled for this grammar"],
                            &[],
                        ),
                    )
                }
                fn reduce(
                    &mut self,
                    action: i8,
                    start_location: Option<&Self::Location>,
                    states: &mut alloc::vec::Vec<i8>,
                    symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
                ) -> Option<__state_machine::ParseResult<Self>> {
                    __reduce(
                        self.input,
                        action,
                        start_location,
                        states,
                        symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                fn simulate_reduce(
                    &self,
                    action: i8,
                ) -> __state_machine::SimulatedReduce<Self> {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["error recovery not enabled for this grammar"],
                            &[],
                        ),
                    )
                }
            }
            fn __token_to_integer<'input>(
                __token: &Token<'input>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> Option<usize> {
                match *__token {
                    Token(1, _) if true => Some(0),
                    Token(2, _) if true => Some(1),
                    Token(3, _) if true => Some(2),
                    Token(4, _) if true => Some(3),
                    Token(5, _) if true => Some(4),
                    Token(6, _) if true => Some(5),
                    Token(7, _) if true => Some(6),
                    Token(8, _) if true => Some(7),
                    Token(9, _) if true => Some(8),
                    Token(10, _) if true => Some(9),
                    Token(11, _) if true => Some(10),
                    Token(12, _) if true => Some(11),
                    Token(13, _) if true => Some(12),
                    Token(14, _) if true => Some(13),
                    Token(15, _) if true => Some(14),
                    Token(16, _) if true => Some(15),
                    Token(0, _) if true => Some(16),
                    _ => None,
                }
            }
            fn __token_to_symbol<'input>(
                __token_index: usize,
                __token: Token<'input>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> __Symbol<'input> {
                match __token_index {
                    0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15
                    | 16 => {
                        match __token {
                            Token(1, __tok0)
                            | Token(2, __tok0)
                            | Token(3, __tok0)
                            | Token(4, __tok0)
                            | Token(5, __tok0)
                            | Token(6, __tok0)
                            | Token(7, __tok0)
                            | Token(8, __tok0)
                            | Token(9, __tok0)
                            | Token(10, __tok0)
                            | Token(11, __tok0)
                            | Token(12, __tok0)
                            | Token(13, __tok0)
                            | Token(14, __tok0)
                            | Token(15, __tok0)
                            | Token(16, __tok0)
                            | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                            _ => {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            pub struct ModuleParser {
                builder: __lalrpop_util::lexer::MatcherBuilder,
                _priv: (),
            }
            impl ModuleParser {
                pub fn new() -> ModuleParser {
                    let __builder = super::__intern_token::new_builder();
                    ModuleParser {
                        builder: __builder,
                        _priv: (),
                    }
                }
                #[allow(dead_code)]
                pub fn parse<'input>(
                    &self,
                    input: &'input str,
                ) -> Result<
                    Box<ModuleTerm>,
                    __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
                > {
                    let mut __tokens = self.builder.matcher(input);
                    __state_machine::Parser::drive(
                        __StateMachine {
                            input,
                            __phantom: core::marker::PhantomData::<(&())>,
                        },
                        __tokens,
                    )
                }
            }
            pub(crate) fn __reduce<'input>(
                input: &'input str,
                __action: i8,
                __lookahead_start: Option<&usize>,
                __states: &mut alloc::vec::Vec<i8>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> Option<
                Result<
                    Box<ModuleTerm>,
                    __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
                >,
            > {
                let (__pop_states, __nonterminal) = match __action {
                    0 => {
                        __reduce0(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    1 => {
                        __reduce1(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    2 => {
                        __reduce2(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    3 => {
                        __reduce3(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    4 => {
                        __reduce4(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    5 => {
                        __reduce5(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    6 => {
                        __reduce6(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    7 => {
                        __reduce7(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    8 => {
                        __reduce8(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    9 => {
                        __reduce9(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    10 => {
                        __reduce10(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    11 => {
                        __reduce11(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    12 => {
                        __reduce12(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    13 => {
                        __reduce13(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    14 => {
                        __reduce14(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    15 => {
                        __reduce15(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    16 => {
                        __reduce16(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    17 => {
                        __reduce17(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    18 => {
                        __reduce18(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    19 => {
                        __reduce19(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    20 => {
                        __reduce20(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    21 => {
                        __reduce21(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    22 => {
                        __reduce22(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    23 => {
                        __reduce23(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    24 => {
                        __reduce24(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    25 => {
                        __reduce25(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    26 => {
                        __reduce26(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    27 => {
                        __reduce27(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    28 => {
                        __reduce28(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    29 => {
                        __reduce29(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    30 => {
                        __reduce30(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    31 => {
                        __reduce31(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    32 => {
                        __reduce32(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    33 => {
                        __reduce33(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    34 => {
                        __reduce34(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    35 => {
                        __reduce35(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    36 => {
                        __reduce36(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    37 => {
                        __reduce37(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    38 => {
                        let __sym0 = __pop_Variant7(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::__action0(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["invalid action code "],
                                &[::core::fmt::ArgumentV1::new_display(&__action)],
                            ),
                        )
                    }
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - __pop_states);
                let __state = *__states.last().unwrap();
                let __next_state = __goto(__state, __nonterminal);
                __states.push(__next_state);
                None
            }
            #[inline(never)]
            fn __symbol_type_mismatch() -> ! {
                ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(&["symbol type mismatch"], &[]),
                )
            }
            fn __pop_Variant1<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<ActionTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant3<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<EntityTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant4<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<FieldTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant6<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<FunctionTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant7<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<ModuleTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant9<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<NameTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant10<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<ParamTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant12<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<ReturnsTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant13<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<ServiceTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant15<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Box<TypeTerm>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant5<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Vec<Box<FieldTerm>>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant2<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Vec<Box<NameTerm>>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant11<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Vec<Box<ParamTerm>>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant8<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Vec<ModuleDefinition>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant14<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Vec<ServiceDefinition>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant0<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, &'input str, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            pub(crate) fn __reduce0<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 6) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 6")
                }
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant11(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action27(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                    __sym5,
                );
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (6, 0)
            }
            pub(crate) fn __reduce1<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 8) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 8")
                }
                let __sym7 = __pop_Variant0(__symbols);
                let __sym6 = __pop_Variant12(__symbols);
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant11(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action28(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                    __sym5,
                    __sym6,
                    __sym7,
                );
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (8, 0)
            }
            pub(crate) fn __reduce2<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            pub(crate) fn __reduce3<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 3) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
                }
                let __sym2 = __pop_Variant9(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (3, 1)
            }
            pub(crate) fn __reduce4<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action18(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (4, 2)
            }
            pub(crate) fn __reduce5<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 5) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 5")
                }
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant5(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action19(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                );
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (5, 2)
            }
            pub(crate) fn __reduce6<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 6) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 6")
                }
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant2(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action20(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                    __sym5,
                );
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (6, 2)
            }
            pub(crate) fn __reduce7<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 7) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 7")
                }
                let __sym6 = __pop_Variant0(__symbols);
                let __sym5 = __pop_Variant5(__symbols);
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant2(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action21(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                    __sym5,
                    __sym6,
                );
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (7, 2)
            }
            pub(crate) fn __reduce8<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant9(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (4, 3)
            }
            pub(crate) fn __reduce9<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                __symbols.push((__start, __Symbol::Variant5(__nt), __end));
                (1, 4)
            }
            pub(crate) fn __reduce10<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant4(__symbols);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant5(__nt), __end));
                (2, 4)
            }
            pub(crate) fn __reduce11<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 7) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 7")
                }
                let __sym6 = __pop_Variant0(__symbols);
                let __sym5 = __pop_Variant12(__symbols);
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action29(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                    __sym5,
                    __sym6,
                );
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (7, 5)
            }
            pub(crate) fn __reduce12<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 8) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 8")
                }
                let __sym7 = __pop_Variant0(__symbols);
                let __sym6 = __pop_Variant12(__symbols);
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant11(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action30(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                    __sym5,
                    __sym6,
                    __sym7,
                );
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (8, 5)
            }
            pub(crate) fn __reduce13<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                __symbols.push((__start, __Symbol::Variant7(__nt), __end));
                (1, 6)
            }
            pub(crate) fn __reduce14<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 7)
            }
            pub(crate) fn __reduce15<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant13(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 7)
            }
            pub(crate) fn __reduce16<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant15(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 7)
            }
            pub(crate) fn __reduce17<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (2, 7)
            }
            pub(crate) fn __reduce18<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant13(__symbols);
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (2, 7)
            }
            pub(crate) fn __reduce19<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant15(__symbols);
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (2, 7)
            }
            pub(crate) fn __reduce20<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                __symbols.push((__start, __Symbol::Variant9(__nt), __end));
                (1, 8)
            }
            pub(crate) fn __reduce21<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 3) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
                }
                let __sym2 = __pop_Variant9(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action35(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant10(__nt), __end));
                (3, 9)
            }
            pub(crate) fn __reduce22<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                __symbols.push((__start, __Symbol::Variant11(__nt), __end));
                (1, 10)
            }
            pub(crate) fn __reduce23<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant10(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action34(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant11(__nt), __end));
                (2, 10)
            }
            pub(crate) fn __reduce24<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31(input, __sym0);
                __symbols.push((__start, __Symbol::Variant12(__nt), __end));
                (1, 11)
            }
            pub(crate) fn __reduce25<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 3) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
                }
                let __sym2 = __pop_Variant9(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant12(__nt), __end));
                (3, 11)
            }
            pub(crate) fn __reduce26<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant13(__nt), __end));
                (4, 12)
            }
            pub(crate) fn __reduce27<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 5) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 5")
                }
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant14(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action9(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                );
                __symbols.push((__start, __Symbol::Variant13(__nt), __end));
                (5, 12)
            }
            pub(crate) fn __reduce28<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (1, 13)
            }
            pub(crate) fn __reduce29<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (1, 13)
            }
            pub(crate) fn __reduce30<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (1, 13)
            }
            pub(crate) fn __reduce31<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant15(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (1, 13)
            }
            pub(crate) fn __reduce32<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (2, 13)
            }
            pub(crate) fn __reduce33<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (2, 13)
            }
            pub(crate) fn __reduce34<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant6(__symbols);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (2, 13)
            }
            pub(crate) fn __reduce35<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant15(__symbols);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant14(__nt), __end));
                (2, 13)
            }
            pub(crate) fn __reduce36<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 6) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 6")
                }
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant9(__symbols);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant9(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action36(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                    __sym5,
                );
                __symbols.push((__start, __Symbol::Variant15(__nt), __end));
                (6, 14)
            }
            pub(crate) fn __reduce37<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 5) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 5")
                }
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant9(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant9(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action37(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                );
                __symbols.push((__start, __Symbol::Variant15(__nt), __end));
                (5, 14)
            }
        }
        pub use self::__parse__Module::ModuleParser;
        mod __intern_token {
            #![allow(unused_imports)]
            use crate::ast::{
                ActionTerm, EntityTerm, FieldTerm, FunctionTerm, ModuleDefinition,
                ModuleTerm, NameTerm, ParamTerm, ReturnsTerm, ServiceDefinition,
                ServiceTerm, TypeTerm,
            };
            #[allow(unused_extern_crates)]
            extern crate lalrpop_util as __lalrpop_util;
            #[allow(unused_imports)]
            use self::__lalrpop_util::state_machine as __state_machine;
            extern crate core;
            extern crate alloc;
            pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
                let __strs: &[(&str, bool)] = &[
                    ("^([\\-0-9A-Za-z]+)", false),
                    ("^(\\()", false),
                    ("^(\\))", false),
                    ("^(,)", false),
                    ("^(:)", false),
                    ("^(;)", false),
                    ("^(action)", false),
                    ("^(array)", false),
                    ("^(define)", false),
                    ("^(entity)", false),
                    ("^(function)", false),
                    ("^(of)", false),
                    ("^(returns)", false),
                    ("^(service)", false),
                    ("^(type)", false),
                    ("^(\\{)", false),
                    ("^(\\})", false),
                    (r"^(\s*)", true),
                ];
                __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied())
                    .unwrap()
            }
        }
        pub(crate) use self::__lalrpop_util::lexer::Token;
        #[allow(unused_variables)]
        fn __action0<'input>(
            input: &'input str,
            (_, __0, _): (usize, Box<ModuleTerm>, usize),
        ) -> Box<ModuleTerm> {
            __0
        }
        #[allow(unused_variables)]
        fn __action1<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ModuleDefinition>, usize),
        ) -> Box<ModuleTerm> {
            { ModuleTerm::new_boxed(definitions) }
        }
        #[allow(unused_variables)]
        fn __action2<'input>(
            input: &'input str,
            (_, entity, _): (usize, Box<EntityTerm>, usize),
        ) -> Vec<ModuleDefinition> {
            {
                let mut definitions: Vec<ModuleDefinition> = Vec::new();
                definitions.push(ModuleDefinition::Entity(entity));
                definitions
            }
        }
        #[allow(unused_variables)]
        fn __action3<'input>(
            input: &'input str,
            (_, service, _): (usize, Box<ServiceTerm>, usize),
        ) -> Vec<ModuleDefinition> {
            {
                let mut definitions: Vec<ModuleDefinition> = Vec::new();
                definitions.push(ModuleDefinition::Service(service));
                definitions
            }
        }
        #[allow(unused_variables)]
        fn __action4<'input>(
            input: &'input str,
            (_, type_definition, _): (usize, Box<TypeTerm>, usize),
        ) -> Vec<ModuleDefinition> {
            {
                let mut definitions: Vec<ModuleDefinition> = Vec::new();
                definitions.push(ModuleDefinition::Type(type_definition));
                definitions
            }
        }
        #[allow(unused_variables)]
        fn __action5<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ModuleDefinition>, usize),
            (_, entity, _): (usize, Box<EntityTerm>, usize),
        ) -> Vec<ModuleDefinition> {
            {
                let mut new_definitions = Vec::new();
                new_definitions.extend(definitions);
                new_definitions.push(ModuleDefinition::Entity(entity));
                new_definitions
            }
        }
        #[allow(unused_variables)]
        fn __action6<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ModuleDefinition>, usize),
            (_, service, _): (usize, Box<ServiceTerm>, usize),
        ) -> Vec<ModuleDefinition> {
            {
                let mut new_definitions = Vec::new();
                new_definitions.extend(definitions);
                new_definitions.push(ModuleDefinition::Service(service));
                new_definitions
            }
        }
        #[allow(unused_variables)]
        fn __action7<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ModuleDefinition>, usize),
            (_, type_definition, _): (usize, Box<TypeTerm>, usize),
        ) -> Vec<ModuleDefinition> {
            {
                let mut new_definitions = Vec::new();
                new_definitions.extend(definitions);
                new_definitions.push(ModuleDefinition::Type(type_definition));
                new_definitions
            }
        }
        #[allow(unused_variables)]
        fn __action8<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<ServiceTerm> {
            { ServiceTerm::new_boxed(name, Vec::new()) }
        }
        #[allow(unused_variables)]
        fn __action9<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, definitions, _): (usize, Vec<ServiceDefinition>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<ServiceTerm> {
            { ServiceTerm::new_boxed(name, definitions) }
        }
        #[allow(unused_variables)]
        fn __action10<'input>(
            input: &'input str,
            (_, entity, _): (usize, Box<EntityTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut definitions: Vec<ServiceDefinition> = Vec::new();
                definitions.push(ServiceDefinition::Entity(entity));
                definitions
            }
        }
        #[allow(unused_variables)]
        fn __action11<'input>(
            input: &'input str,
            (_, action, _): (usize, Box<ActionTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut definitions: Vec<ServiceDefinition> = Vec::new();
                definitions.push(ServiceDefinition::Action(action));
                definitions
            }
        }
        #[allow(unused_variables)]
        fn __action12<'input>(
            input: &'input str,
            (_, function, _): (usize, Box<FunctionTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut definitions: Vec<ServiceDefinition> = Vec::new();
                definitions.push(ServiceDefinition::Function(function));
                definitions
            }
        }
        #[allow(unused_variables)]
        fn __action13<'input>(
            input: &'input str,
            (_, type_definition, _): (usize, Box<TypeTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut definitions: Vec<ServiceDefinition> = Vec::new();
                definitions.push(ServiceDefinition::Type(type_definition));
                definitions
            }
        }
        #[allow(unused_variables)]
        fn __action14<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ServiceDefinition>, usize),
            (_, entity, _): (usize, Box<EntityTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut new_definitions = Vec::new();
                new_definitions.extend(definitions);
                new_definitions.push(ServiceDefinition::Entity(entity));
                new_definitions
            }
        }
        #[allow(unused_variables)]
        fn __action15<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ServiceDefinition>, usize),
            (_, action, _): (usize, Box<ActionTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut new_definitions = Vec::new();
                new_definitions.extend(definitions);
                new_definitions.push(ServiceDefinition::Action(action));
                new_definitions
            }
        }
        #[allow(unused_variables)]
        fn __action16<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ServiceDefinition>, usize),
            (_, function, _): (usize, Box<FunctionTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut new_definitions = Vec::new();
                new_definitions.extend(definitions);
                new_definitions.push(ServiceDefinition::Function(function));
                new_definitions
            }
        }
        #[allow(unused_variables)]
        fn __action17<'input>(
            input: &'input str,
            (_, definitions, _): (usize, Vec<ServiceDefinition>, usize),
            (_, type_definition, _): (usize, Box<TypeTerm>, usize),
        ) -> Vec<ServiceDefinition> {
            {
                let mut new_definitions = Vec::new();
                new_definitions.extend(definitions);
                new_definitions.push(ServiceDefinition::Type(type_definition));
                new_definitions
            }
        }
        #[allow(unused_variables)]
        fn __action18<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<EntityTerm> {
            { EntityTerm::new_boxed(name, Vec::new(), Vec::new()) }
        }
        #[allow(unused_variables)]
        fn __action19<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, fields, _): (usize, Vec<Box<FieldTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<EntityTerm> {
            { EntityTerm::new_boxed(name, Vec::new(), fields) }
        }
        #[allow(unused_variables)]
        fn __action20<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, aspects, _): (usize, Vec<Box<NameTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<EntityTerm> {
            { EntityTerm::new_boxed(name, aspects, Vec::new()) }
        }
        #[allow(unused_variables)]
        fn __action21<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, aspects, _): (usize, Vec<Box<NameTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, fields, _): (usize, Vec<Box<FieldTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<EntityTerm> {
            { EntityTerm::new_boxed(name, aspects, fields) }
        }
        #[allow(unused_variables)]
        fn __action22<'input>(
            input: &'input str,
            (_, aspect_name, _): (usize, Box<NameTerm>, usize),
        ) -> Vec<Box<NameTerm>> {
            {
                let mut aspect_names: Vec<Box<NameTerm>> = Vec::new();
                aspect_names.push(aspect_name);
                aspect_names
            }
        }
        #[allow(unused_variables)]
        fn __action23<'input>(
            input: &'input str,
            (_, aspect_names, _): (usize, Vec<Box<NameTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, aspect_name, _): (usize, Box<NameTerm>, usize),
        ) -> Vec<Box<NameTerm>> {
            {
                let mut new_aspect_names: Vec<Box<NameTerm>> = Vec::new();
                new_aspect_names.extend(aspect_names);
                new_aspect_names.push(aspect_name);
                new_aspect_names
            }
        }
        #[allow(unused_variables)]
        fn __action24<'input>(
            input: &'input str,
            (_, field, _): (usize, Box<FieldTerm>, usize),
        ) -> Vec<Box<FieldTerm>> {
            {
                let mut fields: Vec<Box<FieldTerm>> = Vec::new();
                fields.push(field);
                fields
            }
        }
        #[allow(unused_variables)]
        fn __action25<'input>(
            input: &'input str,
            (_, fields, _): (usize, Vec<Box<FieldTerm>>, usize),
            (_, field, _): (usize, Box<FieldTerm>, usize),
        ) -> Vec<Box<FieldTerm>> {
            {
                let mut new_fields: Vec<Box<FieldTerm>> = Vec::new();
                new_fields.extend(fields);
                new_fields.push(field);
                new_fields
            }
        }
        #[allow(unused_variables)]
        fn __action26<'input>(
            input: &'input str,
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, type_name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<FieldTerm> {
            FieldTerm::new_boxed(name, type_name)
        }
        #[allow(unused_variables)]
        fn __action27<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, params, _): (usize, Vec<Box<ParamTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<ActionTerm> {
            { ActionTerm::new_boxed(name, params, None) }
        }
        #[allow(unused_variables)]
        fn __action28<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, params, _): (usize, Vec<Box<ParamTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, returns, _): (usize, Box<ReturnsTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<ActionTerm> {
            { ActionTerm::new_boxed(name, params, Some(returns)) }
        }
        #[allow(unused_variables)]
        fn __action29<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, returns, _): (usize, Box<ReturnsTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<FunctionTerm> {
            { FunctionTerm::new_boxed(name, Vec::new(), returns) }
        }
        #[allow(unused_variables)]
        fn __action30<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, params, _): (usize, Vec<Box<ParamTerm>>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, returns, _): (usize, Box<ReturnsTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<FunctionTerm> {
            { FunctionTerm::new_boxed(name, params, returns) }
        }
        #[allow(unused_variables)]
        fn __action31<'input>(
            input: &'input str,
            (_, name, _): (usize, Box<NameTerm>, usize),
        ) -> Box<ReturnsTerm> {
            ReturnsTerm::new_boxed(name)
        }
        #[allow(unused_variables)]
        fn __action32<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
        ) -> Box<ReturnsTerm> {
            ReturnsTerm::new_arrayed_boxed(name)
        }
        #[allow(unused_variables)]
        fn __action33<'input>(
            input: &'input str,
            (_, param, _): (usize, Box<ParamTerm>, usize),
        ) -> Vec<Box<ParamTerm>> {
            {
                let mut params: Vec<Box<ParamTerm>> = Vec::new();
                params.push(param);
                params
            }
        }
        #[allow(unused_variables)]
        fn __action34<'input>(
            input: &'input str,
            (_, params, _): (usize, Vec<Box<ParamTerm>>, usize),
            (_, param, _): (usize, Box<ParamTerm>, usize),
        ) -> Vec<Box<ParamTerm>> {
            {
                let mut new_params: Vec<Box<ParamTerm>> = Vec::new();
                new_params.extend(params);
                new_params.push(param);
                new_params
            }
        }
        #[allow(unused_variables)]
        fn __action35<'input>(
            input: &'input str,
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, type_name, _): (usize, Box<NameTerm>, usize),
        ) -> Box<ParamTerm> {
            ParamTerm::new_boxed(name, type_name)
        }
        #[allow(unused_variables)]
        fn __action36<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, resolved_type_name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<TypeTerm> {
            { TypeTerm::new_boxed(name, resolved_type_name) }
        }
        #[allow(unused_variables)]
        fn __action37<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, resolved_type_name, _): (usize, Box<NameTerm>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Box<TypeTerm> {
            { TypeTerm::new_boxed(name, resolved_type_name) }
        }
        #[allow(unused_variables)]
        fn __action38<'input>(
            input: &'input str,
            (_, value, _): (usize, &'input str, usize),
        ) -> Box<NameTerm> {
            NameTerm::new_boxed(value.to_string())
        }
        pub trait __ToTriple<'input> {
            fn to_triple(
                value: Self,
            ) -> Result<
                (usize, Token<'input>, usize),
                __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
            >;
        }
        impl<'input> __ToTriple<'input> for (usize, Token<'input>, usize) {
            fn to_triple(
                value: Self,
            ) -> Result<
                (usize, Token<'input>, usize),
                __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
            > {
                Ok(value)
            }
        }
        impl<'input> __ToTriple<'input>
        for Result<(usize, Token<'input>, usize), &'static str> {
            fn to_triple(
                value: Self,
            ) -> Result<
                (usize, Token<'input>, usize),
                __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
            > {
                match value {
                    Ok(v) => Ok(v),
                    Err(error) => {
                        Err(__lalrpop_util::ParseError::User {
                            error,
                        })
                    }
                }
            }
        }
    }
    pub use parse_error::{ParseError, ParseErrorType};
    pub use parser::Parser;
}
pub mod visitor {
    mod visitor {
        use super::super::ast::{
            ActionTerm, EntityTerm, FieldTerm, FunctionTerm, ModuleTerm, NameTerm,
            ParamTerm, ReturnsTerm, ServiceTerm, TypeTerm,
        };
        use super::VisitorError;
        pub trait Visitor {
            fn process_name(&mut self, _term: &NameTerm) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_type(&mut self, _term: &TypeTerm) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_param(&mut self, _term: &ParamTerm) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_returns(
                &mut self,
                _term: &ReturnsTerm,
            ) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_field(&mut self, _term: &FieldTerm) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_function(
                &mut self,
                _term: &FunctionTerm,
            ) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_action(
                &mut self,
                _term: &ActionTerm,
            ) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_entity(
                &mut self,
                _term: &EntityTerm,
            ) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_service(
                &mut self,
                _term: &ServiceTerm,
            ) -> Result<(), VisitorError> {
                Ok(())
            }
            fn process_module(
                &mut self,
                _term: &ModuleTerm,
            ) -> Result<(), VisitorError> {
                Ok(())
            }
        }
    }
    mod visitor_error {
        use std::fmt;
        pub struct VisitorError {
            message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for VisitorError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "VisitorError",
                    "message",
                    &&self.message,
                )
            }
        }
        impl VisitorError {
            pub fn new(message: String) -> VisitorError {
                VisitorError { message }
            }
            pub fn get_message(&self) -> String {
                self.message.clone()
            }
        }
        impl fmt::Display for VisitorError {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Visitor reason: "],
                            &[::core::fmt::ArgumentV1::new_display(&self.message)],
                        ),
                    )
            }
        }
    }
    pub use visitor::Visitor;
    pub use visitor_error::VisitorError;
}
