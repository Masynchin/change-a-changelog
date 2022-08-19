use clap::{Command, SubCommand};

pub fn cmd() -> Command<'static> {
    Command::new("change-a-changelog")
        .bin_name("change")
        .subcommand_required(true)
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates new CHANGELOG.md")
        )
}
