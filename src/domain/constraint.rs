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
    pub fn convert_value(&self, raw_str: impl ToString) -> Result<ArgumentValue, ConstraintValidateError> {
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

#[cfg(test)]
mod tests {
    use crate::domain::ArgumentValue;
    use super::{Constraint, ConstraintValidateError};

    use rstest::rstest;

    type ConvertResult<T> = Result<T, ConstraintValidateError>;

    #[rstest(input, expected,
        case("123", Ok("123")),
        case("", Ok("")),
        case("Whatever", Ok("Whatever")),
    )]
    fn accept_any_string_for_constraint_text(input: &str, expected: ConvertResult<&str>) {
        assert_eq!(
            Constraint::Text.convert_value(input),
            expected.map(|e| ArgumentValue::Text(e.to_string()))
        );
    }

    #[rstest(input, expected,
        case("123", Err(ConstraintValidateError::FlagValue)),
        case("", Err(ConstraintValidateError::FlagValue)),
        case("Whatever", Err(ConstraintValidateError::FlagValue)),
    )]
    fn accept_no_string_for_constraint_flag(input: &str, expected: ConvertResult<&str>) {
        assert_eq!(
            Constraint::Flag.convert_value(input),
            expected.map(|e| ArgumentValue::Flag(true))
        );
    }

    #[rstest(input, expected,
        case("123", Ok(123f64)),
        case("", Err(ConstraintValidateError::ConstraintUnmet)),
        case("Whatever", Err(ConstraintValidateError::ConstraintUnmet)),
    )]
    fn accept_numeric_string_for_constraint_flag(input: &str, expected: ConvertResult<f64>) {
        assert_eq!(
            Constraint::Number.convert_value(input),
            expected.map(ArgumentValue::Number)
        );
    }

    #[rstest(input, expected,
        case("ChoiceAAA", Ok("ChoiceAAA")),
        case("ChoiceBBB", Ok("ChoiceBBB")),
        case("ChoiceCCC", Ok("ChoiceCCC")),
        case("Choice", Err(ConstraintValidateError::AmbiguosChoice)),
        case("", Err(ConstraintValidateError::AmbiguosChoice)),
        case("Whatever", Err(ConstraintValidateError::AmbiguosChoice)),
    )]
    fn accept_string_in_candicate_for_constraint_choice(input: &str, expected: ConvertResult<&str>) {
        let candicates = vec!["ChoiceAAA", "ChoiceBBB", "ChoiceCCC"].iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(
            Constraint::Choice(candicates).convert_value(input),
            expected.map(|e| ArgumentValue::Choice(e.to_string()))
        );
    }

    #[rstest(input, expected,
        case("ChoiceA", Ok("ChoiceAAA")),
        case("ChoiceB", Ok("ChoiceBBB")),
        case("ChoiceC", Ok("ChoiceCCC")),
    )]
    fn accept_ambiguos_for_constraint_choice(input: &str, expected: ConvertResult<&str>) {
        let candicates = vec!["ChoiceAAA", "ChoiceBBB", "ChoiceCCC"].iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(
            Constraint::Choice(candicates).convert_value(input),
            expected.map(|e| ArgumentValue::Choice(e.to_string()))
        );
    }
}
