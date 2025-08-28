use std::sync::LockResult;

/**
 * 使用anyhow::Error作为错误类型方便转化
 */
pub type Err = anyhow::Error;
/**
 * 使用自定义类型Err实现std::fmt::Display和std::error::Error即可
 */
pub type Result<T> = std::result::Result<T, Err>;

/**
 * 创建一个新的Err
 */
#[macro_export]
macro_rules! newerr {
    ($msg:expr) => {
        $crate::prelude::Err::msg($msg.to_string())
    };
    ($($arg:tt)*) => {
        $crate::prelude::Err::msg(format!($($arg)*))
    };
}

pub trait IgnoreErrExt {
    /**
     * 如果返回Err，则忽略并打印错误信息
     */
    fn ignore_value_err_by_log(self);
}

impl<T> IgnoreErrExt for Result<T> {
    fn ignore_value_err_by_log(self) {
        match self {
            Ok(_) => {}
            std::result::Result::Err(e) => log::warn!("err: {e}"),
        };
    }
}

pub trait ErrMapperExt<T> {
    /**
     * 转变Err类型
     */
    fn map_err_ext(self) -> Result<T>;
}

impl<T> ErrMapperExt<T> for LockResult<T> {
    fn map_err_ext(self) -> Result<T> {
        self.map_err(|e| newerr!("err: {}", e))
    }
}

impl<T> ErrMapperExt<T> for std::thread::Result<T> {
    fn map_err_ext(self) -> Result<T> {
        self.map_err(|e| newerr!("thread err: {:?}", e))
    }
}

#[cfg(test)]
mod tests {
    use crate::log::log_setup;

    use super::*;

    #[derive(Debug)]
    enum NewErr {
        NewErr,
    }

    impl std::fmt::Display for NewErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("NewErr")
        }
    }

    impl std::error::Error for NewErr {}

    #[test]
    fn test_ignore_value_err_by_log() -> Result<()> {
        log_setup();
        let err: Result<()> = Err(newerr!("realy error"));
        err.ignore_value_err_by_log();
        Ok(())
    }

    /**
     * 测试将NewErr转换为anyhow::Error
     */
    #[test]
    fn test_convert() {
        let result = _convert_err_to_anyhow();
        assert!(result.is_err());
    }

    fn _convert_err_to_anyhow() -> Result<()> {
        Err(NewErr::NewErr)?
    }
}
