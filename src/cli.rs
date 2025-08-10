use anyhow::Result;
use std::env;

use clap::{CommandFactory, FromArgMatches, Parser};

#[derive(Parser)]
#[command(about = "Runs a given program in current window's working directory")]
#[command(version)]
pub struct Cli {
    #[arg(short, long, default_value_t = false, help = "Verbose output")]
    pub verbose: bool,
    #[arg(short, long, default_value_t = false, help = "Respect direnv")]
    pub direnv: bool,
    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,
}

pub fn parse_args() -> Result<Cli> {
    let program_name = env::args().into_iter().next().unwrap_or("run-here".into());
    let matches = Cli::command().name(program_name).get_matches();
    let cli = Cli::from_arg_matches(&matches)?;
    Ok(cli)
}
