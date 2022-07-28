use crate::accessibilities::parse_accessibility;
use crate::accessibilities::Accessibility;
use crate::modifiers::parse_modifier;
use crate::modifiers::Modifier;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::bytes::streaming::tag;
use nom::character::complete::space0;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::Err;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct PlantUMLField<'a> {
    pub name: &'a str,
    pub field_type: &'a str,
    pub accessibility: Accessibility,
    pub modifier: Modifier,
}

pub fn doesnt_have_spaces<'a>(element: (&'a str, &'a str)) -> IResult<&str, &str> {
    if !element.1.contains(char::is_whitespace) && !element.1.is_empty() {
        Ok(element)
    } else {
        Err(Err::Error(Error::from_error_kind(
            element.1,
            ErrorKind::Fail,
        )))
    }
}

pub fn parse_field(element: &str) -> IResult<&str, PlantUMLField> {
    let (rest, accessibility) = parse_accessibility(element.trim())?;
    let (rest, modifier) = parse_modifier(rest.trim())?;
    let (rest, name) = parse_field_name(rest.trim())?;
    let (rest, field_type) = parse_field_type(rest.trim())?;

    Ok((
        rest,
        PlantUMLField {
            name,
            field_type,
            accessibility,
            modifier,
        },
    ))
}

pub fn parse_field_name(element: &str) -> IResult<&str, &str> {
    let until_colon = take_until(":")(element)?;
    doesnt_have_spaces(until_colon)
}

pub fn parse_field_type(element: &str) -> IResult<&str, &str> {
    let left_delimiter = pair(tag(":"), space0);
    let (rest, f_type) = preceded(left_delimiter, take_while(|_| true))(element)?;
    let f_type = f_type.trim();
    if !f_type.is_empty() {
        Ok((rest, f_type))
    } else {
        Err(Err::Error(Error::from_error_kind(f_type, ErrorKind::Fail)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plantuml_field() {
        let input = "- nombreEquipo: String";
        let output = parse_field(input).unwrap();

        assert_eq!(
            output.1,
            PlantUMLField {
                name: "nombreEquipo",
                field_type: "String",
                accessibility: Accessibility::Private,
                modifier: Modifier::None
            }
        )
    }

    //PARSE FIELD TYPE
    #[test]
    fn parse_type() {
        let input = ": String";
        let output = parse_field_type(input).unwrap();

        assert_eq!(output, ("", "String"));
    }
    #[test]
    fn parse_field_type_fails() {
        let input = ": ";
        let output = parse_field_type(input);

        assert!(output.is_err());
    }

    //PARSE NAME
    #[test]
    fn parse_name() {
        let input = "nombreEquipo: String";
        let output = parse_field_name(input).unwrap();

        assert_eq!(output, (": String", "nombreEquipo"));
    }
    #[test]
    fn parse_name_fails() {
        let input = ": String";
        let output = parse_field_name(input);

        assert!(output.is_err());
    }
    #[test]
    fn name_cant_have_spaces() {
        let input = "nombre Equipo: String";
        let output = parse_field_name(input);

        assert!(output.is_err());
    }
}
