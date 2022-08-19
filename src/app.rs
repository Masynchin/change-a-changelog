use std::{
    fs::File,
    io::{Result as IoResult, Write, BufWriter},
};

pub fn new(file: File) -> IoResult<()> {
    let mut stream = BufWriter::new(file);
    stream.write_all("# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/0.1.0...HEAD\n".as_bytes())?;

    Ok(())
}
