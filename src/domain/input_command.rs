use std::collections::HashMap;

use super::argument_value::ArgumentValue;

#[derive(Debug, PartialEq)]
pub struct InputtedCommand {
    pub name: String,
    pub args: HashMap<String, ArgumentValue>
}
