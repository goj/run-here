use anyhow::Result;

use crate::{cli::parse_args, execute::run_command, hypr::get_focused_pid_hyprland};

mod cli;
#[cfg(feature = "direnv")]
mod direnv;
mod errors;
mod execute;
mod hypr;
mod pid;
mod processes;

fn main() -> Result<()> {
    let args = parse_args()?;
    run_command(&args, get_focused_pid_hyprland()?)
}
