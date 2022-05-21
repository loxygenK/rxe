use std::fmt::{Debug, Display};

use crate::{helper::{Identify, IdBox}, domain::ArgumentValue};

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

pub trait Constraint {
    fn parse_value(&self, value: Option<&str>) -> Result<ArgumentValue, ValueParseError>;
}

pub trait ValuefulConstraint {
    fn parse_value(&self, value: &str) -> Result<ArgumentValue, IdBox<dyn SpecificParseError>>;
}
impl<T: ValuefulConstraint> Constraint for T {
    fn parse_value(&self, value: Option<&str>) -> Result<ArgumentValue, ValueParseError> {
        let value = value.ok_or(ValueParseError::ValueRequired)?;

        self.parse_value(value).map_err(ValueParseError::ParseFailed)
    }
}
