use std::process::Command;
use std::sync::mpsc::channel;
use std::thread::{self, sleep};
use std::time::Duration;
use std::process::Output;


pub fn wait_for_subprocess(command: String, args: Vec<String>) -> Output {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut cmd = Command::new(command);
        let cmd = cmd.args(args);
        tx.send(cmd.output().unwrap()).unwrap();
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
