use std::{
    fs::File,
    io::{BufWriter, Result as IoResult, Write},
};

use crate::changelog::Changelog;

pub fn new(file: File) -> IoResult<()> {
    let mut stream = BufWriter::new(file);
    stream.write_all(Changelog::new().as_bytes())?;

    Ok(())
}
