use log::*;
use macro_log::logsetup;

// 查看生成的代码： cargo expand --example log_test // 需要安装 cargo-expand
#[logsetup("warn")]
#[cfg(not(feature = "logfile_default"))]
pub fn main() {
    info!("Hello, world!");

    error!("cccc");
    warn!("dddd");
    info!("eeee");
}

// 查看生成的代码： cargo expand --example log_test --features logfile_default// 需要安装 cargo-expand
// 执行： cargo run --package libcommon --example log_test --features logfile_default
#[logsetup("warn", "./log")]
#[cfg(feature = "logfile_default")]
#[tokio::main]
pub async fn main() {
    info!("Hello, world!");

    error!("cccc");
    warn!("dddd");
    info!("eeee");
}
