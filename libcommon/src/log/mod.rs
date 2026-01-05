//！日志相关

pub(crate) mod logger;
pub(crate) mod logwriter;
pub(crate) mod logwriter_default;

pub use logger::{log_setup, log_setup_result,log_set_level};

#[cfg(feature = "logfile")]
pub use logwriter::{LogWriterTask, log_flush, log_setup_with_writer};

#[cfg(feature = "logfile_default")]
pub use logwriter_default::LogWriterDefaultTask;

/// 日志记录宏，使用`debug`打印日志且不会输出调用地址
#[macro_export]
macro_rules! record {
    ($fmt:literal) => {
       $crate::prelude::debug!(target: "log:record", "{}", format!($fmt))
    };
    // info!(logger: my_logger, key1 = 42, key2 = true; "a {} event", "log")
    // info!(logger: my_logger, "a {} event", "log")
    (logger: $logger:expr, $($arg:tt)+) => ({
        $crate::prelude::debug!(target: "log:record", $logger, $($arg)+)
    });

    // info!("a {} event", "log")
    ($($arg:tt)+) => ({$crate::prelude::debug!(target:"log:record", $($arg)+)})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_log() {
        log_setup();

        record!("test log record");
        let a = 1;
        record!("{a}");

        trace!("test log trace");
        debug!("test log debug");
        info!("test log info");
        warn!("test log warn");
        error!("test log error");
    }

    #[test]
    fn test_macro_log() {
        trace!("test macro log trace");
        debug!("test macro log debug");
        info!("test macro log info");
        warn!("test macro log warn");
        error!("test macro log error");
    }
}
