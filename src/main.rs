// #![allow(unused)]
use clap::{Parser, Subcommand};
use std::thread::sleep;
use regex::Regex;
use chrono::Duration;
use spinners::{Spinner, Spinners};


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
}

pub fn main() {
    let smh = Regex::new(r"^(\d+)(s|m|h)$").unwrap();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Time { amount } => {
            match smh.captures(amount) {
                Some(caps) => {
                    let number = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let duration: Duration = match caps.get(2).unwrap().as_str() {
                        "s" => Duration::seconds(number.try_into().expect("invalid integer")),
                        "m" => Duration::minutes(number.try_into().expect("invalid integer")),
                        "h" => Duration::hours(number.try_into().expect("invalid integer")),
                        measure => {
                            eprintln!("measure of time duration not implemented {}", measure);
                            std::process::exit(0x54);
                        }
                    };
                    let mut indicator = Spinner::with_timer(Spinners::Triangle, format!("Waiting for {}", amount));
                    for _ in 0..duration.num_seconds() {
                        sleep(std::time::Duration::from_secs(1));
                    }
                    indicator.stop_with_message("Done!".into());
                }
                None => {
                    eprintln!("invalid amount {}", amount);
                    eprintln!("should a valid integer suffixed with s,m or h (seconds, minutes, hours)");
                    eprintln!("example:");
                    eprintln!("\twait-for time 2m");
                    std::process::exit(0x54);
                }
            }
        }
    }
}
