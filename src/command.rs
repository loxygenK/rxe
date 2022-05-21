use std::collections::HashMap;

use crate::constraints::{ValueParseError, Constraint};
use crate::domain::{Config, InputtedCommand, Argument, ArgumentValue, Command, Constraints};
use crate::constraints::{text::TextConstraint, number::NumberConstraint, choice::ChoiceConstraint, SpecificParseError, flag::FlagConstraint};

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
    args_status: HashMap<String, ParseStatus>
}
impl<'a> CommandParser<'a> {
    fn new(config: &'a Config, line: &[impl ToString]) -> Result<Self, ParseError> {
        let mut line = line.iter().map(ToString::to_string);

        let cmd = line.next().ok_or(ParseError::NoCommandSpecified)?;
        let cmd = config.get_command(&cmd).ok_or(ParseError::CommandNotExist)?;

        let arg = line.collect();

        let args_status = cmd.args.iter()
            .map(|x| (x.name.clone(), ParseStatus::NotParsed))
            .collect::<HashMap<String, ParseStatus>>();

        Ok(Self { cmd, arg, args_status })
    }

    fn parse(mut self) -> Result<InputtedCommand, ParseError> {
        let mut args_status = self.cmd.args.iter()
            .map(|x| (x.name.clone(), ParseStatus::NotParsed))
            .collect::<HashMap<String, ParseStatus>>();

        for arg_text in &self.arg {
            let option = self.parse_option(arg_text)?;
            let expected_arg = self.args_status
                .iter()
                .find(|(_, v)| **v == ParseStatus::ExpectingNext)
                .map(|(k, _)| self.get_argument_or_fail(&k.to_string()));

            let focusing_arg = expected_arg.or(option).ok_or(ParseError::MalformedLine)?;

            let status = args_status.get_mut(&focusing_arg.name).ok_or(ParseError::ArgumentNotExist)?;

            let new_status = match (option, expected_arg) {
                (Some(opt), Some(parsing)) => {
                    let parsed = self.delegate_parse(&parsing.constraint, None)
                        .map_err(ParseError::MalformedArgument)?;

                    ParseStatus::Parsed(parsed)
                },
                (Some(opt), None) => {
                    ParseStatus::ExpectingNext
                },
                (None, Some(parsing)) => {
                    let value = arg_text.trim_matches(|c| c == '"' || c == '\'');
                    let parsed = self.delegate_parse(&parsing.constraint, Some(value))
                        .map_err(ParseError::MalformedArgument)?;

                    ParseStatus::Parsed(parsed)
                },
                (None, None) => {
                    return Err(ParseError::MalformedLine);
                }
            };

            *status = new_status;
        }

        let args = args_status.into_iter()
            .map(|(k, v)| {
                let value = match v {
                    ParseStatus::Parsed(v) => v,
                    _ => self.delegate_parse(&self.get_argument_or_fail(&k).constraint, None).map_err(ParseError::MalformedArgument)?
                };

                Ok((k, value))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(InputtedCommand {
            name: self.cmd.name.to_owned(),
            args
        })
    }

    fn delegate_parse(&self, constraint: &Constraints, value: Option<&str>) -> Result<ArgumentValue, ValueParseError> {
        match constraint {
            Constraints::Text => TextConstraint.parse_value(value),
            Constraints::Flag => FlagConstraint.parse_value(value),
            Constraints::Number => NumberConstraint.parse_value(value),
            Constraints::Choice(c) => ChoiceConstraint::new(c.to_vec()).parse_value(value)
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

    fn get_value_expected_arg(&self) -> Option<&Argument> {
        let arg_name = self.args_status
            .iter()
            .find(|(_, v)| **v == ParseStatus::ExpectingNext)
            .map(|(k, _)| k.to_string())?;

        Some(self.get_argument_or_fail(&arg_name))
    }

    fn get_argument_or_fail(&self, arg_name: &str) -> &Argument {
        self.cmd.get_argument(arg_name)
            .unwrap_or_else(|| panic!("WTF: '{}' existed in the status, but not in the command", &arg_name))
    }

    fn get_mut_status(&mut self, key: &str) -> Result<&mut ParseStatus, ParseError> {
        self.args_status.get_mut(key).ok_or(ParseError::ArgumentNotExist)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rstest::rstest;

    use crate::{domain::{Config, ArgumentValue}, config::DeserializedConfig};
    use crate::constraints::choice::ChoiceError;

    use super::{parse, ParseError};

    #[rstest(input, expected,
        case(
            vec!["test", "--type", "core", "--snapshot"],
            crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Choice("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(true),
            )
        ),
        case(
            vec!["test", "--snapshot", "--type", "core"],
            crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Choice("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(true),
            )
        ),
        case(
            vec!["test", "--type", "core"],
            crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Choice("core".to_owned()),
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
            ParseError::MalformedLine
        ),
        case(
            vec!["test", "--type", "--snapshot"],
            ParseError::MalformedLine
        ),
    )]
    fn decline_incorrect_input(input: Vec<&str>, expected: ParseError) {
        let config: Config = serde_yaml::from_str::<DeserializedConfig>(include_str!("../tests/acceptable_config.yaml")).unwrap().into();

        let result = parse(&config, &input);
        assert_eq!(result.unwrap_err(), expected)
    }
}
