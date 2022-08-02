pub mod accessibilities;
pub mod enums;
pub mod field;
pub mod methods;
pub mod modifiers;

use crate::enums::PlantUMLEnumVariant;
use crate::field::parse_field;
use crate::field::PlantUMLField;
use crate::methods::parse_method;
use crate::methods::PlantUMLMethod;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug)]
pub enum UMLElement<'a> {
    UMLField(PlantUMLField<'a>),
    UMLMethod(PlantUMLMethod<'a>),
    UMLEnumVariant(PlantUMLEnum<'a>),
}

#[derive(Debug, PartialEq)]
pub struct PlantUMLClass<'a> {
    pub name: &'a str,
    pub fields: Vec<PlantUMLField<'a>>,
    pub methods: Vec<PlantUMLMethod<'a>>,
}

#[derive(Debug)]
pub struct PlantUMLInterface<'a> {
    pub methods: Vec<PlantUMLMethod<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct PlantUMLEnum<'a> {
    pub variants: Vec<PlantUMLEnumVariant<'a>>,
}

pub fn parse_class_name(element: &str) -> IResult<&str, &str> {
    delimited(
        tag("class "),
        take_while(|c: char| !c.is_whitespace()),
        tag(" {"),
    )(element)
}

pub fn parse_class_elements<'a, F, T>(parser: F, element: &'a str) -> IResult<&'a str, Vec<T>>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    many0(terminated(&parser, char('\n')))(element)
}

pub fn parse_class(element: &str) -> IResult<&str, PlantUMLClass> {
    let (rest, name) = parse_class_name(element.trim_start())?;
    let (rest, fields) = parse_class_elements(parse_field, rest)?;
    let (rest, methods) = parse_class_elements(parse_method, rest)?;

    Ok((
        rest,
        PlantUMLClass {
            name,
            fields,
            methods,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibilities::Accessibility;
    use crate::field::parse_field;
    use crate::methods::parse_method;
    use crate::methods::MethodArgument;
    use crate::modifiers::Modifier;

    #[test]
    fn parse_class_methods_works() {
        let input = "\t+ {static} void main(String[] args)\n\t- AccionUsuario preguntarUsuario()\n";
        let (rest, methods) =
            parse_class_elements(parse_method, input).expect("Couldn't parse class methods!");

        assert_eq!(rest, "");
        assert_eq!(
            methods,
            vec![
                PlantUMLMethod {
                    accessibility: Accessibility::Public,
                    name: "main",
                    return_type: "void",
                    modifier: Modifier::Static,
                    arguments: vec![MethodArgument {
                        name: "args",
                        argument_type: "String[]"
                    }]
                },
                PlantUMLMethod {
                    accessibility: Accessibility::Private,
                    name: "preguntarUsuario",
                    return_type: "AccionUsuario",
                    modifier: Modifier::None,
                    arguments: vec![]
                }
            ]
        )
    }

    #[test]
    fn parse_class_fields_works() {
        let input = "\t- analizador: AnalizadorEquipos\n\t- analizador: AnalizadorEquipos\n";
        let (rest, fields) =
            parse_class_elements(parse_field, input).expect("Couldn't parse class fields!");
        assert_eq!(
            fields,
            vec![
                PlantUMLField {
                    accessibility: Accessibility::Private,
                    name: "analizador",
                    modifier: Modifier::None,
                    field_type: "AnalizadorEquipos"
                },
                PlantUMLField {
                    accessibility: Accessibility::Private,
                    name: "analizador",
                    modifier: Modifier::None,
                    field_type: "AnalizadorEquipos"
                }
            ]
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn parse_class_name_works() {
        let input = "class VentanaPrograma {\n";
        let (rest, output) = parse_class_name(input).expect("Can't parse the class name");
        assert_eq!(rest, "\n");
        assert_eq!(output, "VentanaPrograma");
    }

    #[test]
    fn parse_class_works() {
        let input = "class VentanaPrograma {
            - analizador: AnalizadorEquipos
            + {static} void main(String[] args)
            + AccionUsuario preguntarUsuario()
        }";
        let (rest, class) = parse_class(input).expect("Can't parse the class");
        assert_eq!("        }", rest);
        assert_eq!(
            class,
            PlantUMLClass {
                name: "VentanaPrograma",
                fields: vec![PlantUMLField {
                    accessibility: Accessibility::Private,
                    modifier: Modifier::None,
                    field_type: "AnalizadorEquipos",
                    name: "analizador"
                }],
                methods: vec![
                    PlantUMLMethod {
                        accessibility: Accessibility::Public,
                        modifier: Modifier::Static,
                        name: "main",
                        return_type: "void",
                        arguments: vec![MethodArgument {
                            argument_type: "String[]",
                            name: "args"
                        }]
                    },
                    PlantUMLMethod {
                        accessibility: Accessibility::Public,
                        modifier: Modifier::None,
                        name: "preguntarUsuario",
                        return_type: "AccionUsuario",
                        arguments: vec![]
                    }
                ]
            }
        )
    }
}
