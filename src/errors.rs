use std::fmt::Display;

#[derive(Debug)]
pub struct NoDomainsError;

impl Display for NoDomainsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no domains are available at this time")
    }
}

impl std::error::Error for NoDomainsError {}


#[derive(Debug)]
pub struct MessageRecvError;

impl Display for MessageRecvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Messages could not be received :(")
    }
}

impl std::error::Error for MessageRecvError {}

#[derive(Debug)]
pub struct Nil;

impl Display for Nil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nil")
    }
}

impl std::error::Error for Nil {}
