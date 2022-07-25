use nom::IResult;

pub mod structs;
pub use structs::*;

pub mod accessibilities;
pub use accessibilities::*;

pub mod modifiers;
pub use modifiers::*;

pub fn parser_uml_inner_element(element: &str) -> IResult<&str, UMLElement> {
    let (rest, accessibility) = parse_accessibility(element)?;
    Ok((
        "",
        UMLElement::UMLField(PlantUMLField {
            name: "",
            field_type: "",
            accessibility: Accessibility::Private,
            modifiers: vec![],
        }),
    ))
}
