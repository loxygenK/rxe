use std::process::Command;
use std::{env, io::Write, fs::File,  path::MAIN_SEPARATOR};

use crate::runner::{CommandRunner, ExecuteError};
use crate::util::get_random_string;

pub struct WindowsCommandRunner;
impl CommandRunner for WindowsCommandRunner {
    fn run_command(&self, line: &str) -> Result<Option<i32>, ExecuteError> {
        let tmp_file = self.prepare_file(line)?;
        let code = Command::new("powershell")
            .args(["-File", &tmp_file])
            .spawn()
            .map_err(|e| ExecuteError::PreparationFailure(e))?
            .wait()
            .map_err(|e| ExecuteError::PreparationFailure(e))?
            .code();

        Ok(code)
    }

}

impl WindowsCommandRunner {
    fn prepare_file(&self, line: &str) -> Result<String, ExecuteError> {
        let tmp_dir = env::temp_dir();
        let tmp_dir = tmp_dir
            .to_str()
            .ok_or(ExecuteError::UnknownEnvironment)?;

        let file_path = format!(
            "{}{}{}.ps1",
            tmp_dir,
            MAIN_SEPARATOR,
            get_random_string()
        );

        let mut tmp_file = File::create(file_path.clone())
            .map_err(|e| ExecuteError::PreparationFailure(e))?;

        tmp_file.write_all(line.as_bytes())
            .map_err(|e| ExecuteError::PreparationFailure(e))?;

        Ok(file_path.clone())
    }
}
