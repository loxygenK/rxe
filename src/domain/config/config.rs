use super::command::Command;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub command: Vec<Command>
}

impl Config {
    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.command.iter().find(|c| c.name == name)
    }
}
