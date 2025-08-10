use anyhow::Result;

use crate::{cli::parse_args, execute::run_command, sway::get_focused_pid_sway};

mod cli;
mod errors;
mod execute;
mod pid;
mod processes;
mod sway;

fn main() -> Result<()> {
    let args = parse_args()?;
    run_command(&args, get_focused_pid_sway()?)
}
