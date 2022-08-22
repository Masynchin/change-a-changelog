mod app;
mod changelog;
mod cmd;

use app::new;
use cmd::cmd;

fn main() -> Result<(), std::io::Error> {
    match cmd().get_matches().subcommand() {
        Some(("new", _)) => new(),
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
