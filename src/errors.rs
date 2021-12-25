use exec::Error as ExecError;
use procfs::ProcError;
use std::io::Error as IoError;
use swayipc::Error as SwayError;

#[derive(Debug)]
pub enum Error {
    IpcError(SwayError),
    ProcFsError(ProcError),
    IoError(IoError),
    NoFocusedWindow,
    ExecFailed,
}

impl From<SwayError> for Error {
    fn from(err: SwayError) -> Error {
        Error::IpcError(err)
    }
}

impl From<ProcError> for Error {
    fn from(err: ProcError) -> Error {
        Error::ProcFsError(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

impl From<ExecError> for Error {
    fn from(_err: ExecError) -> Error {
        Error::ExecFailed
    }
}
