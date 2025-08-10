use anyhow::Result;
use hyprland::{data::Client as HyprlandClient, shared::HyprDataActiveOptional};

use crate::pid::Pid;

pub fn get_focused_pid_hyprland() -> Result<Option<Pid>> {
    let pid = HyprlandClient::get_active()?
        .map(|active| active.pid.into());
    Ok(pid)
}
