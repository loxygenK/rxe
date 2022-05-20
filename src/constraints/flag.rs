use crate::values::flag::FlagValue;

use super::{Constraint, ValueParseError};

pub struct FlagConstraint;
impl Constraint for FlagConstraint {
    type Value = FlagValue;
    type ParseError = ();

    fn parse_value(&self, value: Option<&str>) -> Result<Self::Value, ValueParseError<Self::ParseError>> {
        match value {
            Some(_) => Err(ValueParseError::ValueUneccesary),
            None => Ok(FlagValue::new(true))
        }
    }
}
