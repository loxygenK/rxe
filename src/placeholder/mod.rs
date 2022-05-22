mod parse;

use std::{marker::PhantomData, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub enum PlaceholderParseError {
    MalformedParameter
}

#[derive(Debug, PartialEq, Eq)]
pub struct Placeholder {
    placeholder: String,
    arg_name: String,
    args: HashMap<String, String>,
}
