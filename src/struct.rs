use std::collections::BTreeMap;

use crate::types::Type;

#[derive(Debug)]
#[derive(Clone)]
pub struct Struct {
    pub name: String,
    pub prop: BTreeMap<String, Type>
}

impl Struct {
    pub fn new() -> Self {
        Struct {
            name: String::new(),
            prop: BTreeMap::new()
        }
    }

    pub fn serialize_sign(&self) -> String {
        let mut res = format!("{}:", self.name);

        for (prop_name, typ) in &self.prop {
            res += &format!("{prop_name}:{}:", typ.to_string());
        }

        res
    }

    pub fn deserialize_sign(s: String) -> Self {
        let split = s.split(':').collect::<Vec<&str>>();
        let mut st = Struct::new();
        st.name = split[0].to_string();

        let mut idx = 1;
        let len = split.len();

        while idx < len-1  {
            if idx == len-1 {
                continue;
            }

            let prop_name = split[idx];
            let typ = split[idx+1];

            if typ.is_empty() {
                continue;
            }

            println!("{}: {}", prop_name, typ);

            st.prop.insert(prop_name.to_string(), Type::from_str(typ, None, &Vec::new()));
            idx += 2;
        }

        st
    }
}
