mod app;

use app::cmd;

fn main() {
    match cmd().get_matches().subcommand() {
        Some(("new", _)) => println!("Creating CHANGELOG..."),
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
