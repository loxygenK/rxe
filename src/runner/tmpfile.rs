use std::{fs::File, io::Write, env, path::MAIN_SEPARATOR};

use crate::util::get_random_string;

use super::ExecuteError;

fn create_temporal_file(suffix: &str) -> Option<String> {
    let tmp_dir = env::temp_dir();
    let tmp_dir = tmp_dir.to_str()?;

    Some(format!(
        "{}{}{}.{}",
        tmp_dir,
        MAIN_SEPARATOR,
        get_random_string(),
        suffix
    ))
}

pub fn create_script_file(suffix: &str, line: &str) -> Result<String, ExecuteError> {
    let tmp_file_path = create_temporal_file(suffix)
       .ok_or(ExecuteError::UnknownEnvironment)?;
    let mut tmp_file = File::create(tmp_file_path.clone())
        .map_err(ExecuteError::PreparationFailure)?;

    tmp_file.write_all(line.as_bytes())
        .map_err(ExecuteError::PreparationFailure)?;

    Ok(tmp_file_path)
}
