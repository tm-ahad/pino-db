use std::collections::HashMap;
use crate::err::ErrorType::Syntax;

use crate::err::Error;

pub struct Dsp;

impl Dsp {
    pub fn serialize(map: HashMap<String, String>) -> String {
        let mut res = String::new();

        for (k, v) in map {
            res += &format!("{k}:{v}\n");
        }

        res
    }

    pub fn deserialize(s: String) -> HashMap<String, String> {
        let mut res = HashMap::new();

        for lin in s.lines() {
            let (key, value) = lin.split_once(":")
                .unwrap_or_else(|| {
                    Error::throw(Syntax, "Key and value expected");
                    todo!()
                });

            res.insert(key.to_string(), value.to_string());
        }

        res
    }
}
