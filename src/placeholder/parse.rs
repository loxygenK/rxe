use std::{collections::HashMap, convert::TryInto};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::domain::ArgumentValue;

use super::{Placeholder, PlaceholderParseError};

static PLACEHOLDER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(^|[^\\])\{(.+?)}").unwrap());

fn parse_first_placeholder(line: &str) -> Result<Option<Placeholder>, PlaceholderParseError> {
    let cap = match PLACEHOLDER_REGEX.captures(line) {
        Some(cap) => cap,
        None => return Ok(None)
    };
    let range = cap.get(0).unwrap().range();
    let prefix = &cap[1];
    let mut mat_args = cap[2].split('|');

    // SAFETY: Split iterator yields value at lease one;
    let arg_name = mat_args.next().unwrap();
    let args = mat_args
        .map(|m| {
            let [name, value]: [&str; 2] = m.splitn(2, '=')
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| PlaceholderParseError::MalformedParameter)?;

            Ok((name.to_string(), value.to_string()))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(Some(Placeholder {
        range,
        prefix: prefix.to_string(),
        arg_name: arg_name.to_string(),
        args,
    }))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rstest::rstest;

    use crate::{map, placeholder::{Placeholder, PlaceholderParseError}};

    use super::parse_first_placeholder;

    #[rstest(placeholder, arg_name, args,
        case("{args}", "args", map!(<&str, &str>)),
        case("{args|name=value}", "args", map!(<&str, &str>; "name" => "value")),
        case(
            "{args|name=value|another=hoge=fuga}",
            "args",
            map!(<&str, &str>;
                "name" => "value",
                "another" => "hoge=fuga"
            )
        ),
    )]
    fn parses_unmalformed_placeholder(placeholder: &str, arg_name: &str, args: HashMap<&str, &str>) {
        let parsed = parse_first_placeholder(placeholder)
            .expect("Should success, but failed")
            .expect("Placeholder should be found, but it couldn't be found");

        assert_eq!(
            parsed,
            Placeholder {
                range: 0..(placeholder.len()),
                prefix: "".to_string(),
                arg_name: arg_name.to_string(),
                args: args.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            }
        );
    }

    #[rstest(placeholder,
        case("{args|}"),
        case("{args|namevalue}"),
    )]
    fn declines_malformed_placeholder(placeholder: &str) {
        let error = parse_first_placeholder(placeholder)
            .expect_err("Should fail, but succeeded");

        assert_eq!(error, PlaceholderParseError::MalformedParameter);
    }

    #[rstest(placeholder,
        case(r"\{espaced} {should_be_found}"),
        case(r"{should_be_found} \{espaced}"),
        case(r"Something not \{espaced} but the placeholder that {should_be_found}"),
    )]
    fn can_find_appropriate_placeholder(placeholder: &str) {
        let parsed = parse_first_placeholder(placeholder)
            .expect("Should success, but failed")
            .expect("Placeholder should be found, but it couldn't be found");

        assert_eq!(parsed.arg_name, "should_be_found");
    }
}
