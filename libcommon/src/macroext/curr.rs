///
/// 简化当前目录获取
///
/// # example
/// ```
/// use libcommon::curr_dir;
///
/// let dir = curr_dir!().unwrap();
/// let dir = curr_dir!("a", "b").unwrap();
///
/// ```
///
#[macro_export]
macro_rules! curr_dir {
    () => {
        std::env::current_dir()
    };
    ($($str:expr),*) => {{
        match std::env::current_dir() {
            Ok(mut p) => {
                $(p.push($str);)*
                Ok(p)
            }
            Err(e) => Err(e),
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_curr_dir() -> Result<()> {
        let curr1 = curr_dir!()?;
        let curr2 = curr_dir!("a", "b");
        println!("curr1: {:?}, curr2: {:?}", curr1, curr2);
        assert_eq!(curr1, std::env::current_dir().unwrap());
        Ok(())
    }
}
