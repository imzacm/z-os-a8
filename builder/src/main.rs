mod os_builder;
mod args;

use os_builder::OsBuilder;
use args::{Args, Command};

fn main() {
    let args = Args::default();
    let mut builder = OsBuilder::default();
    builder.target(args.target()).profile(args.profile());
    match args.command() {
        Command::Build => builder.build(),
        Command::Run => {
            builder.build();
            builder.run();
        }
    }
}
