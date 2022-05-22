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


#[cfg(test)]
mod tests{
    use crate::{constraints::{Constraint, ValueParseError}, domain::ArgumentValue, helper::identify::Identify};

    use super::FlagConstraint;

    #[test]
    fn accept_and_raise_flag_when_value_is_not_supplied() {
        let parsed = FlagConstraint.parse_value(None);

        let parsed = parsed.expect("Should success, but failed");
        assert_eq!(parsed, ArgumentValue::Flag(true))
    }

    #[test]
    fn decline_any_value() {
        let parsed = FlagConstraint.parse_value(Some("thing"));

        let error = parsed.expect_err("Should fail, but succeeded");
        assert_eq!(error, ValueParseError::ValueUneccesary)
    }

    #[test]
    fn return_false_value_when_argument_is_not_specified() {
        let parsed = FlagConstraint.fallback();

        let parsed = parsed.expect("Should success, but failed");
        assert_eq!(parsed, ArgumentValue::Flag(false));
    }
}
