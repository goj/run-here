use std::collections::{HashMap, HashSet};
use std::fs::read_link;
use std::path::PathBuf;

use crate::pid::Pid;
use procfs::process::all_processes;

use super::errors::Error;
use multimap::MultiMap;

pub fn leaf_cwds(root_pid: Pid) -> Result<Vec<PathBuf>, Error> {
    let processes = all_processes()?;
    let mut children = MultiMap::new();
    let mut by_pid = HashMap::new();
    let mut result = HashSet::new();
    for proc in &processes {
        by_pid.insert(Pid(proc.pid), proc);
        children.insert(Pid(proc.stat.ppid), Pid(proc.pid));
    }
    add_leaf_cwds(root_pid, &children, &mut result)?;
    Ok(Vec::from_iter(result))
}

fn add_leaf_cwds(pid: Pid, children: &MultiMap<Pid, Pid>, result: &mut HashSet<PathBuf>) -> Result<(), Error> {
    if !children.contains_key(&pid) {
        result.insert(get_cwd(pid)?);
        return Ok(());
    }
    for child in children.get(&pid) {
        add_leaf_cwds(child.clone(), children, result)?;
    }
    Ok(())
}

fn get_cwd(pid: Pid) -> Result<PathBuf, Error> {
    Ok(read_link(format!("/proc/{}/cwd", pid.0))?)
}
