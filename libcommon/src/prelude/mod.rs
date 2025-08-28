mod result;

pub use crate::prelude::result::{Err, ErrMapperExt, IgnoreErrExt, Result};
pub use log::{debug, error, info, trace, warn};
pub use macro_log::logsetup;
pub use macro_logiferr::logiferr;
pub use macro_timer::timer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{log::log_setup, newerr};

    #[test]
    fn test_macro() {
        let _ = macro_result();
    }

    #[logiferr]
    fn macro_result() -> Result<()> {
        log_setup();
        Err(newerr!("test macro result str"))
    }
}
