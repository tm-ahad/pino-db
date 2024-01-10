use crate::Type;
use crate::tokens::{SOME, NONE};
use crate::unwrap_or_none::unwrap_or_none_as_string;
use std::collections::{BTreeMap, HashMap};

pub struct Camel;

impl Camel {
    pub fn serialize(map: HashMap<String, Type>) -> String {
        let mut res: String = String::new();

        for (k, v) in map {
            let v = match v {
                Type::Bool(v) => match v.unwrap() {
                    true => String::from("Some(y)"),
                    false => String::from("Some(n)"),
                },
                Type::Int(i) => unwrap_or_none_as_string(&i),
                Type::String(s) => unwrap_or_none_as_string(&s),
                Type::Null => NONE.to_string(),
                _ => String::new(),
            };

            res += &format!(
                "{k}:{v}\n",
            );
        }

        res
    }

    pub fn serialize_ref(map: BTreeMap<String, &Type>) -> String {
        let mut res: String = String::new();

        for (k, v) in map {
            let v = match v {
                Type::Bool(v) => match v.unwrap() {
                    true => String::from("y"),
                    false => String::from("n"),
                },
                Type::Int(i) => unwrap_or_none_as_string(i),
                Type::String(s) => unwrap_or_none_as_string(s),
                Type::Null => NONE.to_string(),
                _ => String::new(),
            };

            res += &format!(
                "{k}:{v}\n",
            );
        }

        res
    }

    pub fn deserialize(s: String) -> HashMap<String, Type> {
        let mut map = HashMap::<String, Type>::new();

        return if s.is_empty() {
            map
        } else {
            let lines = s.lines();

            for line in lines {
                let val = line.split_once(':').unwrap();

                if val.1.starts_with(SOME) {
                    println!("{}", line);

                    map.insert(
                        val.0.to_string(),
                        match val.1 {
                            "y" => Type::Bool(Some(true)),
                            "n" => Type::Bool(Some(false)),
                            v if v.starts_with('"') && v.ends_with('"') => Type::String(Some(v.to_string())),
                            NONE => Type::Null,
                            v => Type::Int(Some(v.parse::<i32>().unwrap())) ,
                        }
                    );
                } else {
                    map.insert(val.0.to_string(), Type::Null);
                }

                map.insert(
                    val.0.to_string(),
                    match val.1 {
                        "y" => Type::Bool(Some(true)),
                        "n" => Type::Bool(Some(false)),
                        NONE => Type::Null,
                        v if v.starts_with('"') && v.ends_with('"') => Type::String(Some(v.to_string())),
                        "" => continue,
                        v => Type::Int(Some(v.parse::<i32>().unwrap())) 
                    },
                );
            }

            map
        }
    }
}
