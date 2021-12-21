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
