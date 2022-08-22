use std::{
    fs::OpenOptions,
    io::{BufWriter, Result as IoResult, Write},
    path::Path,
};

use git2::Repository;

use crate::changelog::Changelog;

pub fn new() -> IoResult<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("CHANGELOG.md")?;

    let repo = Repository::discover(Path::new(".")).unwrap();
    let origin = repo.find_remote("origin").unwrap();
    let url = origin
        .url()
        .unwrap()
        .to_string()
        .replace("https://github.com/", "")
        .replace(".git", "");

    let mut stream = BufWriter::new(file);
    stream.write_all(Changelog::new(url).as_string().as_bytes())?;

    Ok(())
}
