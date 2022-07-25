use nom::branch::alt;
use nom::character::complete::char;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Accessibility {
    Private,
    Protected,
    Public,
}

pub fn parse_accessibility(element: &str) -> IResult<&str, Accessibility> {
    alt((char('+'), char('-'), char('#')))(element).map(|(rest, simbol)| {
        if simbol == '+' {
            (rest, Accessibility::Public)
        } else if simbol == '-' {
            (rest, Accessibility::Private)
        } else {
            (rest, Accessibility::Protected)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accessibility_private() {
        let (rest, accessibility) = parse_accessibility("- nombreEquipo: String").unwrap();

        assert_eq!(accessibility, Accessibility::Private);
        assert_eq!(rest, " nombreEquipo: String");
    }
    #[test]
    fn parse_accessibility_public() {
        let (rest, accessibility) = parse_accessibility("+ nombreEquipo: String").unwrap();

        assert_eq!(accessibility, Accessibility::Public);
        assert_eq!(rest, " nombreEquipo: String");
    }
    #[test]
    fn parse_accessibility_protected() {
        let (rest, accessibility) = parse_accessibility("# nombreEquipo: String").unwrap();

        assert_eq!(accessibility, Accessibility::Protected);
        assert_eq!(rest, " nombreEquipo: String");
    }
}
