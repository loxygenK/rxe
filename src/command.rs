use std::collections::HashMap;

use crate::constraints::{ValueParseError, Constraint};
use crate::domain::{Config, InputtedCommand, Argument, ArgumentValue, Command, Constraints};
use crate::constraints::{text::TextConstraint, number::NumberConstraint, choice::ChoiceConstraint, flag::FlagConstraint};

use crate::helper::replace_iter::ReplaceIter;

#[derive(Debug, PartialEq)]
pub enum ParseStatus {
    NotParsed,
    ExpectingNext,
    Parsed(ArgumentValue)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NoCommandSpecified,
    CommandNotExist,
    ArgumentNotExist,
    InsufficientArgument,
    MalformedLine,
    MalformedArgument(ValueParseError),
}

pub fn parse(config: & Config, line: & [impl ToString]) -> Result<InputtedCommand, ParseError> {
    CommandParser::new(config, line)?.parse()
}

#[derive(Debug)]
struct CommandParser<'a> {
    cmd: &'a Command,
    arg: Vec<String>,
}
impl<'a> CommandParser<'a> {
    fn new(config: &'a Config, line: &[impl ToString]) -> Result<Self, ParseError> {
        let mut line = line.iter().map(ToString::to_string);

        let cmd = line.next().ok_or(ParseError::NoCommandSpecified)?;
        let cmd = config.get_command(&cmd).ok_or(ParseError::CommandNotExist)?;

        let arg = line.collect();

        Ok(Self { cmd, arg })
    }

    fn parse(mut self) -> Result<InputtedCommand, ParseError> {
        let mut args_status = self.cmd.args.iter()
            .map(|x| (x.name.clone(), ParseStatus::NotParsed))
            .collect::<HashMap<String, ParseStatus>>();

        let mut arg_iter = self.arg.iter().peekable();
        while let Some(current) = arg_iter.next() {
            let next = arg_iter.peek();

            let current_opt = self.parse_option(current)?.ok_or(ParseError::MalformedLine)?;
            let next_opt = next.map(|n| self.parse_option(*n)).transpose()?.flatten();

            let status = args_status.get_mut(&current_opt.name).ok_or(ParseError::ArgumentNotExist)?;

            let new_status = match next_opt {
                Some(_) => {
                    let parsed = self.delegate_parse(&current_opt.constraint, None)
                        .map_err(ParseError::MalformedArgument)?;

                    ParseStatus::Parsed(parsed)
                },
                None => {
                    let parsed = self.delegate_parse(&current_opt.constraint, next.map(|s| s.as_str()))
                        .map_err(ParseError::MalformedArgument)?;

                    arg_iter.next();

                    ParseStatus::Parsed(parsed)
                }
            };

            *status = new_status;
        }

        let args = args_status.into_iter()
            .map(|(k, v)| self.unwrap_parse_status(&k, v).map(|v| (k, v)))
            .replace(
                Err(ParseError::MalformedArgument(ValueParseError::ValueRequired)),
                || Err(ParseError::InsufficientArgument)
            )
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(InputtedCommand {
            name: self.cmd.name.to_owned(),
            args
        })
    }

    fn unwrap_parse_status(&self, arg_name: &str, status: ParseStatus) -> Result<ArgumentValue, ParseError> {
        let arg = self.cmd.get_argument(arg_name)
            .unwrap_or_else(|| panic!("WTF: '{}' existed in the status, but not in the command", &arg_name));

        match status {
            ParseStatus::Parsed(v) => Ok(v),
            _ => self.delegate_fallback(&arg.constraint).map_err(ParseError::MalformedArgument)
        }
    }

    fn delegate_parse(&self, constraint: &Constraints, value: Option<&str>) -> Result<ArgumentValue, ValueParseError> {
        match constraint {
            Constraints::Text => TextConstraint.parse_value(value),
            Constraints::Flag => FlagConstraint.parse_value(value),
            Constraints::Number => NumberConstraint.parse_value(value),
            Constraints::Choice(c) => ChoiceConstraint::new(c.to_vec()).parse_value(value)
        }
    }

    fn delegate_fallback(&self, constraint: &Constraints) -> Result<ArgumentValue, ValueParseError> {
        match constraint {
            Constraints::Text => TextConstraint.fallback(),
            Constraints::Flag => FlagConstraint.fallback(),
            Constraints::Number => NumberConstraint.fallback(),
            Constraints::Choice(c) => ChoiceConstraint::new(c.to_vec()).fallback()
        }
    }

    fn parse_option(&self, str: impl ToString) -> Result<Option<&Argument>, ParseError> {
        let str = str.to_string();
        let arg_name = str.trim_start_matches('-');
        let trimed_dashes = str.len() - arg_name.len();

        if trimed_dashes != 0 && arg_name.is_empty() {
            return Err(ParseError::MalformedLine);
        }
        if trimed_dashes == 1 && arg_name.len() != 1 {
            return Err(ParseError::MalformedLine);
        }
        if trimed_dashes > 2 {
            return Err(ParseError::MalformedLine);
        }

        if trimed_dashes == 0 {
            Ok(None)
        } else {
            self.cmd.get_argument(arg_name)
                .map(Some)
                .ok_or(ParseError::ArgumentNotExist)
        }

    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rstest::rstest;

    use crate::{domain::{Config, ArgumentValue}, config::DeserializedConfig};
    use crate::constraints::{ValueParseError};

    use super::{parse, ParseError};

    #[rstest(input, expected,
        case(
            vec!["test", "--type", "core", "--snapshot"],
            crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Text("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(true),
            )
        ),
        case(
            vec!["test", "--snapshot", "--type", "core"],
            crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Text("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(true),
            )
        ),
        case(
            vec!["test", "--type", "core"],
            crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Text("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(false),
            )
        ),
    )]
    fn accept_correct_input(input: Vec<&str>, expected: HashMap<String, ArgumentValue>) {
        let config: Config = serde_yaml::from_str::<DeserializedConfig>(include_str!("../tests/acceptable_config.yaml")).unwrap().into();

        let result = parse(&config, &input);
        assert_eq!(result.unwrap().args, expected)
    }

    #[rstest(input, expected,
        case(
            vec!["test", "--snapshot"],
            ParseError::InsufficientArgument
        ),
        case(
            vec!["test", "--snapshot", "--type"],
            ParseError::MalformedArgument(ValueParseError::ValueRequired)
        ),
        case(
            vec!["test", "--type", "--snapshot"],
            ParseError::MalformedArgument(ValueParseError::ValueRequired)
        ),
    )]
    fn decline_incorrect_input(input: Vec<&str>, expected: ParseError) {
        let config: Config = serde_yaml::from_str::<DeserializedConfig>(include_str!("../tests/acceptable_config.yaml")).unwrap().into();

        let result = parse(&config, &input);
        assert_eq!(result.unwrap_err(), expected)
    }
}
