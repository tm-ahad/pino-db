use std::fs::{File, create_dir};

pub fn new_db(name: String) {
    let db_file_name = format!("{name}/data.db");
    let struct_file_name = format!("{name}/struct.db");
    let alias_file_name = format!("{name}/alias.db");
    let logs_file_name = format!("{name}/logs");

    let _ = create_dir(name);
    let _ = File::create(db_file_name);
    let _ = File::create(struct_file_name);
    let _ = File::create(alias_file_name);
    let _ = File::create(logs_file_name);
}

