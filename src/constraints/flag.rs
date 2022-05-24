use std::collections::HashMap;

use crate::{domain::ArgumentValue, placeholder::PlaceholderParseError};

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

    fn fill_placeholder(&self, value: &ArgumentValue, placeholder_args: &HashMap<String, String>) -> Result<String , PlaceholderParseError>{
        let if_true = placeholder_args.get("true");
        let if_false = placeholder_args.get("false");

        if if_true.is_none() && if_false.is_none() {
            return Err(PlaceholderParseError::InsufficientParameter("either left or right".to_string()))
        }

        let flag = match value {
            ArgumentValue::Flag(f) => f,
            _ => panic!("Unexpected ArgumentValue: {:#?}", value)
        };

        Ok(
            if *flag {
                if_true.unwrap_or(&"".to_string()).to_string()
            } else {
                if_false.unwrap_or(&"".to_string()).to_string()
            }
        )
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
