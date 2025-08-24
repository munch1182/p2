mod result;

pub use crate::prelude::result::{Err, ErrMapperExt, IgnoreErrExt, Result};
pub use log::{debug, error, info, trace, warn};
pub use macro_log::logsetup;
pub use macro_logiferr::logiferr;
pub use macro_timer::timer;

#[cfg(test)]
mod tests {
    use crate::newerr;

    use super::*;

    #[test]
    fn test_macro() -> std::io::Result<()> {
        Ok(())
    }

    #[logiferr]
    fn macro_result() -> Result<()> {
        Err(newerr!("test macro result"))
    }
}
