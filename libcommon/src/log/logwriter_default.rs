use crate::if_feature;

if_feature!("logfile_default" =>
    use crate::log::logwriter;

    ///
    /// logwriter::LogWriterTask使用tokio的默认实现
    ///
    /// 使用此实现需要tokio运行时
    ///
    /// # example
    ///
    /// ```ignore
    /// use libcommon::log::{LogWriterDefaultTask, log_setup_with_writer};
    /// #[tokio::main]
    /// async fn main() {
    ///    log_setup_with_writer(&LogWriterDefaultTask, ".log");
    /// }
    /// ```
    ///
    pub struct LogWriterDefaultTask;

    impl logwriter::LogWriterTask for LogWriterDefaultTask {
        fn spawn<F>(&self, log: F)
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static,
        {
            tokio::spawn(log);
        }
    }
);
