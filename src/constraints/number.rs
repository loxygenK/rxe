use std::fmt::Display;

use crate::{domain::ArgumentValue, helper::identify::{Identify, IdBox}};

use super::{ValuefulConstraint, SpecificParseError};

#[derive(Debug)]
pub enum NumberParseError {
    NumberParseFailure(String)
}
impl Identify for NumberParseError {
    fn get_identifier(&self) -> String {
        "NumberParseError::NumberParseFailure".to_string()
    }
}
impl Display for NumberParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let NumberParseError::NumberParseFailure(orig) = self;
        write!(f, "'{}' could not be parsed as the number (esp. f64)", orig)
    }
}
impl SpecificParseError for NumberParseError {}

pub struct NumberConstraint;
impl ValuefulConstraint for NumberConstraint {
    fn parse_value(&self, value: &str) -> Result<ArgumentValue, IdBox<dyn SpecificParseError>> {
        match value.parse::<f64>() {
            Ok(v) => Ok(ArgumentValue::Number(v)),
            Err(_) => Err(IdBox::new(Box::new(NumberParseError::NumberParseFailure(value.to_owned()))))
        }
    }
}
