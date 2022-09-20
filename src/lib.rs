pub mod accessibilities;
pub mod classes;
pub mod enums;
pub mod field;
pub mod interfaces;
pub mod methods;
pub mod modifiers;

use crate::enums::PlantUMLEnumVariant;
use crate::field::parse_field;
use crate::field::PlantUMLField;
use crate::methods::parse_method;
use crate::methods::PlantUMLMethod;
use nom::IResult;

#[derive(Debug)]
pub enum UMLElement<'a> {
    UMLField(PlantUMLField<'a>),
    UMLMethod(PlantUMLMethod<'a>),
    UMLEnumVariant(PlantUMLEnum<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct PlantUMLEnum<'a> {
    pub variants: Vec<PlantUMLEnumVariant<'a>>,
}
