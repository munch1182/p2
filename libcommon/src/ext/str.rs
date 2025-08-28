use std::process::Command;

pub trait PrettyStringExt {
    /**
     * 将某些对象转为可读的形式
     */
    fn to_string_pretty(&self) -> String;
}

impl PrettyStringExt for Command {
    /**
     * 将命令转为可读的形式
     */
    fn to_string_pretty(&self) -> String {
        let program = self.get_program().to_string_lossy();
        let args: Vec<String> = self
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();

        format!("{} {}", program, args.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use log::info;

    use crate::log::log_setup;

    use super::*;

    #[test]
    fn test_command() {
        log_setup();
        let mut cmd = Command::new("ls");
        cmd.args(&["-l", "-a"]);
        info!("{}", cmd.to_string_pretty());
        assert_eq!(cmd.to_string_pretty(), "ls -l -a");
    }
}
