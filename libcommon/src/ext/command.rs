use std::ffi::OsStr;

pub struct Command;

pub trait CommandInExt {
    ///
    /// Return the output of the command, or the error if the command failed.
    fn out_or_err(self) -> Vec<u8>;
}

impl CommandInExt for std::process::Output {
    fn out_or_err(self) -> Vec<u8> {
        if self.status.success() {
            self.stdout
        } else {
            self.stderr
        }
    }
}

impl Command {
    pub fn with_args<I, S>(program: impl AsRef<OsStr>, args: I) -> std::process::Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = std::process::Command::new(program);
        cmd.args(args);
        cmd
    }

    ///
    /// Create a command from a program and a string of arguments.
    ///
    /// The arguments are split by whitespace.
    pub fn with_str(program: impl AsRef<OsStr>, args: impl AsRef<str>) -> std::process::Command {
        let args = args.as_ref().split_whitespace();
        Self::with_args(program, args)
    }

    ///
    /// Create a command from a string.
    ///
    /// The first word is the program, the rest are arguments.
    /// panics if no arguments are provided.
    pub fn from_str(command: impl AsRef<str>) -> std::process::Command {
        let mut parts = command.as_ref().split_whitespace();
        let program = parts.next().expect("No command provided");
        Self::with_args(program, parts)
    }

    ///
    /// Create a command from a list of arguments.
    ///
    /// The first argument is the program, the rest are arguments.
    /// panics if no arguments are provided.
    pub fn from_args(args: &[impl AsRef<OsStr>]) -> std::process::Command {
        if args.is_empty() {
            panic!("No command provided")
        }
        Self::with_args(args[0].as_ref(), &args[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext::PrettyStringExt;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_command() {
        fn test(mut cmd: std::process::Command) {
            let result = cmd
                .output()
                .map(|o| String::from_utf8_lossy(&o.out_or_err()).to_string());
            println!("{} => {result:?}", cmd.to_string_pretty())
        }

        test(Command::from_str("cmd /C echo 12345"));
        test(Command::from_args(&["cmd", "/C", "echo", "12345"]));
        test(Command::with_str("cmd", "/C echo 12345"));
        test(Command::with_args("cmd", &["/C", "echo", "12345"]));

        test(Command::from_str("cmd /C netstat -ano | findstr 8080"));

        test(Command::with_str("cmd", "/C netstat -ano | findstr 8080")); // 管道命令需要指定shell，如以cmd为程序
    }
}
