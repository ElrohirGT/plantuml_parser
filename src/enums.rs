use crate::field::doesnt_have_spaces;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct PlantUMLEnumVariant<'a> {
    pub name: &'a str,
}

pub fn parse_enum_variant(element: &str) -> IResult<&str, PlantUMLEnumVariant> {
    let (rest, name) = doesnt_have_spaces(("", element.trim()))?;
    Ok((rest, PlantUMLEnumVariant { name }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_enum_variant_succeeds() {
        let (rest, output) =
            parse_enum_variant("\tSALIR\n").expect("Can't parse `SALIR` enum variant");
        assert_eq!(output, PlantUMLEnumVariant { name: "SALIR" });
        assert_eq!("", rest);

        let (rest, output) =
            parse_enum_variant("\tTOTAL_GOLES\n").expect("Can't parse `TOTAL_GOLES` enum variant");
        assert_eq!(
            output,
            PlantUMLEnumVariant {
                name: "TOTAL_GOLES"
            }
        );
        assert_eq!("", rest);
    }

    #[test]
    fn parse_enum_variant_fails() {
        let error = parse_enum_variant("\tTOTAL GOLES\n");
        assert!(error.is_err());
    }
}
