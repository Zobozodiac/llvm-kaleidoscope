use thiserror::Error;

pub mod expressions;
pub mod functions;

#[derive(Error, Debug)]
#[error("parse error: {:?}", msg)]
pub struct ParseError {
    msg: String,
}
