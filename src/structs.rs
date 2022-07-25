use crate::Accessibility;
use crate::Modifier;

#[derive(Debug)]
pub enum UMLElement<'a> {
    UMLField(PlantUMLField<'a>),
    UMLMethod(PlantUMLMethod<'a>),
    UMLEnumVariant(PlantUMLEnum<'a>),
}

#[derive(Debug)]
pub struct PlantUMLField<'a> {
    pub name: &'a str,
    pub field_type: &'a str,
    pub accessibility: Accessibility,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug)]
pub struct MethodArgument<'a> {
    pub name: &'a str,
    pub argument_type: &'a str,
}

#[derive(Debug)]
pub struct PlantUMLMethod<'a> {
    pub name: &'a str,
    pub return_type: &'a str,
    pub accessibility: Accessibility,
    pub arguments: Vec<MethodArgument<'a>>,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug)]
pub struct PlantUMLEnumVariant<'a> {
    pub name: &'a str,
}

#[derive(Debug)]
pub struct PlantUMLClass<'a> {
    pub fields: Vec<PlantUMLField<'a>>,
    pub methods: Vec<PlantUMLMethod<'a>>,
}

#[derive(Debug)]
pub struct PlantUMLInterface<'a> {
    pub methods: Vec<PlantUMLMethod<'a>>,
}

#[derive(Debug)]
pub struct PlantUMLEnum<'a> {
    pub variants: Vec<PlantUMLEnumVariant<'a>>,
}
