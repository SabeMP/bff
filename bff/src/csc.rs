use std::io::{Read, Write};

use crate::BffResult;

pub fn csc_copy<R: Read, W: Write>(reader: R, writer: &mut W) -> BffResult<()> {
    for byte in reader.bytes() {
        let byte = byte?;
        writer.write_all(&[!byte])?;
    }

    Ok(())
}

pub fn csc_buffer(data: &mut [u8]) {
    for byte in data {
        *byte = !*byte;
    }
}
