use std::num::ParseFloatError;

use crate::values::number::NumberValue;

use super::ValuefulConstraint;

pub struct NumberConstraint;
impl ValuefulConstraint for NumberConstraint {
    type Value = NumberValue;
    type ParseError = ParseFloatError;

    fn parse_value(&self, value: &str) -> Result<Self::Value, Self::ParseError> {
        let value = value.parse()?;

        Ok(NumberValue::new(value))
    }
}
