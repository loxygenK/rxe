use std::collections::HashMap;

use crate::domain::{Config, InputtedCommand, Argument, ArgumentValue, Command, Constraint, ConstraintValidateError};

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
    MalformedArgument(ConstraintValidateError),
}

pub fn parse(config: &Config, line: &[impl ToString]) -> Result<InputtedCommand, ParseError> {
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
        for arg_text in &self.arg {
            let option = self.parse_option(arg_text)?;
            let expected_arg = self.get_value_expected_arg();

            if expected_arg.is_some() == option.is_some() {
                return Err(ParseError::MalformedLine);
            }

            let applicable_option = option.as_ref().or(expected_arg.as_ref()).ok_or(ParseError::MalformedLine)?;
            let arg = self.cmd.get_argument(&applicable_option)
                .ok_or_else(|| panic!("WTF: '{}' existed in the status, but not in the command", &applicable_option))?;
            let status = self.args_status.get_mut(applicable_option).ok_or(ParseError::ArgumentNotExist)?;

            match option {
                Some(_) => {
                    if arg.constraint == Constraint::Flag {
                        *status = ParseStatus::Parsed(ArgumentValue::Flag(true));
                    } else {
                        *status = ParseStatus::ExpectingNext;
                    }
                },
                None => {
                    let value = arg_text.trim_matches(|c| c == '"' || c == '\'');
                    let value = arg.constraint.convert_value(&value).map_err(ParseError::MalformedArgument)?;

                    *status = ParseStatus::Parsed(value);
                }
            }
        }

        Ok(InputtedCommand {
            name: self.cmd.name.to_owned(),
            args: self.status_to_values()?
        })
    }

    fn status_to_values(&self) -> Result<HashMap<String, ArgumentValue>, ParseError> {
        self.args_status.iter().map(|(k, v)| {
            let k = k.to_owned();

            match v {
                ParseStatus::NotParsed => {
                    if self.cmd.get_argument(&k).expect("").constraint == Constraint::Flag {
                        Ok((k, ArgumentValue::Flag(false)))
                    } else {
                        Err(ParseError::InsufficientArgument)
                    }
                },
                ParseStatus::ExpectingNext => {
                    Err(ParseError::MalformedLine)
                },
                ParseStatus::Parsed(v) => {
                    Ok((k, v.clone()))
                }
            }
        }).collect::<Result<HashMap<_, _>, _>>()
    }

    fn parse_option(&self, str: impl ToString) -> Result<Option<String>, ParseError> {
        let str = str.to_string();
        let arg_name = str.trim_start_matches("-");
        let trimed_dashes = str.len() - arg_name.len();

        if trimed_dashes != 0 && arg_name.len() == 0 {
            return Err(ParseError::MalformedLine);
        }

        match trimed_dashes {
            0 => Ok(None),
            1 => if arg_name.len() == 1 {
                     Ok(Some(arg_name.to_owned()))
                 } else {
                     Err(ParseError::MalformedLine)
                 },
            2 => Ok(Some(arg_name.to_owned())),
            _ => Err(ParseError::MalformedLine)
        }
    }

    fn get_value_expected_arg(&self) -> Option<String> {
        self.args_status
            .iter()
            .find(|(_, v)| **v == ParseStatus::ExpectingNext)
            .map(|(k, _)| k.to_string())
    }

    fn get_mut_status(&mut self, key: &str) -> Result<&mut ParseStatus, ParseError> {
        self.args_status.get_mut(key).ok_or(ParseError::ArgumentNotExist)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rstest::rstest;

    use crate::{domain::{Config, ArgumentValue, ConstraintValidateError}, config::DeserializedConfig};

    use super::{parse, ParseError};

    #[rstest(input, expected,
        case(
            vec!["test", "--type", "core", "--snapshot"],
            Ok(crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Choice("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(true),
            ))
        ),
        case(
            vec!["test", "--snapshot", "--type", "core"],
            Ok(crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Choice("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(true),
            ))
        ),
        case(
            vec!["test", "--type", "core"],
            Ok(crate::map!(<String, ArgumentValue>;
                "type".to_owned() => ArgumentValue::Choice("core".to_owned()),
                "snapshot".to_owned() => ArgumentValue::Flag(false),
            ))
        ),
        case(
            vec!["test", "--type", "hoge"],
            Err(ParseError::MalformedArgument(ConstraintValidateError::AmbiguosChoice))
        ),
        case(
            vec!["test", "--snapshot"],
            Err(ParseError::InsufficientArgument)
        ),
        case(
            vec!["test", "--snapshot", "--type"],
            Err(ParseError::MalformedLine)
        ),
        case(
            vec!["test", "--type", "--snapshot"],
            Err(ParseError::MalformedLine)
        ),
    )]
    fn accept_correct_input(input: Vec<&str>, expected: Result<HashMap<String, ArgumentValue>, ParseError>) {
        let config: Config = serde_yaml::from_str::<DeserializedConfig>(include_str!("../tests/acceptable_config.yaml")).unwrap().into();

        let result = parse(&config, &input);
        assert_eq!(result.map(|r| r.args), expected)
    }
}
