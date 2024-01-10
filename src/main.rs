mod command;
mod tokens;
mod r#struct;
mod err;
mod consts;
mod types;
mod camel;
mod serialize_structs;
mod parse_command;
mod new_db;
mod unwrap_or_none;
mod dsp;
mod open_file;

use command::Command;
use dsp::Dsp;
use r#struct::Struct;
use types::Type;
use std::{io::{self, Write, Read}, env, fs::File, process::exit, collections::{BTreeMap, HashMap}};
use err::{Error, ErrorType};
use new_db::new_db;
use open_file::open_file;
use std::fs::OpenOptions;

use crate::{camel::Camel, parse_command::parse_command, serialize_structs::serialize_structs};

fn set_to_db(db: &mut Database, key: String, value: Type) {
    match value {
        Type::StructT(ref st) => {
            for (prop_name, typ) in &st.prop {
                let key = format!("{}.{}", key, prop_name);
                let val = Type::from_str(&typ.to_string(), None, &Vec::new());
                db.set(key, val)
            }

            db.set(key, value.clone());
        },
        _ => {
            db.set(key, value);
            println!("Value set successfully.");
        }
    }
}

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let aof = argv.contains(&String::from("--aof"));

    match argv.len() {
        1 => {
            println!("Usage: pino <db_directory>");
            exit(0);
        },
        3 if argv[1] == "new" => {
            new_db(argv[2].clone());
            exit(0)
        },
        _ => {}
    }

    
    let dir = &argv[1];

    let db_file_path = format!("{dir}/data.db");
    let struct_file_path = format!("{dir}/struct.db");
    let alias_file_path = format!("{dir}/alias.db");
    let logs_file_path = format!("{dir}/logs");

    let mut db_file = open_file(db_file_path);
    let mut alias_file = open_file(alias_file_path);
    let mut struct_file = open_file(struct_file_path);
    let mut logs_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(logs_file_path.clone())
        .unwrap_or_else(|e| {
            Error::throw(
                ErrorType::OS, 
                &format!("{}: {}", logs_file_path, e)
            );
            todo!()
        });

    let mut struct_map: Vec<Struct> = Vec::new();

    let mut struct_cont = String::new();
    let _ = struct_file.read_to_string(&mut struct_cont);

    let mut alias_cont = String::new();
    let _ = alias_file.read_to_string(&mut alias_cont);

    let mut alias_map = Dsp::deserialize(alias_cont);
    let mut alias_helper = HashMap::<String,String>::new();

    for lin in struct_cont.lines() {
        let st = Struct::deserialize_sign(lin.to_string());
        struct_map.push(st);
    }

    let mut db: Database = Database::new(&mut db_file);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if aof {
            let _ = logs_file.write_all(format!("{}\n", input).as_bytes());
        }

        let command = parse_command(&input, &struct_map);

        match command {
            Command::Get(key) => {
                if key == "*" {
                    println!("{:?}", db.data);
                    continue;
                }

                let mut new_key = alias_map.get(&key).cloned();

                if new_key.is_none() {
                    new_key = alias_helper.get( &key).cloned();
                }

                if new_key.is_none() {
                    for (k, v) in alias_map.clone() {
                        if key.starts_with(&k) {
                            let key_len = k.len();
                            let mut n_key = key.clone();
                            n_key.replace_range(0..key_len, &v);
                            alias_helper.insert(key.clone(), n_key.clone());
                            new_key = Some(n_key);
                            break; // Break after finding a match to avoid further unnecessary iterations
                        }
                    }
                }

                let new_key = new_key.unwrap_or(key.clone());

                if let Some(value) = db.get(&new_key) {
                    match value {
                        Type::StructT(st) => {
                            let mut map = BTreeMap::new();

                            for prop_name in st.prop.keys() {
                                let key = format!("{}.{}", key, prop_name);
                                let v = db.get(&key);

                                if let Some(val) = v {
                                    map.insert(prop_name.to_string(), val);
                                }
                            }

                            println!("{}", Camel::serialize_ref(map)); 
                        },
                        _ => println!("{:?}", value)
                    }
                } else {
                    println!("Key '{}' not found.", key);
                }
            }
            Command::Set(key, value) => set_to_db(&mut db, key, value),
            Command::Remove(key) => {
                if db.delete(&key).is_some() {
                    println!("Deleted key '{}'", key);
                } else {
                    println!("Key '{}' not found.", key);
                }
            },
            Command::Clear => db.clear(),
            Command::SetRange(name, typ, range) => {
                let range_clone = range.clone();

                for i in range_clone {
                    let key = format!("{}.{i}", &name);
                    set_to_db(&mut db, key, typ.clone());
                }

                set_to_db(
                    &mut db, 
                    format!("{name}.len"), 
                    Type::Uint(Some((range.len()+1) as u32))
                );
            },
            Command::Alias(name, to) => {
                alias_map.insert(name, to);
            }
            Command::Exit => {
                let ser = Camel::serialize(db.data);
                let struct_ser = serialize_structs(struct_map);
                let alias_ser = Dsp::serialize(alias_map);

                let _  = db_file.write_all(ser.as_bytes());
                let _ = struct_file.write_all(struct_ser.as_bytes());
                let _ = alias_file.write_all(alias_ser.as_bytes());
                exit(0)
            },
            Command::None => {},
            Command::Struct(st) => struct_map.push(st)
        }
    }
}

struct Database {
    pub data: HashMap<String, Type>,
}

impl Database {
    fn new(file: &mut File) -> Self  {
        let mut cont = String::new();
        let _ = file.read_to_string(&mut cont);

        Database {
            data: Camel::deserialize(cont),
        }
    }

    fn get(&self, key: &str) -> Option<&Type> {
        self.data.get(key)
    }

    fn set(&mut self, key: String, value: Type) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &str) -> Option<Type> {
        self.data.remove(key)
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}
