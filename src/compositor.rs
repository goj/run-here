use anyhow::Result;

use crate::pid::Pid;
use crate::sway::get_focused_pid_sway;
use crate::hypr::get_focused_pid_hyprland;

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Compositor {
    Sway,
    Hyprland,
}

impl Compositor {
    pub fn get_focused_pid(&self) -> Result<Pid> {
        match self {
            Compositor::Sway => get_focused_pid_sway(),
            Compositor::Hyprland => get_focused_pid_hyprland(),
        }
    }
}

