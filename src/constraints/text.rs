use crate::{domain::ArgumentValue, helper::identify::IdBox};

use super::{ValuefulConstraint, SpecificParseError};

pub struct TextConstraint;
impl ValuefulConstraint for TextConstraint {
    fn parse_value(&self, value: &str) -> Result<ArgumentValue, IdBox<dyn SpecificParseError>> {
        Ok(ArgumentValue::Text(value.to_owned()))
    }
}

#[cfg(test)]
mod tests{
    use crate::{constraints::{Constraint, ValueParseError}, domain::ArgumentValue, helper::identify::Identify};

    use super::TextConstraint;

    #[test]
    fn accpet_any_value() {
        let parsed = TextConstraint.parse_value(Some("text"));

        let parsed = parsed.expect("Should success, but failed");
        assert_eq!(parsed, ArgumentValue::Text("text".to_string()))
    }

    #[test]
    fn fail_fallback() {
        let parsed = TextConstraint.fallback();

        let error = parsed.expect_err("Should fail, but succeeded");
        assert_eq!(error, ValueParseError::ValueRequired)
    }
}
