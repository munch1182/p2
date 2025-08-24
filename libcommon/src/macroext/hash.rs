///
/// 简化hash求值写法
///
/// # example
/// ```
/// use libcommon::hash;
///
/// hash!("a", "b", "c");
/// ```
///
#[macro_export]
macro_rules! hash {
    ($($str:expr),*) => {{
        use std::hash::{DefaultHasher, Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        $($str.hash(&mut hasher);)*
        hasher.finish()
    }};
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[test]
    fn test_hash() -> std::io::Result<()> {
        let a = "aaaaaaaaaa";
        let b = "bbbbbbbbbb";
        let c = "cccccccccc";
        let mut hasher = DefaultHasher::new();
        a.hash(&mut hasher);
        b.hash(&mut hasher);
        c.hash(&mut hasher);
        assert_eq!(hash!(a, b, c), hasher.finish());
        Ok(())
    }
}
