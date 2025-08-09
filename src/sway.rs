use anyhow::Result;
use crate::errors::Error;

use super::pid::Pid;
use swayipc::{Connection, Node};

fn get_focused_pid(node: Node) -> Option<Pid> {
    if node.focused {
        return node.pid.map(Pid);
    }
    let children = node.nodes.into_iter().chain(node.floating_nodes);
    children.flat_map(get_focused_pid).next()
}

pub fn get_focused_pid_sway() -> Result<Pid> {
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    get_focused_pid(tree).ok_or_else(|| Error::FindingWindowPidFailed.into())
}

