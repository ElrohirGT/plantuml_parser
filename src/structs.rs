use crate::PlantUMLMethod;
use crate::PlantUMLField;

#[derive(Debug)]
pub enum UMLElement<'a> {
    UMLField(PlantUMLField<'a>),
    UMLMethod(PlantUMLMethod<'a>),
    UMLEnumVariant(PlantUMLEnum<'a>),
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
