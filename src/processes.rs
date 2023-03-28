use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_link;
use std::path::PathBuf;

use crate::pid::Pid;
use procfs::process::all_processes;
use users::get_current_uid;

use super::errors::Error;
use multimap::MultiMap;

pub fn leaf_cwds(root_pid: Pid) -> Result<Vec<PathBuf>, Error> {
    let processes = all_processes()?;
    let mut children = MultiMap::new();
    let mut by_pid = HashMap::new();
    let mut result = HashSet::new();
    let current_uid = get_current_uid();
    for proc in &processes {
        if proc.owner != current_uid || should_ignore(&proc.stat.comm) {
            continue;
        }
        by_pid.insert(Pid(proc.pid), proc);
        children.insert(Pid(proc.stat.ppid), Pid(proc.pid));
    }
    add_leaf_cwds(0, root_pid, &children, &mut result)?;
    Ok(Vec::from_iter(result))
}

fn should_ignore(comm: &str) -> bool {
    comm == "wl-copy"
        || comm == ".cargo-wrapped"
        || comm == "make"
        || comm == "pyright-langser"
        || comm == "node"
}

fn add_leaf_cwds(
    n: usize,
    pid: Pid,
    children: &MultiMap<Pid, Pid>,
    result: &mut HashSet<PathBuf>,
) -> Result<(), Error> {
    print_debug(n, pid, children);
    if !children.contains_key(&pid) {
        result.insert(get_cwd(pid)?);
        return Ok(());
    }
    if let Some(child) = children.get(&pid) {
        add_leaf_cwds(n + 1, *child, children, result)?;
    }
    Ok(())
}

fn print_debug(n: usize, pid: Pid, children: &MultiMap<Pid, Pid>) {
    eprintln!(
        "{}PID: {} -> {:?} {}",
        padding(n),
        pid.0,
        children.get(&pid),
        procfs::process::Process::new(pid.0).unwrap().stat.comm
    );
}

fn padding(n: usize) -> String {
    String::from_utf8(vec![b' '; n]).unwrap()
}

fn get_cwd(pid: Pid) -> Result<PathBuf, Error> {
    let cwd = read_link(format!("/proc/{}/cwd", pid.0))?;
    eprintln!("/proc/{}/cwd -> {:?}", pid.0, cwd);
    Ok(cwd)
}
