use std::collections::HashMap;
use std::ops::Range;

use crate::{domain::ArgumentValue, constraints::{text::TextConstraint, Constraint, number::NumberConstraint, flag::FlagConstraint, choice::ChoiceConstraint}};

use super::{Placeholder, PlaceholderParseError};

pub(super) fn fill_first_placeholder(original: &str, values: &HashMap<String, ArgumentValue>, placeholder: &Placeholder) -> Result<(String, Range<usize>), PlaceholderParseError> {
    let original = original.to_owned();
    let value = values.get(&placeholder.arg_name).ok_or(PlaceholderParseError::NotExistingArgument)?;

    let filling_value = match value {
        ArgumentValue::Text(t) => TextConstraint.fill_placeholder(value, &placeholder.args),
        ArgumentValue::Number(t) => NumberConstraint.fill_placeholder(value, &placeholder.args),
        ArgumentValue::Flag(t) => FlagConstraint.fill_placeholder(value, &placeholder.args),
        ArgumentValue::Choice(t) => TextConstraint.fill_placeholder(value, &placeholder.args),
    }?;

    let mut bytes = original
        .as_bytes()
        .to_vec();

    bytes.splice(
        placeholder.range.clone(),
        format!("{}{}", placeholder.prefix, filling_value).as_bytes().to_vec()
    );

    match String::from_utf8(bytes) {
        Ok(t) => Ok((t, Range { start: placeholder.range.start, end: placeholder.range.start + filling_value.len() } )),
        Err(_) => Err(PlaceholderParseError::CorruptedDuringFill)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use std::{ops::Range, collections::HashMap};

    use crate::{map, domain::ArgumentValue};

    use super::{fill_first_placeholder, Placeholder};

    #[rstest(placeholder, expected, value, placeholder_arg,
        case("1234____9012", "123456789012", ArgumentValue::Text("5678".to_string()), None),
        case("あいう____く", "あいうえおかきく", ArgumentValue::Text("えおかき".to_string()), None),
        case("1 + 1 = ____", "1 + 1 = 2", ArgumentValue::Number(2f64), None),
        case(
            "けもみみはいい: ____",
            "けもみみはいい: 正しい",
            ArgumentValue::Flag(true),
            Some(map!("true" => "正しい", "false" => "正しくない"))
        ),
    )]
    fn can_fill_placeholder(placeholder: &str, expected: &str, value: ArgumentValue, placeholder_arg: Option<HashMap<&str, &str>>) {
        let value_map = map!("fill".to_string() => value);

        let blanket_start = (placeholder.find('_').unwrap());

        let filled = fill_first_placeholder(placeholder, &value_map, &Placeholder {
            range: blanket_start..(blanket_start + 4),
            arg_name: "fill".to_string(),
            prefix: "".to_string(),
            args: placeholder_arg.unwrap_or_default().iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
        });
        assert_eq!(filled.map(|(s, r)| s), Ok(expected.to_string()))
    }
}
