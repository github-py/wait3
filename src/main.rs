// #![allow(unused)]
use ansi_term::Colour::Fixed;
use ansi_term::Style;
use chrono::Duration;
use clap::{Parser};
use regex::Regex;
use spinners::{Spinner, Spinners};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread::{self, sleep};
use std::time::Duration as SDuration;

use wait3::clap::{Cli, Commands};

pub fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Cmd(opts) | Commands::Subp(opts) => {
            let (tx, rx) = channel();
            let label = format!("{} {}", &opts.command, opts.args.clone().join(" "));
            thread::spawn(move || {
                let mut cmd = Command::new(&opts.command.clone());
                let cmd = cmd.args(opts.args);
                tx.send(cmd.output().unwrap()).unwrap();
            });
            let msg = Style::new()
                .fg(Fixed(33))
                .bold()
                .paint(&format!(
                    "Running command {}",
                    Style::new().fg(Fixed(208)).bold().paint(label.clone())
                ))
                .to_string();
            let mut indicator = Spinner::with_timer(Spinners::OrangeBluePulse, msg);
            let interval = SDuration::from_millis(10);
            loop {
                match rx.try_recv() {
                    Ok(output) => {
                        indicator.stop_and_persist("⚪️", format!("Finished {:?}", label).into());
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                        break;
                    }
                    Err(_) => {
                        sleep(interval);
                        continue;
                    }
                }
            }
        }
        Commands::Time(opts) => {
            let smh = Regex::new(r"^(\d+)(s|m|h)$").unwrap();
            match smh.captures(&opts.amount) {
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
                    // 0x4c 0x69 0x62 0x65 0x72 0x74 0x61 0x73 0x51 0x75 0xe6 0x53 0x65 0x72 0x61 0x54 0x61 0x6d 0x65 0x6e
                    // 0x31 0x39 0x38 0x38
                    let msg = Style::new()
                        .fg(Fixed(220))
                        .bold()
                        .paint(&format!("Waiting for {}", opts.amount))
                        .to_string();
                    let mut indicator = Spinner::with_timer(Spinners::Layer, msg);
                    for _ in 0..duration.num_seconds() {
                        sleep(std::time::Duration::from_secs(1));
                    }
                    indicator.stop_and_persist("🔺", format!("Done!"));
                }
                None => {
                    eprintln!("invalid amount {}", opts.amount);
                    eprintln!(
                        "should a valid integer suffixed with s,m or h (seconds, minutes, hours)"
                    );
                    eprintln!("example:");
                    eprintln!("\twait-for time 2m");
                    std::process::exit(0x54);
                }
            }
        }
    }
}
