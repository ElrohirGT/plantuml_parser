//! Lite parser for class diagrams made using [PlantUML](https://plantuml.com/es/). **It currently doesn't support generics!**
//!
//! This is not a plantuml to {insert programming language} converter! But it should be simple to implement one using this library.
//!
//! The list of parsers implemented are:
//! - interfaces (only parses methods)
//! - classes (abstract/static classes included)
//! - enums
//!
//! This library doesn't check if the supplied input generates a logical code structure, if you define a method abstract and static, this library will take your word for it.
//!
//! Here's an example of usage:
//! ```rust
//! use plantuml_parser::parse_uml_from_contents;
//! let input = "class VentanaPrograma {
//! - analizador: AnalizadorEquipos
//! + {ctor} VentanaPrograma()
//! + {static} void main(String[] args)
//! + {abstract} AccionUsuario preguntarUsuario()
//! }
//!
//! enum AccionUsuario {
//! SALIR
//! TOTAL_GOLES
//! TOTAL_TIROS_ESQUINA
//! TOTAL_TARJETAS_AMARILLAS
//! TOTAL_TARJETAS_ROJAS
//! }\n";
//! let (res, content) = parse_uml_from_contents(input).expect("PlantUML couldn't be parsed!");
//! assert_eq!("", res);
//! assert_eq!(content.classes[0].name, "VentanaPrograma")
//! ```

pub mod accessibilities;
pub mod classes;
pub mod enums;
pub mod field;
pub mod interfaces;
pub mod methods;
pub mod modifiers;

use crate::classes::parse_class;
use crate::classes::PlantUMLClass;
use crate::enums::parse_enum;
use crate::enums::PlantUMLEnum;
use crate::field::parse_field;
use crate::field::PlantUMLField;
use crate::interfaces::parse_interface;
use crate::interfaces::PlantUMLInterface;
use crate::methods::parse_method;
use crate::methods::PlantUMLMethod;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct PlantUMLFile<'a> {
    pub classes: Vec<PlantUMLClass<'a>>,
    pub interfaces: Vec<PlantUMLInterface<'a>>,
    pub enums: Vec<PlantUMLEnum<'a>>,
}

fn try_parse_element<'a, OK, P: Fn(&'a str) -> IResult<&'a str, OK>>(
    content: &'a str, ok_array: &mut Vec<OK>,
    err_array: &mut Vec<nom::Err<nom::error::Error<&'a str>>>, parser: P,
) -> &'a str {
    match parser(content) {
        Ok((rest, elm)) => {
            ok_array.push(elm);
            rest
        }
        Err(e) => {
            err_array.push(e);
            match content.find('}') {
                Some(i) => &content[i..],
                None => content,
            }
        }
    }
}

//TODO: This should return a custom error instead of a Vec<String>
pub fn parse_uml_from_contents(content: &str) -> Result<(&str, PlantUMLFile), Vec<String>> {
    let mut trimmed = content.trim_start();
    let mut classes = vec![];
    let mut interfaces = vec![];
    let mut enums = vec![];
    let mut errors = vec![];
    loop {
        match trimmed.find('\n') {
            None => break,
            Some(i) => {
                let line = &trimmed[..i];
                if line.starts_with("class") {
                    trimmed = try_parse_element(trimmed, &mut classes, &mut errors, parse_class);
                } else if line.starts_with("interface") {
                    trimmed =
                        try_parse_element(trimmed, &mut interfaces, &mut errors, parse_interface);
                } else if line.starts_with("enum") {
                    trimmed = try_parse_element(trimmed, &mut enums, &mut errors, parse_enum);
                } else {
                    trimmed = &trimmed[i..];
                }
                trimmed = trimmed.trim_start();
            }
        };
    }

    if errors.len() != 0 {
        Err(errors.iter().map(|e| format!("{:?}", e)).collect())
    } else {
        Ok((
            trimmed,
            PlantUMLFile {
                classes,
                interfaces,
                enums,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibilities::Accessibility;
    use crate::enums::PlantUMLEnumVariant;
    use crate::methods::MethodArgument;
    use crate::modifiers::Modifier;
    use crate::PlantUMLClass;

    #[test]
    fn parse_content_fails() {
        let input = "class Ventana Programa {
    - analizador: AnalizadorEquipos
    + {ctor} VentanaPrograma()
    + {static} void main(String[] args)
    + {abstract} AccionUsuario preguntarUsuario()
}

enum AccionUsuario {
    SALIR
    TOTAL_ GOLES
    TOTAL_TIROS_ESQUINA
    TOTAL_TARJETAS_AMARILLAS
    TOTAL_TARJETAS_ROJAS
}\n";
        let res = parse_uml_from_contents(input);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn parse_content_works() {
        let input = "class VentanaPrograma {
    - analizador: AnalizadorEquipos
    + {ctor} VentanaPrograma()
    + {static} void main(String[] args)
    + {abstract} AccionUsuario preguntarUsuario()
}

enum AccionUsuario {
    SALIR
    TOTAL_GOLES
    TOTAL_TIROS_ESQUINA
    TOTAL_TARJETAS_AMARILLAS
    TOTAL_TARJETAS_ROJAS
}\n";
        let (res, content) = parse_uml_from_contents(input).expect("PlantUML couldn't be parsed!");
        assert_eq!("", res);
        assert_eq!(
            PlantUMLFile {
                enums: vec![PlantUMLEnum {
                    name: "AccionUsuario",
                    variants: vec![
                        PlantUMLEnumVariant { name: "SALIR" },
                        PlantUMLEnumVariant {
                            name: "TOTAL_GOLES"
                        },
                        PlantUMLEnumVariant {
                            name: "TOTAL_TIROS_ESQUINA"
                        },
                        PlantUMLEnumVariant {
                            name: "TOTAL_TARJETAS_AMARILLAS"
                        },
                        PlantUMLEnumVariant {
                            name: "TOTAL_TARJETAS_ROJAS"
                        },
                    ]
                }],
                interfaces: vec![],
                classes: vec![PlantUMLClass {
                    name: "VentanaPrograma",
                    fields: vec![PlantUMLField {
                        name: "analizador",
                        accessibility: Accessibility::Private,
                        field_type: "AnalizadorEquipos",
                        modifier: Modifier::None
                    }],
                    methods: vec![
                        PlantUMLMethod {
                            name: "VentanaPrograma",
                            accessibility: Accessibility::Public,
                            modifier: Modifier::Constructor,
                            return_type: "",
                            arguments: vec![]
                        },
                        PlantUMLMethod {
                            name: "main",
                            accessibility: Accessibility::Public,
                            modifier: Modifier::Static,
                            return_type: "void",
                            arguments: vec![MethodArgument {
                                name: "args",
                                argument_type: "String[]"
                            }]
                        },
                        PlantUMLMethod {
                            name: "preguntarUsuario",
                            accessibility: Accessibility::Public,
                            modifier: Modifier::Abstract,
                            return_type: "AccionUsuario",
                            arguments: vec![]
                        }
                    ]
                }]
            },
            content
        );
    }
}
