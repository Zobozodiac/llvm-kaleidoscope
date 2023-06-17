use thiserror::Error;

pub mod expressions;
pub mod functions;

#[derive(Error, Debug, PartialEq)]
#[error("parse error: {:?}", msg)]
pub struct ParseError {
    msg: String,
}
