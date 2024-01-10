use crate::err::Error;
use crate::ErrorType::{Syntax, Typerr};
use crate::r#struct::Struct;

#[derive(Debug)]
#[derive(Clone)]
pub enum Type {
    String(Option<String>),
    Bool(Option<bool>),
    Int(Option<i32>),
    Uint(Option<u32>),
    StructT(Struct),
    Null,
}

impl Type {
    pub fn from_str(_type: &str, val: Option<&str>, structs: &[Struct]) -> Type {
        if val.is_none() {
            return match _type {
                "string" => Type::String(None),
                "int" => Type::Int(None),
                "bool" => Type::Bool(None),
                "uint" => Type::Uint(None),
                _ => {
                    let sts = structs
                        .iter()
                        .filter(|st| st.name == _type)
                        .collect::<Vec<&Struct>>();

                    return if sts.is_empty() {
                        Error::throw(Typerr, &format!("Invalid type {}", _type));
                        todo!()
                    } else {
                        let st = sts[0];

                        Type::StructT(st.clone())
                    }
                }
            }
        } else {
            let val = val.unwrap();

            match _type {
                "string" => Type::String(Some(val.to_string())),
                "bool" => Type::Bool(Some(match val {
                    "y" => true,
                    "n" => false,
                    _ => {
                        Error::throw(Typerr, &format!("Invalid bool {}", val));
                        false
                    }
                })),
                "int" => Type::Int(Some(val.parse::<i32>().unwrap_or_else(|_| {
                    Error::throw(Syntax, &format!("Invalid integer {val}"));
                    todo!()
                }))),
                "uint" => Type::Int(Some(val.parse::<i32>().unwrap_or_else(|_| {
                    Error::throw(Syntax, &format!("Invalid integer {val}"));
                    todo!()
                }))),
                _ => {
                    let sts = structs
                        .iter()
                        .filter(|st| st.name == _type)
                        .collect::<Vec<&Struct>>();

                    if sts.is_empty() {
                        Error::throw(Typerr, &format!("Invalid type {}", _type));
                        todo!()
                    } else {
                        let st = sts[0];

                        Type::StructT(st.clone())
                    }
                }
            }
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match &self {
            Type::String(_) => "string",
            Type::Bool(_) => "bool",
            Type::Int(_) => "int",
            Type::Uint(_) => "uint",
            Type::StructT(_) => "struct",
            Type::Null => "None",
        }.to_string()
    }
}
