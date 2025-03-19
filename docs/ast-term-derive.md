# ASTTerm Derive Macro Documentation

The `ASTTerm` derive macro provides functionality for working with abstract syntax trees (AST) in Rust projects. It generates boilerplate code for common AST node operations, including getter/setter methods and trait implementations.

## Key Features

1. **Automatically Generated Methods**
- Getter and setter methods for struct fields
- Mutable reference getters
- Default constructor (`new()`) for structs with named fields

2. **Generated Traits**
```rust
impl ASTTerm for YourStruct {}
impl Visitable for YourStruct {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error>;
}
impl Convertable for YourStruct {
    fn try_convert<'c, T: Convertable>(&'c self) -> Option<&'c T>;
}
```

3. **Field Attributes**
- `#[prop]`: Generates basic getter/setter methods
- `#[subnode_prop]`: Marks fields that should be processed during visitor calls

## Example Usage

### Struct Definition
```rust
#[derive(ASTTerm)]
struct ActionDeclarationTerm {
    #[prop]
    location: Location,
    
    #[subnode_prop]
    action: Box<ActionTerm>,
    
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    
    #[subnode_prop]
    parameters: Box<ParametersBlockTerm>,
    
    #[subnode_prop]
    returns: Option<Box<ReturnsDeclarationTerm>>,
    
    #[subnode_prop]
    semicolumn: Box<SemicolumnTerm>,
}
```

### Generated Methods

#### Getters and Setters
```rust
fn location(&self) -> &Location {
    &self.location
}

fn location_mut(&mut self) -> &mut Location {
    &mut self.location
}

fn set_location(value: Location) {
    self.location = value;
}
```

#### Mutable Getters for Subnodes
```rust
fn action_mut(&mut self) -> &mut Box<ActionTerm> {
    &mut self.action
}

fn identifier_mut(&mut self) -> &mut Box<IdentifierTerm> {
    &mut self.identifier
}

fn parameters_mut(&mut self) -> &mut Box<ParametersBlockTerm> {
    &mut self.parameters
}

fn returns_mut(&mut self) -> Option<&mut Box<ReturnsDeclarationTerm>> {
    Some(&mut self.returns)
}

fn semicolumn_mut(&mut self) -> &mut Box<SemicolumnTerm> {
    &mut self.semicolumn
}
```

#### Constructor
```rust
fn new(
    location: Location,
    action: Box<ActionTerm>,
    identifier: Box<IdentifierTerm>,
    parameters: Box<ParametersBlockTerm>,
    returns: Option<Box<ReturnsDeclarationTerm>>,
    semicolumn: Box<SemicolumnTerm>,
) -> ActionDeclarationTerm {
    ActionDeclarationTerm {
        location,
        action,
        identifier,
        parameters,
        returns,
        semicolumn,
    }
}
```

## Visitor Pattern Integration

All fields marked with `subnode_prop` implement the visitor pattern:

```rust
impl Visitable for ActionDeclarationTerm {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.process(self)?;
        self.parameters.accept(visitor)?;
        Ok(())
    }
}
```
