use swayipc::Connection;
use run_here::{get_focused_pid, Error};

fn main() -> Result<(), Error> {
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    let focused_pid = get_focused_pid(tree)?;
    println!("focused_pid: {:?}", focused_pid);
    Ok(())
}
