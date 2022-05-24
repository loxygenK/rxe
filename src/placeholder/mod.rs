mod parse;
mod fill;

use std::{collections::HashMap, ops::Range};

#[derive(Debug, PartialEq, Eq)]
pub enum PlaceholderParseError {
    NotExistingArgument,
    MalformedParameter,
    InsufficientParameter(String),
    CorruptedDuringFill
}

#[derive(Debug, PartialEq, Eq)]
pub struct Placeholder {
    range: Range<usize>,
    arg_name: String,
    prefix: String,
    args: HashMap<String, String>,
}
