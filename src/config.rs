use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::domain::{Constraint, Command, Argument, Config};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeserializedConstraint {
    Text,
    Flag,
    Number,
    Choice(Vec<String>)
}
impl Into<Constraint> for DeserializedConstraint {
    fn into(self) -> Constraint {
        match self {
            DeserializedConstraint::Text => Constraint::Text,
            DeserializedConstraint::Flag => Constraint::Flag,
            DeserializedConstraint::Number => Constraint::Number,
            DeserializedConstraint::Choice(v) => Constraint::Choice(v.clone())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedArgument {
    #[serde(flatten)]
    constraint: DeserializedConstraint,
    short: Option<String>,

    #[serde(default)]
    multi: bool
}
impl Into<Argument> for (String, DeserializedArgument) {
    fn into(self) -> Argument {
        let (name, arg) = self;

        Argument {name, short_hand: arg.short, constraint: arg.constraint.into(), multi: arg.multi }
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

        Command { name, args: cmd.args.into_iter().map(Into::into).collect(), run: cmd.run }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedConfig {
    cmd: HashMap<String, DeserializedCommand>
}
impl Into<Config> for DeserializedConfig {
    fn into(self) -> Config {
        Config { command: self.cmd.into_iter().map(Into::into).collect() }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{Config, Command, Constraint, Argument};

    use super::DeserializedConfig;

    #[test]
    fn accept_correct_yaml_content() {
        let config: Config = serde_yaml::from_str::<DeserializedConfig>(include_str!("../tests/acceptable_config.yaml")).unwrap().into();

        assert_eq!(
            config,
            Config {
                command: vec![
                    Command {
                        name: "test".to_string(),
                        args: vec![
                            Argument {
                                name: "type".to_string(),
                                constraint: Constraint::Choice(vec!["core".to_string(), "frontend".to_string(), "types".to_string()]),
                                short_hand: Some("t".to_string()),
                                multi: true
                            },
                            Argument {
                                name: "snapshot".to_string(),
                                constraint: Constraint::Flag,
                                short_hand: None,
                                multi: false
                            }
                        ],
                        run: "pnpm --filter=${type} test ${snapshot?=-u}".to_string()
                    }
                ]
            }
        )
    }
}
