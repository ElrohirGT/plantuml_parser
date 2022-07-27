use nom::combinator::opt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::sequence::preceded;
use nom::multi::many1;
use nom::branch::alt;
use crate::doesnt_have_spaces;
use crate::Accessibility;
use crate::IResult;
use crate::Modifier;
use nom::bytes::complete::take_until;

#[derive(Debug, PartialEq)]
pub struct MethodArgument<'a> {
    pub name: &'a str,
    pub argument_type: &'a str,
}

pub fn parse_method_argument(element: &str) -> IResult<&str, MethodArgument> {
    let type_parser = take_until(" ");
    let mut name_parser = alt((take_until(","), take_until(")")));

    let (rest, argument_type) = type_parser(element.trim())?;
    let parsed_name = name_parser(rest.trim())?;
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
    many1(preceded(preceded(opt(tag(",")), space0), parse_method_argument))(element)
}

#[derive(Debug)]
pub struct PlantUMLMethod<'a> {
    pub name: &'a str,
    pub return_type: &'a str,
    pub accessibility: Accessibility,
    pub arguments: Vec<MethodArgument<'a>>,
    pub modifiers: Modifier,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(output.1[0], MethodArgument {
            name: "nombre_producto",
            argument_type: "String"
        });
        assert_eq!(output.1[1], MethodArgument {
            name: "cantidad",
            argument_type: "int"
        });

    }
    #[test]
    fn cant_parse_method_argument() {
        let input = "String nombre producto)"; //Parameter has spaces
        let output = parse_method_arguments(input);
        assert!(output.is_err());

        let input = "String nombre_producto"; //Doesn't have comma or parenthesis
        let output = parse_method_arguments(input);
        assert!(output.is_err());

        let input = "nombre_producto"; //Doesn't have type
        let output = parse_method_arguments(input);
        assert!(output.is_err());
    }
}