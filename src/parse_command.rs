use crate::{Command, ErrorType, Error, Type, Struct};
use crate::tokens::{GET, SET, REMOVE, STRUCT, EXIT, SET_RANGE, ALIAS};
use crate::err::ErrorType::{Typerr, Syntax};

pub fn parse_command(input: &str, struct_map: &[Struct]) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        [GET, key] => Command::Get(key.to_string()),
        [SET, key, value] => {
            let value = value.split_once(':')
                .unwrap_or_else(|| {
                    Error::throw(ErrorType::Syntax, "Type and value expected");
                    todo!()
                });

            let val = match value.1 {
                "None" => None,
                _ => Some(value.1)
            };

            Command::Set(key.to_string(), Type::from_str(value.0, val, struct_map))
        },
        [REMOVE, key] => Command::Remove(key.to_string()),
        [STRUCT, name, sign] => {
            let sign = sign.replace(',', ":");

            let st = Struct::deserialize_sign(format!("{name}:{}", sign));
            Command::Struct(st)
        },
        [SET_RANGE, name, type_and_range] => {
            let (typ, raw_range) = type_and_range.split_once(':')
                .unwrap_or_else(|| {
                    Error::throw(Syntax, "Type and range expected");
                    todo!()
                });

            let (start, end) = raw_range.split_once("..")
                .unwrap_or_else(|| {
                    Error::throw(Syntax, "Type and range expected");
                    todo!()
                });

            let start = start.parse::<u32>().unwrap_or_else(|_| {
                Error::throw(Typerr, &format!("Invalid integer {start}"));
                todo!()
            });

            let end = end.parse::<u32>().unwrap_or_else(|_| {
                Error::throw(Typerr, &format!("Invalid integer {end}"));
                todo!()
            });
            
            let range = start..end;

            Command::SetRange(
                name.to_string(),
                Type::from_str(typ, None, struct_map), 
                range
            )
        },
        [ALIAS, name, to] => Command::Alias(
            name.to_string(), 
            to.to_string()
        ),
        [EXIT] => Command::Exit,
        _ => {
            eprintln!("Invalid command");
            Command::None
        }
    }
}
