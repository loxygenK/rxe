pub mod choice;
pub mod number;
pub mod text;
pub mod flag;

pub enum ValueParseError<T> {
    ValueRequired,
    ValueUneccesary,
    ParseFailed(T)
}

pub trait Constraint {
    type Value;
    type ParseError;

    fn parse_value(&self, value: Option<&str>) -> Result<Self::Value, ValueParseError<Self::ParseError>>;
}

pub trait ValuefulConstraint {
    type Value;
    type ParseError;

    fn parse_value(&self, value: &str) -> Result<Self::Value, Self::ParseError>;
}
impl<T: ValuefulConstraint> Constraint for T {
    type Value = T::Value;
    type ParseError = T::ParseError;

    fn parse_value(&self, value: Option<&str>) -> Result<Self::Value, ValueParseError<Self::ParseError>> {
        let value = value.ok_or(ValueParseError::ValueRequired)?;

        self.parse_value(value).map_err(ValueParseError::ParseFailed)
    }
}
