use swayipc::{Connection, Fallible, Node};

fn get_focused_pid(node: Node) -> Option<i32> {
    if node.focused {
        return node.pid;
    }
    let children = node.nodes.into_iter()
        .chain(node.floating_nodes.into_iter());
    children.flat_map(get_focused_pid).next()
}

fn main() -> Fallible<()> {
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    let focused_pid = get_focused_pid(tree);
    println!("focused_pid: {:?}", focused_pid);
    Ok(())
}
