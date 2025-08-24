///
/// 简化条件编译
///
/// # example
/// ```
/// use libcommon::if_feature;
///
/// if_feature!(not("logfile") =>
///     
///    use std::fs;
///    pub fn test() {
///    }
/// );
/// if_feature!("logfile" =>
///     
///    use std::fs;
///    pub fn test() {
///    }
/// );
/// ```
#[macro_export]
macro_rules! if_feature {
    ($feature:literal => $($code:item)*) => {
        $(
            #[cfg(feature = $feature)]
            $code
        )*
    };
    (not($feature:literal) => $($code:item)*) => {
        $(
            #[cfg(not(feature = $feature))]
            $code
        )*
    };
}
