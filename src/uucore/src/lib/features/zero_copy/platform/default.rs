use std::fs::File;
use std::io::{self, Write, Stdin, Stdout, Stderr};
use std::process::Stdio;

use crate::features::zero_copy::{AsRawObject, FromRawObject};

pub enum RawObject {}

impl AsRawObject for File {
    fn as_raw_object(&self) -> RawObject {
        unreachable!()
    }
}

impl AsRawObject for Stdin {
    fn as_raw_object(&self) -> RawObject {
        unreachable!()
    }
}

impl AsRawObject for Stdout {
    fn as_raw_object(&self) -> RawObject {
        unreachable!()
    }
}

impl AsRawObject for Stderr {
    fn as_raw_object(&self) -> RawObject {
        unreachable!()
    }
}

impl FromRawObject for File {
    unsafe fn from_raw_object(obj: RawObject) -> Option<Self> {
        match obj {}
    }
}

impl FromRawObject for Stdio {
    unsafe fn from_raw_object(obj: RawObject) -> Option<Self> {
        match obj {}
    }
}

pub struct PlatformZeroCopyWriter;

impl PlatformZeroCopyWriter {
    pub unsafe fn new(_obj: RawObject) -> Result<Self, ()> {
        Err(())
    }
}

impl Write for PlatformZeroCopyWriter {
    fn write(&mut self, _bytes: &[u8]) -> io::Result<usize> {
        unreachable!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unreachable!()
    }
}
