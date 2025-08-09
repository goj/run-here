use anyhow::{bail, Result};
use clap::Parser;
use errno::Errno;
use std::{env, path::PathBuf};

use crate::compositor::Compositor;

mod compositor;
mod errors;
mod hypr;
mod pid;
mod processes;
mod sway;

#[derive(Parser)]
#[command(name = "run-here (for Sway & Hyprland")]
#[command(about = "Runs a given program in current window's PWD")]
#[command(version)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    #[arg(short, long, value_enum, default_value_t = Compositor::Sway)]
    compositor: Compositor,
    cmd: String,
    #[arg(last = true)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match get_directory(args.compositor) {
        Ok(dir) => env::set_current_dir(dir.as_path())?,
        Err(e) => eprintln!("Error: {e}, not changing directory."),
    }
    exec_it(&args)
}

fn get_directory(compositor: Compositor) -> Result<PathBuf> {
    let pid = compositor.get_focused_pid()?;
    Ok(processes::interesting_descendant_dir(pid)?)
}

fn exec_it(args: &Cli) -> Result<()> {
    const ENOENT: Errno = Errno(2);
    match exec::Command::new(&args.cmd).args(&args.args).exec() {
        exec::Error::BadArgument(_) => bail!("Executing failed: bad argument!"),
        exec::Error::Errno(ENOENT) => bail!("Executing failed: command `{}` not found!", &args.cmd),
        exec::Error::Errno(errno) => bail!("Error {} when executing.", errno.0),
    }
}
