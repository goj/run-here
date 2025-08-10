use std::env;

#[cfg(feature = "direnv")]
use crate::direnv::apply_direnv;
use crate::{cli::Cli, errors::Error, pid::Pid, processes};
use anyhow::{bail, Result};
use errno::Errno;

pub fn run_command(args: &Cli, maybe_pid: Option<Pid>) -> Result<()> {
    if let Some(pid) = maybe_pid {
        env::set_var("__RUN_HERE_WINDOW_PID__", pid.0.to_string());
        change_dir(pid)?;
    }
    exec_it(&args)
}

fn change_dir(pid: Pid) -> Result<()> {
    let tree = processes::build_process_tree(pid)?;
    if let Some(proc) = processes::find_process(&tree) {
        env::set_var("__RUN_HERE_DIR_PID__", proc.pid().to_string());
        let dir = proc.cwd()?;
        env::set_current_dir(dir.as_path())?;
    } else {
        eprintln!("Couldn't find a process to run the command in.");
    }
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
