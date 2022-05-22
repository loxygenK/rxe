use crate::domain::ArgumentValue;

use super::{Constraint, ValueParseError};

pub struct FlagConstraint;
impl Constraint for FlagConstraint {
    fn parse_value(&self, value: Option<&str>) -> Result<ArgumentValue, ValueParseError> {
        match value {
            Some(_) => Err(ValueParseError::ValueUneccesary),
            None => Ok(ArgumentValue::Flag(true))
        }
    }

    fn fallback(&self) -> Result<ArgumentValue, ValueParseError> {
        Ok(ArgumentValue::Flag(false))
    }
}
