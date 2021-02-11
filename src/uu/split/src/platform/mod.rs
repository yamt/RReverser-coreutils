#[cfg(unix)]
pub use self::unix::instantiate_current_writer;

#[cfg(not(unix))]
pub use self::generic::instantiate_current_writer;

#[cfg(unix)]
mod unix;

#[cfg(not(unix))]
mod generic;
