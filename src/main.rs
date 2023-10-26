// #![allow(unused)]
use ansi_term::Colour::Fixed;
use ansi_term::Style;
use chrono::Duration;
use clap::{Parser};
use regex::Regex;
use spinners::{Spinner, Spinners};


use std::thread::{sleep};


use wait3::clap::{Cli, Commands};
use wait3::func::wait_for_subprocess;

pub fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Cmd(opts) | Commands::Command(opts) | Commands::Subp(opts) => {
            let label = format!("{} {}", &opts.command, opts.args.clone().join(" "));

            let msg = Style::new()
                .fg(Fixed(33))
                .bold()
                .paint(&format!(
                    "Running command {}\r",
                    Style::new().fg(Fixed(208)).bold().paint(label.clone())
                ))
                .to_string();
            let mut indicator = Spinner::with_timer(Spinners::OrangeBluePulse, msg);
            let mut output = wait_for_subprocess(opts.command.clone(), opts.args.clone());
            while Some(opts.exit_code) != output.status.code() {
                output = wait_for_subprocess(opts.command.clone(), opts.args.clone());
            }
            indicator.stop_with_newline();
        }
        Commands::Time(opts) => {
            let smh = Regex::new(r"^(\d+)(s|m|h|ms|ml|ns)$").unwrap();
            match smh.captures(&opts.amount) {
                Some(caps) => {
                    let number = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let duration: Duration = match caps.get(2).unwrap().as_str() {
                        "ns" => Duration::nanoseconds(number.try_into().expect("invalid integer")),
                        "ms" => Duration::microseconds(number.try_into().expect("invalid integer")),
                        "ml" => Duration::milliseconds(number.try_into().expect("invalid integer")),
                        "s" => Duration::seconds(number.try_into().expect("invalid integer")),
                        "m" => Duration::minutes(number.try_into().expect("invalid integer")),
                        "h" => Duration::hours(number.try_into().expect("invalid integer")),
                        measure => {
                            eprintln!("measure of time duration not implemented {}", measure);
                            std::process::exit(0x54);
                        }
                    };
                    // 0x4c 0x69 0x62 0x65 0x72 0x74 0x61 0x73 0x51 0x75 0xe6 0x53 0x65 0x72 0x61 0x54 0x61 0x6d 0x65 0x6e
                    // 0x31 0x39 0x38 0x38
                    let msg = Style::new()
                        .fg(Fixed(220))
                        .bold()
                        .paint(&format!("Waiting for {}", opts.amount))
                        .to_string();
                    let mut indicator = Spinner::with_timer(Spinners::Layer, msg);
                    for _ in 0..duration.num_seconds() {
                        sleep(std::time::Duration::from_millis(1));
                    }
                    indicator.stop_with_newline();
                }
                None => {
                    eprintln!("invalid amount {}", opts.amount);
                    eprintln!(
                        "should a valid integer suffixed with {{ns,ms,ml}},s,m or h ({{nano,micro,milli,}}seconds, minutes, hours)"
                    );
                    eprintln!("example:");
                    eprintln!("\twait-for time 2m");
                    std::process::exit(0x54);
                }
            }
        }
    }
}
