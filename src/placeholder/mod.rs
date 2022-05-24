mod parse;
mod fill;

use std::{collections::HashMap, ops::Range};
use crate::domain::ArgumentValue;

#[derive(Debug, PartialEq, Eq)]
pub enum PlaceholderParseError {
    NotExistingArgument,
    MalformedParameter,
    InsufficientParameter(String),
    CorruptedDuringFill
}

#[derive(Debug, PartialEq, Eq)]
pub struct Placeholder {
    range: Range<usize>,
    arg_name: String,
    prefix: String,
    args: HashMap<String, String>,
}

pub fn fill_placeholder(line: &str, values: &HashMap<String, ArgumentValue>) -> Result<String, PlaceholderParseError> {
    let mut line = line.to_owned();
    let mut previous_index = 0;
    while let Some(p) = parse::parse_first_placeholder(&line, previous_index)? {
        let (new_line, range) = fill::fill_first_placeholder(&line, values, &p)?;
        line = new_line;
        previous_index = range.end + 1;

        if p.range.start > 2 {
            let double_slash_range = Range { start: p.range.start, end: p.range.start + 2 };
            if &line[double_slash_range.clone()] == r"\\" {
                line.replace_range(double_slash_range, r"\");
                previous_index -= 1;
            }
        }
    }

    let line = line.replace(r"\\{", r"\{");

    Ok(line)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rstest::rstest;

    use crate::{map, domain::ArgumentValue};
    use crate::placeholder::fill::fill_first_placeholder;
    use crate::placeholder::fill_placeholder;

    #[rstest(original, expected, value,
        case(
            "{is_noon|true=Hello|false=Good evening}, {target}!",
            "Hello, world!",
            map!(
                "is_noon" => ArgumentValue::Flag(true),
                "target" => ArgumentValue::Text("world".to_string())
            )
        ),
        case(
            r"\{fill} {fill} \{fill} {fill}",
            r"\{fill} FILLED \{fill} FILLED",
            map!(
                "fill" => ArgumentValue::Text("FILLED".to_string()),
            )
        ),
    )]
    fn can_fill_multiple_placeholders(original: &str, expected: &str, value: HashMap<&str, ArgumentValue>) {
        assert_eq!(
            fill_placeholder(original, &value.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
            Ok(expected.to_string())
        )
    }

    #[rstest(original, expected, value,
        case(
            r"{fill} \{fill} \\{fill} \\\{fill} \\\\{fill}",
            r"FILLED \{fill} \FILLED \\FILLED \\\FILLED",
            map!(
                "fill" => ArgumentValue::Text("FILLED".to_string()),
            )
        ),
    )]
    fn can_handle_only_single_escape(original: &str, expected: &str, value: HashMap<&str, ArgumentValue>) {
        assert_eq!(
            fill_placeholder(original, &value.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
            Ok(expected.to_string())
        )
    }
}