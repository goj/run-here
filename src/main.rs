use anyhow::{bail, Result};
use errno::Errno;
use errors::Error;
use std::env;
use swayipc::Connection;

mod errors;
mod pid;
mod processes;
mod windows;

struct CliArgs {
    cmd: String,
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = parse_args()?;
    change_directory()?;
    exec_it(&args)
}

fn parse_args() -> Result<CliArgs> {
    let mut args_iter = env::args();
    args_iter.next(); // Skip own name
    let cmd = args_iter.next().ok_or(Error::NoCommandSpecified)?;
    let args = args_iter.collect::<Vec<_>>();
    Ok(CliArgs { cmd, args })
}

fn change_directory() -> Result<()> {
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    let focused_pid = windows::get_focused_pid(tree).ok_or(Error::NoFocusedWindow)?;
    eprintln!("got focused pid: {}", focused_pid.0);
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


fn exec_it(args: &CliArgs) -> Result<()> {
    const ENOENT: Errno = Errno(2);
    match exec::Command::new(&args.cmd).args(&args.args).exec() {
        exec::Error::BadArgument(_) => bail!("Executing failed: bad argument!"),
        exec::Error::Errno(ENOENT) => bail!("Executing failed: command `{}` not found!", &args.cmd),
        exec::Error::Errno(errno) => bail!("Error {} when executing.", errno.0),
    }
}
