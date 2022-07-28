use crate::enums::PlantUMLEnum;
use crate::field::PlantUMLField;
use crate::methods::PlantUMLMethod;

#[derive(Debug)]
pub enum UMLElement<'a> {
    UMLField(PlantUMLField<'a>),
    UMLMethod(PlantUMLMethod<'a>),
    UMLEnumVariant(PlantUMLEnum<'a>),
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
