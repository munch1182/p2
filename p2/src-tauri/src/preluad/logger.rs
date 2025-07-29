#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        crate::preluad::logger::_print_log(format_args!($($arg)*))
    };
}

pub fn _print_log(args: std::fmt::Arguments) {
    // todo 自己的日志逻辑
    println!("{}", args)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        log!("hello world");
    }
}
