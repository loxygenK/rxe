mod tmpfile;
mod run;

use std::io::Error;

use self::{run::run_command, tmpfile::create_script_file};

pub enum ExecuteStatus {
    Exited(i32),
    Terminated
}

pub enum ExecuteError {
    UnknownEnvironment,
    PreparationFailure(Error)
}

#[cfg(target_family = "windows")]
pub fn run_script(line: &str) -> Result<ExecuteStatus, ExecuteError> {
    let script_file = create_script_file("ps1", line)?;
    run_command("powershell", &["-File", &script_file])
}

#[cfg(target_family = "unix")]
pub fn run_script(line: &str) -> Result<ExecuteStatus, ExecuteError> {
    let script_file = create_script_file("sh", line)?;
    run_command("sh", &["-c", &script_file])
}

#[cfg(not(any(target_family = "windows", target_family = "unix")))]
compile_error!("This crate cannot be built for the environment which is not either of Windows or Unix family");
