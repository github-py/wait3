// use crate::coreio::ensure_dir_exists;
// use crate::errors::Error;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(arg_required_else_help(true), about = "waits for units of time")]
    Time(TimeOpts),
    #[command(arg_required_else_help(true), about = "waits for subprocess (i.e.: command)")]
    Subp(SubprocessOpts),
    #[command(arg_required_else_help(true), about = "waits for command (i.e.: subprocess)")]
    Cmd(SubprocessOpts),
}

#[derive(Args, Debug)]
pub struct TimeOpts {
    pub amount: String,
}

#[derive(Args, Debug)]
pub struct SubprocessOpts {
    pub command: String,
    pub args: Vec<String>,
}
