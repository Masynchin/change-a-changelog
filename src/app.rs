use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
    result::Result,
};

use git2::Repository;

use crate::changelog::Changelog;

#[derive(Debug)]
pub enum NewError {
    UnableToOpenFile(String),
    UnableToFindRepo(String),
    UnableToFindOrigin(String),
    UnableToGetOriginUrl(String),
    UnableToWrite(String),
}

pub fn new() -> Result<(), NewError> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("CHANGELOG.md")
        .map_err(|e| NewError::UnableToOpenFile(e.to_string()))?;

    let repo = Repository::discover(Path::new("."))
        .map_err(|e| NewError::UnableToFindRepo(e.message().to_string()))?;
    let origin = repo
        .find_remote("origin")
        .map_err(|e| NewError::UnableToFindOrigin(e.to_string()))?;
    let url = origin
        .url()
        .ok_or(NewError::UnableToGetOriginUrl(
            "Cannot parse origin url.".to_string(),
        ))?
        .to_string()
        .replace("https://github.com/", "")
        .replace(".git", "");

    let mut stream = BufWriter::new(file);
    stream
        .write_all(Changelog::new(url).as_string().as_bytes())
        .map_err(|e| NewError::UnableToWrite(e.to_string()))?;

    Ok(())
}
