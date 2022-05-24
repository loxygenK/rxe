mod tmpfile;
mod run;

use std::{io::Error, fmt::Display};

use self::{run::run_command, tmpfile::create_script_file};

pub enum ExecuteStatus {
    Exited(i32),
    Terminated
}

pub enum ExecuteError {
    UnknownEnvironment,
    PreparationFailure(Error)
}
impl Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecuteError::UnknownEnvironment => write!(f, "The location to create temporal file could not be determined."),
            ExecuteError::PreparationFailure(e) => write!(f, "Unexpected error occured during the preparation: {}", e)
        }
    }
}

#[cfg(target_family = "windows")]
pub fn run_script(line: &str) -> Result<ExecuteStatus, ExecuteError> {
    let script_file = create_script_file("ps1", line)?;
    run_command("powershell", &["-File", &script_file])
}

#[cfg(target_family = "unix")]
pub fn run_script(line: &str) -> Result<ExecuteStatus, ExecuteError> {
    use std::{fs, os::unix::prelude::PermissionsExt};

    let script_file = create_script_file("sh", line)?;
    fs::set_permissions(&script_file, fs::Permissions::from_mode(0o755)).map_err(ExecuteError::PreparationFailure)?;
    run_command("sh", &["-c", &script_file])
}

#[cfg(not(any(target_family = "windows", target_family = "unix")))]
compile_error!("This crate cannot be built for the environment which is not either of Windows or Unix family");
