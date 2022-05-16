use crate::domain::ArgumentValue;

#[derive(Debug, PartialEq, Eq)]
pub enum ConstraintValidateError {
    ConstraintUnmet,
    AmbiguosChoice,
    FlagValue
}

#[derive(Debug, PartialEq, Eq)]
pub enum Constraint {
    Text,
    Flag,
    Number,
    Choice(Vec<String>)
}
impl Constraint {
    pub fn convert_value(&self, raw_str: &impl ToString) -> Result<ArgumentValue, ConstraintValidateError> {
        let raw_str = raw_str.to_string();

        match self {
            Constraint::Text => Ok(ArgumentValue::Text(raw_str)),
            Constraint::Flag => Err(ConstraintValidateError::FlagValue),
            Constraint::Number => {
                if let Ok(num) = raw_str.parse::<f64>() {
                    Ok(ArgumentValue::Number(num))
                } else {
                    Err(ConstraintValidateError::ConstraintUnmet)
                }
            },
            Constraint::Choice(candicates) => {
                let matched: Vec<_> = candicates.iter().filter(|c| c.starts_with(&raw_str)).collect();
                if matched.len() != 1 {
                    return Err(ConstraintValidateError::AmbiguosChoice);
                }

                Ok(ArgumentValue::Choice(matched[0].to_owned()))
            }
        }
    }
}
