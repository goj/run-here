use errors::Error;
use std::env;
use std::process;
use swayipc::Connection;

mod errors;
mod pid;
mod processes;
mod windows;

fn main() -> Result<(), errors::Error> {
    let argv: Vec<String> = env::args().skip(1).collect();
    if argv.len() < 1 {
        eprintln!("Got args: {:?}", argv);
        println!("Must specify command to execute");
        process::exit(1);
    }
    if try_changing_directory().is_err() {
        eprintln!("Didn't change directory");
    }
    exec_it(&argv)
}

fn try_changing_directory() -> Result<(), Error> {
    let mut connection = Connection::new()?;
    eprintln!("connected");
    let tree = connection.get_tree()?;
    eprintln!("got tree");
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

fn exec_it(argv: &Vec<String>) -> Result<(), errors::Error> {
    let cmd = &argv[0];
    let mut args = argv.clone();
    args.remove(0);
    eprintln!("cmd: {:?} argv: {:?}", cmd, args);
    return Err(Error::from(exec::Command::new(cmd).args(&args).exec()));
}
