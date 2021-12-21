use swayipc::Connection;

mod errors;
mod pid;
mod windows;

fn main() -> Result<(), errors::Error> {
    let mut connection = Connection::new()?;
    let tree = connection.get_tree()?;
    let focused_pid = windows::get_focused_pid(tree)?;
    println!("focused_pid: {:?}", focused_pid);
    Ok(())
}
