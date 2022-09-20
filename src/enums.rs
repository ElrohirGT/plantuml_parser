use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct PlantUMLEnum<'a> {
    pub name: &'a str,
    pub variants: Vec<PlantUMLEnumVariant<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PlantUMLEnumVariant<'a> {
    pub name: &'a str,
}

pub fn parse_enum_variant(element: &str) -> IResult<&str, PlantUMLEnumVariant> {
    let (rest, name) = terminated(
        take_till(|c: char| c.is_whitespace() || c == '}'),
        char('\n'),
    )(element.trim_start())?;
    Ok((rest, PlantUMLEnumVariant { name }))
}

pub fn parse_enum(element: &str) -> IResult<&str, PlantUMLEnum> {
    let (rest, name) = delimited(
        tag("enum "),
        take_while(|c: char| !c.is_whitespace()),
        tag(" {"),
    )(element)?;

    let (rest, variants) = terminated(many0(parse_enum_variant), tag("}"))(rest)?;

    Ok((rest, PlantUMLEnum { name, variants }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_enum_succeeds() {
        let input = "enum AccionUsuario {
    SALIR
    TOTAL_GOLES
    TOTAL_TIROS_ESQUINA
}\n";
        let (rest, enu) = parse_enum(input).expect("Couldn't parse the enum!");
        assert_eq!("\n", rest);
        assert_eq!(
            enu,
            PlantUMLEnum {
                name: "AccionUsuario",
                variants: vec![
                    PlantUMLEnumVariant { name: "SALIR" },
                    PlantUMLEnumVariant {
                        name: "TOTAL_GOLES"
                    },
                    PlantUMLEnumVariant {
                        name: "TOTAL_TIROS_ESQUINA"
                    },
                ]
            }
        )
    }

    #[test]
    fn parse_enum_fails() {
        let input = "enum Accion Usuario {
    SALIR
    TOTAL_GOLES
    TOTAL_TIROS_ESQUINA
}";
        let res = parse_enum(input);
        assert_eq!(true, res.is_err());

        let input = "enum Accion Usuario {
    SALIR
    TOTAL GOLES
    TOTAL_TIROS_ESQUINA
}";
        let res = parse_enum(input);
        assert_eq!(true, res.is_err());
    }

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
