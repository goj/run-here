use swayipc::Node;
use swayipc::Error as SwayError;

#[derive(Debug)]
pub enum Error {
    IpcError(SwayError),
    NoFocusedWindow,
}

impl From<SwayError> for Error {
    fn from(err: SwayError) -> Error {
        Error::IpcError(err)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Pid(i32);

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
