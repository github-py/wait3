use chrono::Duration;
use regex::Regex;
use spinners::{Spinner, Spinners};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use subprocess::{Exec, Redirection};

pub fn for_time(amount: &str) {
    let smh = Regex::new(r"^(\d+)(s|m|h)$").unwrap();
    match smh.captures(&amount) {
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
            let mut indicator =
                Spinner::with_timer(Spinners::Triangle, format!("Waiting for {}", amount));
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

pub fn for_shell_command(command: &str) {
    let (txout, rxout) = channel();
    // let (txerr, rxerr) = channel();

    let thread_command = String::from(command.clone());
    let subprocess_thread = thread::spawn(move || {
        let cmd = Exec::cmd(thread_command.to_owned());

        let out = cmd
            .stdout(Redirection::Pipe)
            .capture()
            .expect("failed to capture stdout")
            .stdout_str();
        // let err = cmd.clone()
        //     .stderr(Redirection::Pipe)
        //     .capture()
        //     .expect("failed to capture stdout")
        //     .stderr_str();

        txout
            .send(out.to_owned())
            .expect("Unable to send stdout to main thread");
        // txerr
        //     .send(err.to_owned())
        //     .expect("Unable to send stderr to main thread");
    });

    let stdout_receiver = thread::spawn(move || {
        let value = rxout.recv().expect("Unable to receive stdout from channel");
        println!("{value}");
    });
    // let stderr_receiver = thread::spawn(move || {
    //     let value = rxerr.recv().expect("Unable to receive stderr from channel");
    //     println!("{value}");
    // });
    let mut indicator = Spinner::new(
        Spinners::Triangle,
        format!("Waiting for command `{}`", command),
    );

    println!("Waiting for command: `{}`", command);
    subprocess_thread
        .join()
        .expect("The subprocess_thread thread has panicked");
    stdout_receiver
        .join()
        .expect("The stdout receiver thread has panicked");
    // stderr_receiver
    //     .join()
    //     .expect("The stderr receiver thread has panicked");
    indicator.stop_with_message(format!("The command `{}` has finished", command));
}
