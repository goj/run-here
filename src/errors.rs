use exec::Error as ExecError;
use procfs::ProcError;
use std::io::Error as IoError;
use swayipc::Error as SwayError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No command specified")]
    NoCommandSpecified,
    #[error("IPC error: {0}")]
    IpcFailed(#[from] SwayError),
    #[error("/proc access error: {0}")]
    CouldNotReadProcFs(#[from] ProcError),
    #[error("IO error: {0}")]
    IoFailed(#[from] IoError),
    #[error("No focused window found")]
    NoFocusedWindow,
    #[error("Failure executing the program: {0}")]
    ExecFailed(#[from] ExecError),
}
