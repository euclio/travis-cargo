extern crate docopt;
extern crate rustc_serialize;

pub mod coverage;
pub mod coveralls;
pub mod doc_upload;
pub mod manifest;
pub mod utils;

pub use manifest::{Manifest, Target};
