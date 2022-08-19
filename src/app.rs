use std::{
    fs::File,
    io::{Result as IoResult, Write, BufWriter},
};

use crate::changelog::Changelog;

pub fn new(file: File) -> IoResult<()> {
    let mut stream = BufWriter::new(file);
    stream.write_all(Changelog::new().as_bytes())?;

    Ok(())
}
