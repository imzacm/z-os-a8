use crate::os_builder::{BuildProfile, Target};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub enum Command {
    Build,
    Run,
}

#[derive(Debug, Clone, StructOpt)]
pub struct Args {
    #[structopt(default_value, long)]
    target: Target,
    #[structopt(long)]
    release: bool,
    #[structopt(subcommand)]
    command: Command,
}

impl Args {
    pub fn target(&self) -> Target {
        self.target
    }

    pub fn profile(&self) -> BuildProfile {
        if self.release { BuildProfile::Release } else { BuildProfile::Debug }
    }

    pub fn command(&self) -> &Command {
        &self.command
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::from_args()
    }
}
