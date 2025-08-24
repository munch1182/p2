pub(crate) mod logger;
pub(crate) mod logwriter;
pub(crate) mod logwriter_default;

pub use logger::{log_setup, log_setup_result};

#[cfg(feature = "logfile")]
pub use logwriter::{LogWriterTask, log_flush, log_setup_with_writer};

#[cfg(feature = "logfile_default")]
pub use logwriter_default::LogWriterDefaultTask;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_log() {
        log_setup();

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
