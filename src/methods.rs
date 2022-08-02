use crate::accessibilities::parse_accessibility;
use crate::accessibilities::Accessibility;
use crate::field::doesnt_have_spaces;
use crate::modifiers::parse_modifier;
use crate::modifiers::Modifier;
use crate::IResult;
use nom::branch::alt;
use nom::bytes::complete::take_until1;
use nom::character::complete::space0;
use nom::character::streaming::char;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;

#[derive(Debug, PartialEq)]
pub struct MethodArgument<'a> {
    pub name: &'a str,
    pub argument_type: &'a str,
}

pub fn parse_method_argument(element: &str) -> IResult<&str, MethodArgument> {
    let type_parser = take_until1(" ");
    let mut name_parser = alt((take_until1(","), take_until1(")")));

    let (rest, argument_type) = type_parser(element.trim_start())?;
    let parsed_name = name_parser(rest.trim_start())?;
    let (rest, name) = doesnt_have_spaces(parsed_name)?;

    Ok((
        rest,
        MethodArgument {
            name,
            argument_type,
        },
    ))
}

pub fn parse_method_arguments(element: &str) -> IResult<&str, Vec<MethodArgument>> {
    many0(preceded(
        preceded(opt(char(',')), space0),
        parse_method_argument,
    ))(element)
}

#[derive(Debug, PartialEq)]
pub struct PlantUMLMethod<'a> {
    pub name: &'a str,
    pub return_type: &'a str,
    pub accessibility: Accessibility,
    pub arguments: Vec<MethodArgument<'a>>,
    pub modifier: Modifier,
}

// + void setNombreEquipo(String nombre)
pub fn parse_method(element: &str) -> IResult<&str, PlantUMLMethod> {
    let (rest, accessibility) = parse_accessibility(element.trim_start())?;
    let (rest, modifier) = parse_modifier(rest.trim_start())?;
    let (rest, return_type) = take_until1(" ")(rest.trim_start())?;
    let (rest, name) = take_until1("(")(rest.trim_start())?;
    let (rest, arguments) = parse_method_arguments(rest[1..].trim_start())?;

    Ok((
        &rest[1..],
        PlantUMLMethod {
            accessibility,
            modifier,
            return_type,
            name,
            arguments,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    //METHOD PARSER
    #[test]
    fn parse_method_without_parameters() {
        let input = "\t+ void setNombreEquipo()\n";
        let (_, output) = parse_method(input).unwrap();
        assert_eq!(
            output,
            PlantUMLMethod {
                name: "setNombreEquipo",
                accessibility: Accessibility::Public,
                modifier: Modifier::None,
                return_type: "void",
                arguments: vec![]
            }
        );
    }
    #[test]
    fn parse_method_without_modifier() {
        let input = "\t+ void setNombreEquipo(String nombre)\n";
        let output = parse_method(input);

        assert!(output.is_ok());
        let (_, output) = output.unwrap();
        assert_eq!(
            output,
            PlantUMLMethod {
                name: "setNombreEquipo",
                accessibility: Accessibility::Public,
                modifier: Modifier::None,
                return_type: "void",
                arguments: vec![MethodArgument {
                    name: "nombre",
                    argument_type: "String"
                }]
            }
        );
    }
    #[test]
    fn parse_method_with_modifier() {
        let input = "\t+ {abstract} void setNombreEquipo(String nombre)\n";
        let output = parse_method(input);

        assert!(output.is_ok());
        let (_, output) = output.unwrap();
        assert_eq!(
            output,
            PlantUMLMethod {
                name: "setNombreEquipo",
                accessibility: Accessibility::Public,
                modifier: Modifier::Abstract,
                return_type: "void",
                arguments: vec![MethodArgument {
                    name: "nombre",
                    argument_type: "String"
                }]
            }
        );
    }
    #[test]
    fn parse_method_multiple_params() {
        let input = "\t+ {abstract} void setNombreEquipo(String nombre, int posicion)\n";
        let output = parse_method(input);

        assert!(output.is_ok());
        let (_, output) = output.unwrap();
        assert_eq!(
            output,
            PlantUMLMethod {
                name: "setNombreEquipo",
                accessibility: Accessibility::Public,
                modifier: Modifier::Abstract,
                return_type: "void",
                arguments: vec![
                    MethodArgument {
                        name: "nombre",
                        argument_type: "String"
                    },
                    MethodArgument {
                        name: "posicion",
                        argument_type: "int"
                    }
                ]
            }
        );
    }
    #[test]
    fn parse_method_fails() {
        let mut output = parse_method("\t {abstract} void setNombreEquipo(String nombre)\n");

        assert!(output.is_err());
        output = parse_method("\t {abstract} setNombreEquipo(String nombre)\n");
        assert!(output.is_err());
    }

    //PARSING METHOD ARGUMENTS
    #[test]
    fn parse_mthd_argument_paren() {
        let input = "String nombre)";
        let output = parse_method_argument(input);

        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output.1.name, "nombre");
        assert_eq!(output.1.argument_type, "String");
    }
    #[test]
    fn parse_mthd_argument_comma() {
        let input = "String nombre,";
        let output = parse_method_argument(input);

        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output.1.name, "nombre");
        assert_eq!(output.1.argument_type, "String");
    }
    #[test]
    fn parse_mutliple_method_arguments_spaces() {
        let input = "String nombre_producto, int cantidad)";
        let output = parse_method_arguments(input);

        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(
            output.1[0],
            MethodArgument {
                name: "nombre_producto",
                argument_type: "String"
            }
        );
        assert_eq!(
            output.1[1],
            MethodArgument {
                name: "cantidad",
                argument_type: "int"
            }
        );
    }
    #[test]
    fn cant_parse_method_argument() {
        let input = "String nombre producto)"; //Parameter has spaces
        let output = parse_method_argument(input);
        assert!(output.is_err(), "Parameter shouldn't have spaces!");

        let input = "String nombre_producto"; //Doesn't have comma or parenthesis
        let output = parse_method_argument(input);
        assert!(
            output.is_err(),
            "Parameter should have a comma or parenthesis at the end!"
        );

        let input = "nombre_producto"; //Doesn't have type
        let output = parse_method_argument(input);
        assert!(output.is_err(), "Parameter needs a type");
    }
}
