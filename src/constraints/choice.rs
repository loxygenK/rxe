use std::fmt::Display;

use crate::{domain::ArgumentValue, helper::{IdBox, Identify}};

use super::{ValuefulConstraint, SpecificParseError};

#[derive(Debug)]
pub enum ChoiceError {
    Ambiguous(String),
    NotIncluded(String)
}
impl Identify for ChoiceError {
    fn get_identifier(&self) -> String {
        match self {
            ChoiceError::Ambiguous(_) => "ChoiceError::Ambiguous".to_string(),
            ChoiceError::NotIncluded(_) => "ChoiceError::NotIncluded".to_string()
        }
    }
}
impl Display for ChoiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChoiceError::Ambiguous(cmd) => write!(f, "'{}' is too ambiguous. Type the choice longer", cmd),
            ChoiceError::NotIncluded(cmd) => write!(f, "'{}' is not available as the choice.", cmd),
        }
    }
}
impl SpecificParseError for ChoiceError {}

pub struct ChoiceConstraint {
    choices: Vec<String>
}
impl ValuefulConstraint for ChoiceConstraint {
    fn parse_value(&self, value: &str) -> Result<ArgumentValue, IdBox<dyn SpecificParseError>> {
        let matched: Vec<_> = self.choices.iter().filter(|c| c.starts_with(&value)).collect();
        if matched.is_empty() {
            return Err(IdBox::new(Box::new(ChoiceError::NotIncluded(value.to_owned()))));
        }
        if matched.len() > 1 {
            return Err(IdBox::new(Box::new(ChoiceError::Ambiguous(value.to_owned()))));
        }

        Ok(ArgumentValue::Text(matched[0].to_owned()))
    }
}
impl ChoiceConstraint {
    pub fn new(choices: Vec<String>) -> Self {
        Self { choices }
    }
}
