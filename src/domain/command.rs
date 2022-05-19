use super::argument::Argument;

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    pub name: String,
    pub args: Vec<Argument>,
    pub run: String
}
impl Command {
    pub fn get_argument(&self, name: &str) -> Option<&Argument> {
        self.args.iter().find(|c| c.name == name)
    }
}
