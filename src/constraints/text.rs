use crate::{domain::ArgumentValue, helper::IdBox};

use super::{ValuefulConstraint, SpecificParseError};

pub struct TextConstraint;
impl ValuefulConstraint for TextConstraint {
    fn parse_value(&self, value: &str) -> Result<ArgumentValue, IdBox<dyn SpecificParseError>> {
        Ok(ArgumentValue::Text(value.to_owned()))
    }
}
