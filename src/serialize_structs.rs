use crate::r#struct::Struct;

pub fn serialize_structs(structs: Vec<Struct>) -> String {
    println!("{:?}", structs
        .iter()
        .map(|st| st.serialize_sign())
        .collect::<Vec<String>>());
    structs
        .iter()
        .map(|st| st.serialize_sign())
        .collect::<Vec<String>>()
        .join("\n")
}
