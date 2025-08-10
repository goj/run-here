use anyhow::Result;
use std::env;

use clap::{CommandFactory, FromArgMatches, Parser};

#[derive(Parser)]
#[command(about = "Runs a given program in current window's working directory")]
#[command(version)]
pub struct Cli {
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
    pub cmd: String,
    #[arg(last = true)]
    pub args: Vec<String>,
}

pub fn parse_args() -> Result<Cli> {
    let program_name = env::args().into_iter().next().unwrap_or("run-here".into());
    let matches = Cli::command().name(program_name).get_matches();
    let cli = Cli::from_arg_matches(&matches)?;
    Ok(cli)
}
