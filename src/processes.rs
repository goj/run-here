use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use crate::pid::Pid;
use procfs::process::{all_processes, Process};
use users::get_current_uid;

use super::errors::Error;
use multimap::MultiMap;

/// A SOA holding the process tree in a pre-order traversal.
#[derive(Debug)]
pub struct ProcessTree {
    processes: Vec<Process>,
    num_children: Vec<u16>,
}

pub fn build_process_tree(root: Pid) -> Result<ProcessTree, Error> {
    let processes = all_processes()?;
    let mut by_pid = HashMap::new();
    let mut parent_children = MultiMap::new();
    let current_uid = get_current_uid();

    for proc in processes.into_iter().filter_map(|res| res.ok()) {
        if proc.uid().ok() == Some(current_uid) {
            parent_children.insert(Pid(proc.stat()?.ppid), Pid(proc.pid()));
            by_pid.insert(Pid(proc.pid()), proc);
        }
    }

    let mut stack = vec![root];
    let mut result = ProcessTree {
        processes: vec![],
        num_children: vec![],
    };
    let empty = vec![];
    while let Some(proc) = stack.pop() {
        result.processes.push(by_pid.remove(&proc).unwrap());
        let children = parent_children.get_vec(&proc).unwrap_or(&empty);
        result.num_children.push(children.len().try_into().unwrap());
        for &child in children.iter().rev() {
            stack.push(child);
        }
    }
    result.processes.reverse();
    result.num_children.reverse();
    Ok(result)
}

pub fn first_interesting(tree: &ProcessTree) -> Option<&Process> {
    let editor = env::var("EDITOR").ok();
    let shell = env::var("SHELL").ok();
    tree.processes
        .iter()
        .position(|proc| {
            let cmdline = proc.cmdline().unwrap_or(vec![]);
            let cmd = cmdline.get(0);
            cmd.is_some() && (cmd == editor.as_ref() || cmd == shell.as_ref())
        })
        .map(|i| &tree.processes[i])
}

pub fn first_leaf(tree: &ProcessTree) -> Option<&Process> {
    tree.num_children
        .iter()
        .position(|&n| n == 0)
        .map(|i| &tree.processes[i])
}

pub fn interesting_descendant_dir(root_pid: Pid) -> Result<PathBuf, Error> {
    let tree = build_process_tree(root_pid)?;
    let proc = first_interesting(&tree)
        .or_else(|| first_leaf(&tree))
        .ok_or(Error::NoSuitablePwdFound)?;
    Ok(proc.cwd()?)
}
