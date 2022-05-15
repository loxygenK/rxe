use std::collections::HashMap;

pub enum ConstraintValidateError {
    ConstriantUnmet,
    AmbiguosChoice
}

#[derive(Debug, PartialEq, Eq)]
pub enum Constraint {
    Text,
    Flag,
    Number,
    Choice(Vec<String>)
}
impl Constraint {
    pub fn valid(&self, raw_str: &impl ToString) -> Result<(), ConstraintValidateError> {
        let raw_str = raw_str.to_string();

        match self {
            Constraint::Text => Ok(()),
            Constraint::Flag => Ok(()),
            Constraint::Number => {
                if raw_str.parse::<f64>().is_ok() {
                    Ok(())
                } else {
                    Err(ConstraintValidateError::ConstriantUnmet)
                }
            },
            Constraint::Choice(candicates) => {
                let matched: Vec<_> = candicates.iter().filter(|c| c.starts_with(&raw_str)).collect();
                if matched.len() != 1 {
                    return Err(ConstraintValidateError::AmbiguosChoice);
                }

                Ok(())
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

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub command: Vec<Command>
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
