use std::env;

use crate::{cli::Cli, errors::Error, pid::Pid, processes};
#[cfg(feature = "direnv")]
use crate::direnv::apply_direnv;
use anyhow::{bail, Result};
use errno::Errno;

pub fn run_command(args: &Cli, maybe_pid: Option<Pid>) -> Result<()> {
    if let Some(pid) = maybe_pid {
        change_dir(pid)?;
    }
    exec_it(&args)
}

fn change_dir(pid: Pid) -> Result<()> {
    let dir = processes::interesting_descendant_dir(pid)?;
    env::set_current_dir(dir.as_path())?;
    Ok(())
}

fn exec_it(args: &Cli) -> Result<()> {
    const ENOENT: Errno = Errno(2);
    #[cfg(feature = "direnv")]
    if args.direnv {
        apply_direnv()?;
    }
    let cmd = args.command.get(0).ok_or(Error::MissingCommand)?;
    match exec::Command::new(cmd).args(&args.command[1..]).exec() {
        exec::Error::BadArgument(_) => bail!("Executing failed: bad argument!"),
        exec::Error::Errno(ENOENT) => bail!("Executing failed: command `{}` not found!", &cmd),
        exec::Error::Errno(errno) => bail!("Error {} when executing.", errno.0),
    }
}
