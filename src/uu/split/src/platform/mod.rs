#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(not(unix), path = "generic.rs")]
mod imp;

pub use imp::instantiate_current_writer;
