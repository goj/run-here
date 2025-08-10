use procfs::ProcError;
use std::io::Error as IoError;
use swayipc::Error as SwayError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Sway IPC error: {0}")]
    SwayIpcFailed(#[from] SwayError),
    #[error("/proc access error: {0}")]
    CouldNotReadProcFs(#[from] ProcError),
    #[error("IO error: {0}")]
    IoFailed(#[from] IoError),
    #[error("Couldn't find suitable PWD")]
    NoSuitablePwdFound,
    #[error("Missing command to execute")]
    MissingCommand,
}
