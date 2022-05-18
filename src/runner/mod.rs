mod nix;
mod win;

use std::io::Error;

#[cfg(target_family = "windows")]
pub use win::*;

#[cfg(target_family = "unix")]
pub use nix::*;

#[cfg(not(any(
    target_family = "window",
    target_family = "unix"
)))]
compile_error!("This crate cannot be build for the platform which is not either of windows or unix.");

pub type ExitCode = u8;

pub enum ExecuteError {
    UnknownEnvironment,
    PreparationFailure(Error),
}

pub trait CommandRunner {
    fn run_command(&self, line: &str) -> Result<Option<i32>, ExecuteError>;
}
