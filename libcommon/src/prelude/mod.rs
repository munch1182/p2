mod result;

pub use crate::log::log_setup;
pub use crate::logsetup;
pub use crate::prelude::result::{Err, ErrMapperExt, Result};
pub use log::{debug, error, info, trace, warn};
