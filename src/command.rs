use std::ops::Range;

use crate::types::Type;
use crate::r#struct::Struct;

pub enum Command {
    Get(String),
    Set(String, Type),
    Remove(String),
    Struct(Struct),
    SetRange(String, Type, Range<u32>),
    Alias(String, String),
    Clear,
    None,
    Exit,
}
