use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ConstraintValidateError {
    ConstraintUnmet,
    AmbiguosChoice,
    FlagValue
}

#[derive(Debug, PartialEq, Eq)]
pub enum Constraint {
    Text,
    Flag,
    Number,
    Choice(Vec<String>)
}
impl Constraint {
    pub fn convert_value(&self, raw_str: &impl ToString) -> Result<ArgumentValue, ConstraintValidateError> {
        let raw_str = raw_str.to_string();

        match self {
            Constraint::Text => Ok(ArgumentValue::Text(raw_str)),
            Constraint::Flag => Err(ConstraintValidateError::FlagValue),
            Constraint::Number => {
                if let Ok(num) = raw_str.parse::<f64>() {
                    Ok(ArgumentValue::Number(num))
                } else {
                    Err(ConstraintValidateError::ConstraintUnmet)
                }
            },
            Constraint::Choice(candicates) => {
                let matched: Vec<_> = candicates.iter().filter(|c| c.starts_with(&raw_str)).collect();
                if matched.len() != 1 {
                    return Err(ConstraintValidateError::AmbiguosChoice);
                }

                Ok(ArgumentValue::Choice(matched[0].to_owned()))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub short_hand: Option<String>,
    pub constraint: Constraint,
    pub multi: bool
}

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

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub command: Vec<Command>
}

impl Config {
    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.command.iter().find(|c| c.name == name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentValue {
    Text(String),
    Flag(bool),
    Number(f64),
    Choice(String)
}

#[derive(Debug, PartialEq)]
pub struct InputtedCommand {
    pub name: String,
    pub args: HashMap<String, ArgumentValue>
}
