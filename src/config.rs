use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::domain::{Constraints, Command, Argument, Config, ArgumentValue};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeserializedConstraint {
    Text,
    Flag,
    Number,
    Choice(Vec<String>)
}

impl From<DeserializedConstraint> for Constraints {
    fn from(desr: DeserializedConstraint) -> Self {
        match desr {
            DeserializedConstraint::Text => Constraints::Text,
            DeserializedConstraint::Flag => Constraints::Flag,
            DeserializedConstraint::Number => Constraints::Number,
            DeserializedConstraint::Choice(v) => Constraints::Choice(v)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedArgument {
    #[serde(flatten)]
    constraint: DeserializedConstraint,
    short: Option<String>,

    #[serde(default)]
    multi: bool,
}
impl From<(String, DeserializedArgument)> for Argument {
    fn from(desr: (String, DeserializedArgument)) -> Argument {
        let (name, arg) = desr;

        Argument {name, short_hand: arg.short, constraint: arg.constraint.into(), multi: arg.multi }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedCommand {
    args: HashMap<String, DeserializedArgument>,
    run: String
}
impl From<(String, DeserializedCommand)> for Command {
    fn from(desr: (String, DeserializedCommand)) -> Command {
        let (name, cmd) = desr;

        Command { name, args: cmd.args.into_iter().map(Into::into).collect(), run: cmd.run }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeserializedConfig {
    cmd: HashMap<String, DeserializedCommand>
}
impl From<DeserializedConfig> for Config {
    fn from(desr: DeserializedConfig) -> Config {
        Config { command: desr.cmd.into_iter().map(Into::into).collect() }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{Config, Constraints, Argument};

    use super::DeserializedConfig;

    #[test]
    fn accept_correct_yaml_content() {
        let config: Config = serde_yaml::from_str::<DeserializedConfig>(include_str!("../tests/acceptable_config.yaml")).unwrap().into();

        let expected: Vec<Argument> = vec![
            Argument {
                name: "type".to_string(),
                constraint: Constraints::Choice(vec!["core".to_string(), "frontend".to_string(), "types".to_string()]),
                short_hand: Some("t".to_string()),
                multi: true
            },
            Argument {
                name: "snapshot".to_string(),
                constraint: Constraints::Flag,
                short_hand: None,
                multi: false
            }
        ];

        let cmd = config.command.get(0).expect("command length was 0");
        for arg in &cmd.args {
            let expected_arg = expected.iter().find(|e| e.name == arg.name).expect(&format!("Extraneous argument: {}", arg.name));

            assert_eq!(arg, expected_arg);
        }
    }
}
