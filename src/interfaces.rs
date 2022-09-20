use crate::parse_method;
use crate::PlantUMLMethod;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct PlantUMLInterface<'a> {
    pub name: &'a str,
    pub methods: Vec<PlantUMLMethod<'a>>,
}

pub fn parse_interface_name(element: &str) -> IResult<&str, &str> {
    delimited(
        tag("interface "),
        take_while(|c: char| !c.is_whitespace()),
        tag(" {"),
    )(element)
}

pub fn parse_interface(element: &str) -> IResult<&str, PlantUMLInterface> {
    let (rest, name) = parse_interface_name(element)?;
    let (rest, methods) = terminated(many0(terminated(parse_method, char('\n'))), char('}'))(rest)?;

    Ok((rest, PlantUMLInterface { name, methods }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibilities::Accessibility;
    use crate::methods::MethodArgument;
    use crate::modifiers::Modifier;
    #[test]
    fn parse_interface_name_works() {
        let input = "interface ILlanta {\n";
        let (rest, name) = parse_interface_name(input).expect("The name couldn't be parsed!");
        assert_eq!("ILlanta", name);
        assert_eq!("\n", rest);
    }

    #[test]
    fn parse_interface_name_fails() {
        let input = "interface ILlanta asdf {\n";
        let result = parse_interface_name(input);
        assert_eq!(true, result.is_err());

        let input = "interface ILlanta asdf{\n";
        let result = parse_interface_name(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn parse_interface_works() {
        let input = "interface ICarro {
    - void Avanzar()
    + string Serie()
    # boolean GetEncendido()
    - void SetEncendido(boolean encendido)
}\n";
        let (rest, interface) = parse_interface(input).expect("Coudln't parse the interface!");
        assert_eq!("\n", rest);
        assert_eq!(
            interface,
            PlantUMLInterface {
                name: "ICarro",
                methods: vec![
                    PlantUMLMethod {
                        accessibility: Accessibility::Private,
                        modifier: Modifier::None,
                        name: "Avanzar",
                        arguments: vec![],
                        return_type: "void",
                    },
                    PlantUMLMethod {
                        accessibility: Accessibility::Public,
                        modifier: Modifier::None,
                        name: "Serie",
                        arguments: vec![],
                        return_type: "string",
                    },
                    PlantUMLMethod {
                        accessibility: Accessibility::Protected,
                        modifier: Modifier::None,
                        name: "GetEncendido",
                        arguments: vec![],
                        return_type: "boolean"
                    },
                    PlantUMLMethod {
                        accessibility: Accessibility::Private,
                        modifier: Modifier::None,
                        name: "SetEncendido",
                        arguments: vec![MethodArgument {
                            name: "encendido",
                            argument_type: "boolean"
                        }],
                        return_type: "void"
                    }
                ]
            }
        );
    }

    #[test]
    fn parse_interface_fails() {
        let input = "interface ICarro  assd{
    - void Avanzar()
    + string Serie()
    # boolean GetEncendido()
    - void SetEncendido(boolean encendido)
}\n";
        let result = parse_interface(input);
        assert_eq!(true, result.is_err());
    }
}
