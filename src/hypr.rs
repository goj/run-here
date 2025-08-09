use anyhow::Result;
use hyprland::{data::Client as HyprlandClient, shared::HyprDataActiveOptional};

use crate::{errors::Error, pid::Pid};

pub fn get_focused_pid_hyprland() -> Result<Pid> {
    let pid = HyprlandClient::get_active()?
        .ok_or(Error::NoActiveHyprlandClient)?
        .pid;
    Ok(pid.into())
}
