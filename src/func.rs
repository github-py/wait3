use crate::cmd::{shell, Bash};
use std::sync::mpsc::channel;
use std::thread::{self, sleep};
use std::time::Duration;
use std::process::Output;


pub fn wait_for_subprocess(shell_command: String) -> Output {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let err = format!("could not send output of command {:?} to parent thread", shell_command);
        let output = shell::<Bash>(&shell_command);
        tx.send(output).expect(&err);
    });
    let interval = Duration::from_millis(10);
    loop {
        match rx.try_recv() {
            Ok(output) => {
                return output.clone()
            }
            Err(_) => {
                sleep(interval);
            }
        }
    }

}
