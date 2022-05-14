use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::domain::{Constraint, Command, Argument, Config};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeserializedConstraint {
    Text,
    Number,
    Choice(Vec<String>)
}
impl Into<Constraint> for DeserializedConstraint {
    fn into(self) -> Constraint {
        match self {
            DeserializedConstraint::Text => Constraint::Text,
            DeserializedConstraint::Number => Constraint::Number,
            DeserializedConstraint::Choice(v) => Constraint::Choice(v.clone())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedArgument {
    constriant: DeserializedConstraint,
    multi: bool
}
impl Into<Argument> for (String, DeserializedArgument) {
    fn into(self) -> Argument {
        let (name, arg) = self;

        Argument::new(name, arg.constriant.into(), arg.multi)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedCommand {
    args: HashMap<String, DeserializedArgument>,
    run: String
}
impl Into<Command> for (String, DeserializedCommand) {
    fn into(self) -> Command {
        let (name, cmd) = self;

        Command::new(name, cmd.args.into_iter().map(Into::into).collect(), cmd.run)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedConfig {
    command: HashMap<String, DeserializedCommand>
}
impl Into<Config> for DeserializedConfig {
    fn into(self) -> Config {
        Config::new(self.command.into_iter().map(Into::into).collect())
    }
}
