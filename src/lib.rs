#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExclusiveScope {
    Cluster,
    Node,
    Process,
    Resource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExclusiveRequest {
    pub scope: ExclusiveScope,
    pub key: String,
    pub lease: Duration,
}

#[derive(Debug)]
pub struct ExclusiveError {
    pub message: String,
}

impl fmt::Display for ExclusiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.message) }
}
impl Error for ExclusiveError {}

pub trait ExclusiveLease: Send {
    fn renew(&mut self, lease: Duration) -> Result<(), ExclusiveError>;
    fn release(self: Box<Self>) -> Result<(), ExclusiveError>;
}

pub trait Exclusiveness: Send + Sync {
    fn acquire(&self, request: ExclusiveRequest) -> Result<Box<dyn ExclusiveLease>, ExclusiveError>;
}
