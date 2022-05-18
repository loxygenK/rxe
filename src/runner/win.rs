use std::{env, io::Write, fs::File, io::Error, path::MAIN_SEPARATOR};

use crate::runner::{CommandRunner, ExecuteError};
use crate::util::get_random_string;

pub struct WindowsCommandRunner;
impl CommandRunner for WindowsCommandRunner {
    fn run_command(&self, line: &str) -> Result<(), ExecuteError> {
        let tmp_file = self.prepare_file(line);

        todo!()
    }

}

impl WindowsCommandRunner {
    fn prepare_file(&self, line: &str) -> Result<String, ExecuteError> {
        let tmp_dir = env::temp_dir();
        let tmp_dir = tmp_dir
            .to_str()
            .ok_or(ExecuteError::PreparationFailure(None))?;

        let file_path = format!(
            "{}{}{}.ps1",
            tmp_dir,
            MAIN_SEPARATOR,
            get_random_string()
        );

        let mut tmp_file = File::create(file_path.clone())
            .map_err(|e| ExecuteError::PreparationFailure(Some(e)))?;

        tmp_file.write_all(line.as_bytes())
            .map_err(|e| ExecuteError::PreparationFailure(Some(e)))?;

        Ok(file_path.clone())
    }
}
