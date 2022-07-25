use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Abstract,
    Static,
    Virtual,
    Constructor,
    None,
}

pub fn parse_modifier(element: &str) -> IResult<&str, Modifier> {
    opt(alt((
        tag("{static}"),
        tag("{abstract}"),
        tag("{virtual}"),
        tag("{ctor}"),
    )))(element)
    .map(|(rest, simbol)| match simbol {
        Some(s) => match s {
            "{static}" => (rest, Modifier::Static),
            "{abstract}" => (rest, Modifier::Abstract),
            "{virtual}" => (rest, Modifier::Virtual),
            "{ctor}" => (rest, Modifier::Constructor),
            _ => (rest, Modifier::None),
        },
        None => (rest, Modifier::None),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn optional_parsing_works() {
        let input = "nombreEquipo: String";

        let parser_output = parse_modifier(input).unwrap();
        assert_eq!(parser_output, (input, Modifier::None));
    }

    #[test]
    fn parsing_works() {
        let cases = HashMap::from([
            ("{static}", Modifier::Static),
            ("{abstract}", Modifier::Abstract),
            ("{virtual}", Modifier::Virtual),
            ("{ctor}", Modifier::Constructor),
        ]);
        for modifier in cases.keys() {
            let input = format!("{} nombreEquipo: String", modifier);

            let parser_output = parse_modifier(&input).unwrap();
            assert_eq!(parser_output.0, " nombreEquipo: String");
            assert_eq!(&parser_output.1, cases.get(modifier).unwrap());
        }
    }
}
