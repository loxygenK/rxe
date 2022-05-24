use std::{fmt::Display, collections::HashMap};

use crate::{domain::ArgumentValue, helper::identify::{IdBox, Identify}, placeholder::PlaceholderParseError};

use super::{ValuefulConstraint, SpecificParseError};

#[derive(Debug)]
pub enum ChoiceError {
    Ambiguous(String),
    NotIncluded(String)
}
impl Identify for ChoiceError {
    fn get_identifier(&self) -> String {
        match self {
            ChoiceError::Ambiguous(_) => "ChoiceError::Ambiguous".to_string(),
            ChoiceError::NotIncluded(_) => "ChoiceError::NotIncluded".to_string()
        }
    }
}
impl Display for ChoiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChoiceError::Ambiguous(cmd) => write!(f, "'{}' is too ambiguous. Type the choice longer", cmd),
            ChoiceError::NotIncluded(cmd) => write!(f, "'{}' is not available as the choice.", cmd),
        }
    }
}
impl SpecificParseError for ChoiceError {}

pub struct ChoiceConstraint {
    choices: Vec<String>
}
impl ValuefulConstraint for ChoiceConstraint {
    fn parse_value(&self, value: &str) -> Result<ArgumentValue, IdBox<dyn SpecificParseError>> {
        let matched: Vec<_> = self.choices.iter().filter(|c| c.starts_with(&value)).collect();
        if matched.is_empty() {
            return Err(IdBox::new(Box::new(ChoiceError::NotIncluded(value.to_owned()))));
        }
        if matched.len() > 1 {
            return Err(IdBox::new(Box::new(ChoiceError::Ambiguous(value.to_owned()))));
        }

        Ok(ArgumentValue::Text(matched[0].to_owned()))
    }

    fn fill_placeholder(&self, value: &ArgumentValue, _placeholder_args: &HashMap<String, String>) -> Result<String, PlaceholderParseError> {
        match value {
            ArgumentValue::Text(t) => Ok(t.to_string()),
            _ => panic!("Unexpected ArgumentValue: {:#?}", value)
        }
    }
}
impl ChoiceConstraint {
    pub fn new(choices: Vec<String>) -> Self {
        Self { choices }
    }
}

#[cfg(test)]
mod tests{
    use rstest::{fixture, rstest};
    use crate::{constraints::{Constraint, ValueParseError}, domain::ArgumentValue, helper::identify::Identify};

    use super::{ChoiceConstraint, ChoiceError};

    #[fixture]
    pub fn constraint() -> ChoiceConstraint {
        ChoiceConstraint::new(vec![
            "ChoiceAAA".to_string(),
            "ChoiceBBB".to_string(),
            "DDD".to_string(),
        ])
    }

    #[rstest(choice, expected,
        case("ChoiceAAA", "ChoiceAAA"),
        case("ChoiceA", "ChoiceAAA"),
        case("ChoiceBB", "ChoiceBBB"),
        case("D", "DDD"),
    )]
    fn accepts_and_strictify_choice(constraint: ChoiceConstraint, choice: &str, expected: &str) {
        let parsed = constraint.parse_value(Some(choice));

        let parsed = parsed.expect("Should success, but failed");
        assert_eq!(parsed, ArgumentValue::Text(expected.to_string()))
    }

    #[rstest(choice, expected,
        case("H", ChoiceError::NotIncluded("H".to_string())),
        case("", ChoiceError::Ambiguous("".to_string())),
        case("Choice", ChoiceError::Ambiguous("".to_string())),
    )]
    fn declines_not_choicable_value(constraint: ChoiceConstraint, choice: &str, expected: ChoiceError) {
        let parsed = constraint.parse_value(Some(choice));

        let error = parsed.expect_err("Should fail, but succeeded");
        let error = match error {
            ValueParseError::ParseFailed(f) => f,
            _ => panic!("Unexpected error yielded: {:#?}", error)
        };
        assert_eq!(error.get_identifier(), expected.get_identifier())
    }

    #[rstest]
    fn fail_fallback(constraint: ChoiceConstraint) {
        let parsed = constraint.fallback();

        let error = parsed.expect_err("Should fail, but succeeded");
        assert_eq!(error, ValueParseError::ValueRequired)
    }
}
