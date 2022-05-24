use std::{fmt::{Debug, Display}, collections::HashMap, ops::Deref};

use crate::{helper::identify::{Identify, IdBox}, domain::ArgumentValue, placeholder::PlaceholderParseError};

pub mod choice;
pub mod number;
pub mod text;
pub mod flag;

pub trait SpecificParseError: Debug + Display + Identify {}

#[derive(Debug, PartialEq)]
pub enum ValueParseError {
    ValueRequired,
    ValueUneccesary,
    ParseFailed(IdBox<dyn SpecificParseError>)
}
impl Display for ValueParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueParseError::ValueRequired => write!(f, "The value is required for the argument."),
            ValueParseError::ValueUneccesary => write!(f, "The value of the argument should not be given."),
            ValueParseError::ParseFailed(e) => {
                write!(f, "The value was not appropriate: {}", e.deref())
            }
        }
    }
}

pub trait Constraint {
    fn parse_value(&self, value: Option<&str>) -> Result<ArgumentValue, ValueParseError>;
    fn fallback(&self) -> Result<ArgumentValue, ValueParseError>;
    fn fill_placeholder(&self, value: &ArgumentValue, placeholder_args: &HashMap<String, String>) -> Result<String, PlaceholderParseError>;
}

pub trait ValuefulConstraint {
    fn parse_value(&self, value: &str) -> Result<ArgumentValue, IdBox<dyn SpecificParseError>>;
    fn fill_placeholder(&self, value: &ArgumentValue, placeholder_args: &HashMap<String, String>) -> Result<String, PlaceholderParseError>;
}
impl<T: ValuefulConstraint> Constraint for T {
    fn parse_value(&self, value: Option<&str>) -> Result<ArgumentValue, ValueParseError> {
        let value = value.ok_or(ValueParseError::ValueRequired)?;

        self.parse_value(value).map_err(ValueParseError::ParseFailed)
    }

    fn fallback(&self) -> Result<ArgumentValue, ValueParseError> {
        Err(ValueParseError::ValueRequired)
    }

    fn fill_placeholder(&self, value: &ArgumentValue, placeholder_args: &HashMap<String, String>) -> Result<String, PlaceholderParseError> {
        self.fill_placeholder(value, placeholder_args)
    }


}
