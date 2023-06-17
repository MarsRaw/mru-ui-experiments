//!
//! This is where we'll put all the interop with mars-raw-utls
//! 

/// Should basically run a mars-raw-utils commands, anything that app can do should be mirrored here.
pub enum BackendInstruction{
    Download(String)
}