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

#[cfg(test)]
mod tests{
    use rstest::{fixture, rstest};
    use crate::{constraints::{Constraint, ValueParseError}, domain::ArgumentValue, helper::identify::Identify};

    use super::{NumberConstraint, NumberParseError};

    #[rstest(input, expected,
        case("123", 123f64),
        case("123456789.87654321", 123456789.87654321f64),
        case("-500", -500f64),
        case("+123", 123f64),
    )]
    fn accepts_numeric_value(input: &str, expected: f64) {
        let parsed = NumberConstraint.parse_value(Some(input));

        let parsed = parsed.expect("Should success, but failed");
        assert_eq!(parsed, ArgumentValue::Number(expected))
    }

    #[rstest(input,
        case("Not numeric"),
        case("0xA"),
        case("A")
    )]
    fn declines_non_numeric_value(input: &str) {
        let parsed = NumberConstraint.parse_value(Some(input));

        let error = parsed.expect_err("Should fail, but succeeded");
        let error = match error {
            ValueParseError::ParseFailed(f) => f,
            _ => panic!("Unexpected error yielded: {:#?}", error)
        };
        assert_eq!(
            error.get_identifier(),
            NumberParseError::NumberParseFailure(input.to_string()).get_identifier()
        )
    }

    #[rstest]
    fn fail_fallback() {
        let parsed = NumberConstraint.fallback();

        let error = parsed.expect_err("Should fail, but succeeded");
        assert_eq!(error, ValueParseError::ValueRequired)
    }
}
