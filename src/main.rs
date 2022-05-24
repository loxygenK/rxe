mod config;
mod constraints;
mod command;
mod domain;
mod helper;
mod macros;
mod placeholder;
mod runner;
mod util;
mod cmd;
mod prompt;

use std::env;
use std::process::exit;

use prompt::error;

use crate::cmd::{Environment, exec::execute};

fn main() {
    let env = Environment {
        config_file: env::var("RXE_CONFIG").ok(),
        args: env::args().collect()
    };

    match execute(env) {
        Ok(e) => exit(e),
        Err(e) => {
            error("Exiting abnormally due to the above error.");
            exit(e);
        }
    }
}
