/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Alexander Batischev <eual.jp@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "redox", path = "redox.rs")]
#[cfg_attr(target_os = "wasi", path = "wasi.rs")]
mod imp;

pub use imp::{supports_pid_checks, Pid, ProcessChecker};
