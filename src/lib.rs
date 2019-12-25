extern crate futures;

mod reqrep;
mod blob;
mod unsafe_impl;

pub use reqrep::Service as ReqRepService;
pub use blob::{DrainBlob, ResultBlob, DecodeError, Elementary};
