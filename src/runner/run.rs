use std::process::Command;

use super::{ExecuteStatus, ExecuteError};

pub fn run_command(program: &str, args: &[&str]) -> Result<ExecuteStatus, ExecuteError> {
    let code = Command::new(program)
        .args(args)
        .spawn()
        .map_err(ExecuteError::PreparationFailure)?
        .wait()
        .map_err(ExecuteError::PreparationFailure)?
        .code();

    match code {
        Some(c) => Ok(ExecuteStatus::Exited(c)),
        None => Ok(ExecuteStatus::Terminated)
    }
}
