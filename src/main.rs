// #![allow(unused)]
use clap::{Parser, Subcommand};
use wait3 as wait;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help(true))]
    Time {
        amount: String,
    },
    #[command(arg_required_else_help(true))]
    #[command(alias("command"))]
    #[command(alias("shell-command"))]
    Sh {
        command: String,
    },
}

pub fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Time { amount } => {
            wait::for_time(amount);
        }
        Commands::Sh { command } => {
            wait::for_shell_command(command);
        }
    }
}
