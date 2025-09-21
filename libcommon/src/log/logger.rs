use colored::{Color, Colorize};

use crate::log::logwriter;
use crate::newerr;
use crate::prelude::Result;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::max()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let (level, color) = match record.level() {
            log::Level::Error => ("ERROR", Color::Red),
            log::Level::Warn => (" WARN", Color::Yellow),
            log::Level::Info => (" INFO", Color::Green),
            log::Level::Debug => ("DEBUG", Color::Cyan),
            log::Level::Trace => ("TRACT", Color::White),
        };
        let is_record = record.target() == "log:record";
        let mut str = if is_record {
            format!("{}", record.args())
        } else {
            format!("{}: {}", level, record.args())
        };
        if !is_record && let (Some(f), Some(l)) = (record.file(), record.line()) {
            str = format!("{str}    ===> ({f}:{l})");
        }
        println!("{}", str.color(color));
        if let Err(e) = logwriter::write(str) {
            eprintln!("log write failed {e:?}");
        }
    }

    fn flush(&self) {
        logwriter::flush();
    }
}

static LOGGER: Logger = Logger;

/**
 * 初始化日志
 * 如果已经初始化过日志会报错，忽略这个报错
 *
 * debug模式下会输出所有日志，否则会输出info及以上日志
 */
pub fn log_setup() {
    let result = log_setup_result();
    if let Err(e) = result {
        eprintln!("log setup failed {e:?}");
    }
}

/**
 * 初始化日志
 * 如果已经初始化过日志会报错
 *
 * debug模式下会输出所有日志，否则会输出info及以上日志
 */
pub fn log_setup_result() -> Result<()> {
    // 错误说明已经设置过
    log::set_logger(&LOGGER).map_err(|e| newerr!("log setup failed {:?}", e))?;
    _log_setup_level();
    Ok(())
}

#[cfg(debug_assertions)]
fn _log_setup_level() {
    log::set_max_level(log::LevelFilter::Trace);
}

#[cfg(not(debug_assertions))]
fn _log_setup_level() {
    log::set_max_level(log::LevelFilter::Info);
}
