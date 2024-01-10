
pub fn unwrap_or_none_as_string<T: std::fmt::Debug>(op: &Option<T>) -> String {
    match op {
        Some(v) => format!("{:?}", v),
        None => String::from("None"),
    }
}
