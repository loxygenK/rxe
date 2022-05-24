pub(crate) mod exec;

use std::env;

pub struct Environment {
    pub config_file: Option<String>,
    pub args: Vec<String>
}
