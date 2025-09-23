use std::sync::LockResult;

/// 使用[anyhow::Error]作为错误类型方便转化
pub type Err = anyhow::Error;
///
/// 使用[crate::prelude::Err]作为错误类型
///
/// 如果使用自定义类型Err方便转换，需要自定义Err实现std::fmt::Display和std::error::Error
pub type Result<T> = std::result::Result<T, Err>;

///
///
/// 创建一个新的Err
///
/// 支持:
/// ```ignore
/// newerr!("{}, {}", a, b)
/// newerr!("{a}")
/// newerr!(Err::NewErr) => newerr!("{:?}", Err::NewErr)
/// newerr!(a) => newerr!("{a:?}")
///```
///
#[macro_export]
macro_rules! newerr {
    ($fmt:literal) => {
        $crate::prelude::Err::msg(format!($fmt))
    };
    ($fmt:literal, $($arg:tt)*) => {
        $crate::prelude::Err::msg(format!($fmt, $($arg)*))
    };
    ($err:expr) => {
        $crate::prelude::Err::msg(format!("{:?}", $err))
    };
}

/// 错误类型转换，将不支持自动转换的常用错误类型手动转换
pub trait ErrMapperExt<T> {
    fn newerr(self) -> Result<T>;
}

impl<T> ErrMapperExt<T> for LockResult<T> {
    ///
    /// 将[std::sync::LockResult]转换为[Result]
    ///
    /// # example
    /// ```ignore
    /// use std::sync::{Arc, Mutex};
    /// use libcommon::prelude::{Result, ErrMapperExt};
    ///
    /// let mutex = Arc::new(Mutex::new(1));
    /// let result: Result<()> = mutex.lock().newerr();
    /// assert!(result.is_ok());
    /// ```
    fn newerr(self) -> Result<T> {
        self.map_err(|e| newerr!(e))
    }
}

impl<T> ErrMapperExt<T> for std::thread::Result<T> {
    ///
    /// 将[std::thread::Result]转换为[Result]
    ///
    /// # example
    /// ```
    /// use std::thread;
    /// use libcommon::prelude::{Result, ErrMapperExt};
    ///
    /// let result: Result<()> = thread::spawn(|| ()).join().newerr();
    /// assert!(result.is_ok());
    /// ```
    fn newerr(self) -> Result<T> {
        self.map_err(|e| newerr!(e))
    }
}

impl<T> ErrMapperExt<T> for Option<T> {
    fn newerr(self) -> Result<T> {
        self.ok_or_else(|| newerr!("err from None"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::log_setup;
    use log::debug;

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
    fn test_convert() {
        let result = _convert_err_to_anyhow();
        assert!(result.is_err());
    }

    #[test]
    fn test_newerr() {
        log_setup();
        let err: Result<()> = Err(newerr!(NewErr::NewErr));
        debug!("{:#?}", err);
        assert!(err.is_err());
        let a = 1;
        let err: Result<()> = Err(newerr!(a));
        // debug!("{err:?}");
        assert!(err.is_err());
        let a = 2;
        let err: Result<()> = Err(newerr!("{a}"));
        debug!("{err:#?}");
        assert!(err.is_err());
    }

    fn _convert_err_to_anyhow() -> Result<()> {
        Err(NewErr::NewErr)?
    }
}
