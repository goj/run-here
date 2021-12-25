use super::errors::Error;
use super::pid::Pid;
use swayipc::Node;

pub fn get_focused_pid(node: Node) -> Result<Pid, Error> {
    get_focused_pid_impl(node).ok_or(Error::NoFocusedWindow)
}

fn get_focused_pid_impl(node: Node) -> Option<Pid> {
    if node.focused {
        return node.pid.map(Pid);
    }
    let children = node
        .nodes
        .into_iter()
        .chain(node.floating_nodes.into_iter());
    children.flat_map(get_focused_pid_impl).next()
}
