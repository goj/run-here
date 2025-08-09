use derive_more::From;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, From)]
pub struct Pid(pub i32);
