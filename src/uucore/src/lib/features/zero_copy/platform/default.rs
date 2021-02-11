use std::fs::File;
use std::io::{self, Stderr, Stdout, Write};

use crate::features::zero_copy::AsRawObject;

pub struct RawObject;

impl AsRawObject for File {
    fn as_raw_object(&self) -> RawObject {
        RawObject
    }
}

impl AsRawObject for Stdout {
    fn as_raw_object(&self) -> RawObject {
        RawObject
    }
}

impl AsRawObject for Stderr {
    fn as_raw_object(&self) -> RawObject {
        RawObject
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
