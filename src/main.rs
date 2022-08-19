mod app;
mod changelog;
mod cmd;

use std::fs::OpenOptions;

use app::new;
use cmd::cmd;

fn main() -> Result<(), std::io::Error> {
    match cmd().get_matches().subcommand() {
        Some(("new", _)) => new(OpenOptions::new()
            .write(true)
            .create(true)
            .open("CHANGELOG.md")?),
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
