pub use std::process::{Command, Output};

// pub trait Shell<'a>: Display {
pub trait Shell<'a> {
    fn new() -> Self where Self: Sized;
    fn command(&self) -> String {
        "/usr/bin/env".to_string()
    }
    fn get_path(&self) -> String;
    fn exec_params(&self) -> Vec<String> {
        vec!["-c".to_string()]
    }
    // fn fmt(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
    //     write!(f, "{} {} ", self.ex_path(), self.sh_arg())
    // }

    fn spawn(&self, shell_command: &str) -> Result<Output, std::io::Error> {
        let shell_executable_path = self.get_path();
        let mut args: Vec<String> = if shell_executable_path.len() > 0 {
            vec![shell_executable_path]
        } else {
            Vec::new()
        };
        args.extend(self.exec_params());
        args.push(format!("{}", shell_command));
        let mut cmd = Command::new(&self.command());
        Ok(cmd.args(args).output()?)
    }
}
// impl Display for dyn Shell<'_>+'static {
//     // fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
//     //     write!(f, "{} {} ", self.ex_path(), self.sh_arg())
//     // }
// }

#[derive(Debug, Clone)]
pub struct Bash {}

impl Shell<'_> for Bash {
    fn new() -> Bash {
        Bash {}
    }
    fn get_path(&self) -> String {
        format!("bash")
    }

}

pub fn shell<S: for <'a> Shell<'a>>(cmd: &str) -> Output {
    let shell = S::new();
    shell.spawn(cmd).expect(&format!("could not execute shell command {:?}", cmd))
}
