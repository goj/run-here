use errors::Error;
use swayipc::Connection;
use std::{env, process, path::Path};

mod errors;
mod pid;
mod processes;
mod windows;

fn main() -> Result<(), errors::Error> {
    let argv: Vec<String> = env::args().skip(1).collect();
    if argv.len() < 1 {
        println!("Must specify command to execute");
        process::exit(1);
    }
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    let focused_pid = windows::get_focused_pid(tree)?;
    let cwds = processes::leaf_cwds(focused_pid)?;
    if cwds.len() == 1 {
        return run_here(cwds[0].as_path(), argv);
    }
    println!("cwds: {:?}", cwds);
    Ok(())
}

fn run_here(cwd: &Path, argv: Vec<String>) -> Result<(), errors::Error> {
    let cmd = &argv[0];
    argv.clone().drain(0..1);
    env::set_current_dir(cwd)?;
    return Err(Error::from(exec::Command::new(cmd).args(&argv).exec()));
}
