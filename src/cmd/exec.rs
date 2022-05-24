use crate::cmd::Environment;
use crate::command::parse;
use crate::config::{read_from_yaml, ReadError};
use crate::domain::{Config, InputtedCommand};
use crate::placeholder::fill_placeholder;
use crate::prompt::error;
use crate::runner::{run_script, ExecuteStatus};

const DEFAULT_FILES: [&str; 4] = ["rxe.yaml", "rxe.yml", ".rxe.yaml", ".rxe.yml"];

pub fn execute(env: Environment) -> Result<i32, i32> {
    let config = read_config(&env).map_err(|_| 1)?;
    let args = parse_command_arg(&config, &env).map_err(|_| 1)?;
    let script = generate_script(&config, &args).map_err(|_| 1)?;
    let result = run(&script).map_err(|_| 1)?;

    Ok(result)
}

fn read_config(env: &Environment) -> Result<Config, ()> {
    let candidates = env.config_file.clone()
        .map(|f| vec![f])
        .unwrap_or_else(|| DEFAULT_FILES.map(ToString::to_string).to_vec());

    for file_path in &candidates {
        let config = read_from_yaml(file_path);
        match config {
            Ok(c) => return Ok(c),
            Err(ReadError::FileNotExist) => continue,
            Err(e) => {
                error("Error occurred during reading the config file.");
                error(format!("  {}", e));
                return Err(());
            }
        }
    }

    error("Could not found any possible config file. Following files were tried:");
    error(candidates.iter().map(|c| format!("  - {}", c)).collect::<Vec<_>>().join("\n"));

    Err(())
}

fn parse_command_arg(config: &Config, env: &Environment) -> Result<InputtedCommand, ()> {
    let cmd = parse(config, &env.args[1..]);
    match cmd {
        Ok(c) => Ok(c),
        Err(e) => {
            error(format!("Could not parse the command argument: {}", e));
            error("Please check the argument you passed to `rxe`, or configuration file.");

            Err(())
        }
    }
}

fn generate_script(config: &Config, cmd: &InputtedCommand) -> Result<String, ()> {
    let script = fill_placeholder(&config.get_command(&cmd.name).unwrap().run, &cmd.args);
    match script {
        Ok(s) => Ok(s),
        Err(e) => {
            error(format!("Could not fill the placeholder of the command: {}", e));
            error("Please check the configuration file.");

            Err(())
        }
    }
}

fn run(script: &str) -> Result<i32, ()> {
    match run_script(script) {
        Ok(ExecuteStatus::Exited(c)) => Ok(c),
        Ok(ExecuteStatus::Terminated) => {
            error("The program was terminated by the signal!");

            Err(())
        }
        Err(e) => {
            error(format!("An error occurred during the execution: {}", e));

            Err(())
        }
    }
}
