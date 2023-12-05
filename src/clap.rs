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
    #[command(arg_required_else_help(true), about = "alias for the command command")]
    Cmd(SubprocessOpts),
    #[command(arg_required_else_help(true), about = "waits for command (i.e.: subprocess)")]
    Command(SubprocessOpts),
}

#[derive(Args, Debug)]
pub struct TimeOpts {
    pub amount: String,
}

#[derive(Args, Debug)]
pub struct SubprocessOpts {
    pub shell_command: String,

    #[arg(short = 'c', long = "exit", default_value_t = 0, help = "wait until command exits with this code")]
    pub exit_code: i32,
}
