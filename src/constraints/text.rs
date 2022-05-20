use crate::values::text::TextValue;

use super::ValuefulConstraint;

pub struct TextConstraint;
impl ValuefulConstraint for TextConstraint {
    type Value = TextValue;
    type ParseError = ();

    fn parse_value(&self, value: &str) -> Result<Self::Value, Self::ParseError> {
        Ok(TextValue::new(value.to_owned()))
    }
}
