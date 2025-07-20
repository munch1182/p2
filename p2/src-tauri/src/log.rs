#[macro_export]
macro_rules! log {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        // todo 自己的日志逻辑
        println!($($arg)*);
    }};
}
