use crate::values::text::TextValue;

use super::ValuefulConstraint;

pub enum ChoiceError {
    Ambiguous,
    NotIncluded
}

pub struct ChoiceConstraint {
    choices: Vec<String>
}
impl ValuefulConstraint for ChoiceConstraint {
    type Value = TextValue;
    type ParseError = ChoiceError;

    fn parse_value(&self, value: &str) -> Result<Self::Value, Self::ParseError> {
        let matched: Vec<_> = self.choices.iter().filter(|c| c.starts_with(&value)).collect();
        if matched.is_empty() {
            return Err(ChoiceError::NotIncluded);
        }
        if matched.len() > 1 {
            return Err(ChoiceError::Ambiguous);
        }

        Ok(TextValue::new(matched[0].to_owned()))
    }
}
