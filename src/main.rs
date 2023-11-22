use anyhow::{bail, Result};
use clap::Parser;
use errno::Errno;
use std::{env, path::PathBuf};
use swayipc::Connection;

use crate::errors::Error;

mod errors;
mod pid;
mod processes;
mod windows;

#[derive(Parser)]
#[command(name = "run-here (for Sway and i3)")]
#[command(about = "Runs a given program in current window's PWD")]
#[command(version)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    cmd: String,
    #[arg(last = true)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match get_directory() {
        Ok(dir) => env::set_current_dir(dir.as_path())?,
        Err(e) => eprintln!("Error: {e}, not changing directory."),
    }
    exec_it(&args)
}

fn get_directory() -> Result<PathBuf> {
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    let focused_pid = windows::get_focused_pid(tree).ok_or(Error::FindingWindowPidFailed)?;
    Ok(processes::interesting_descendant_dir(focused_pid)?)
}

fn exec_it(args: &Cli) -> Result<()> {
    const ENOENT: Errno = Errno(2);
    match exec::Command::new(&args.cmd).args(&args.args).exec() {
        exec::Error::BadArgument(_) => bail!("Executing failed: bad argument!"),
        exec::Error::Errno(ENOENT) => bail!("Executing failed: command `{}` not found!", &args.cmd),
        exec::Error::Errno(errno) => bail!("Error {} when executing.", errno.0),
    }
}
