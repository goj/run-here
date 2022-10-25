use anyhow::{bail, Result};
use clap::Parser;
use errno::Errno;
use std::env;
use swayipc::Connection;

mod errors;
mod pid;
mod processes;
mod windows;

#[derive(Parser)]
#[command(name = "run-here (for Sway and i3)")]
#[command(about = "Runs a given program in current window's PWD")]
#[command(version)]
struct Cli {
    cmd: String,
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    change_directory()?;
    exec_it(&args)
}

fn change_directory() -> Result<()> {
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    let focused_pid = match windows::get_focused_pid(tree) {
        Some(pid) => pid,
        None => {
            eprintln!("No focused PID, not changing directory");
            return Ok(());
        }
    };
    let cwds = processes::leaf_cwds(focused_pid)?;
    eprintln!("got cwds: {:?}", cwds);
    if cwds.len() == 1 {
        eprintln!("only one cwd");
        env::set_current_dir(cwds[0].as_path())?;
    } else {
        eprintln!("multiple cwds");
    }
    Ok(())
}

fn exec_it(args: &Cli) -> Result<()> {
    const ENOENT: Errno = Errno(2);
    match exec::Command::new(&args.cmd).args(&args.args).exec() {
        exec::Error::BadArgument(_) => bail!("Executing failed: bad argument!"),
        exec::Error::Errno(ENOENT) => bail!("Executing failed: command `{}` not found!", &args.cmd),
        exec::Error::Errno(errno) => bail!("Error {} when executing.", errno.0),
    }
}
