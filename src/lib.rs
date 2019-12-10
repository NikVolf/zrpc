extern crate futures;

mod reqrep;
mod blob;

pub use reqrep::Service as ReqRepService;
pub use blob::{DrainBlob, ResultBlob, DecodeError};