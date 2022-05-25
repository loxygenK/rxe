pub(crate) mod exec;

pub struct Environment {
    pub config_file: Option<String>,
    pub args: Vec<String>
}
