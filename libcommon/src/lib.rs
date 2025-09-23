/// 拓展方法
pub mod ext;
/// 日志相关
pub mod log;
/// 预处理
///
/// 包括通用[crate::prelude::Err]和[crate::prelude::Result]类型，以及日志[crate::log]相关
pub mod prelude;

mod macroext;

pub use log::{log_setup, log_setup_result};
pub use macro_builder::{Builder, Default_With, With};
pub use macro_log::logsetup;
pub use macro_logiferr::logiferr;
pub use macro_timer::timer;

#[cfg(test)]
mod tests {
    use crate::{Builder, With, log_setup, logiferr, newerr, prelude::Result, prelude::error};

    #[test]
    fn test_macro() {
        let _ = macro_result();
    }

    #[logiferr]
    fn macro_result() -> Result<()> {
        log_setup();
        Err(newerr!("test macro result str"))
    }

    #[test]
    fn test_macro_builder() {
        let user = UserBuilder::new("Jack".to_string(), 22)
            .address("Beijing".to_string())
            .build();
        assert!(user.address.is_some());
        let user = User::default()
            .with_agent(None)
            .with_address("Beijing".to_string());
        assert!(user.agent.is_none());
    }

    #[allow(unused)]
    #[derive(With, Builder, Default)]
    struct User {
        name: String,
        #[with(skip)]
        age: u16,
        #[with(keep)]
        agent: Option<bool>,
        address: Option<String>,
    }
}
