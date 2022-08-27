mod app;
mod changelog;
mod cmd;

use app::{new, NewError};
use cmd::cmd;

fn main() -> Result<(), NewError> {
    match cmd().get_matches().subcommand() {
        Some(("new", _)) => new(),
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
