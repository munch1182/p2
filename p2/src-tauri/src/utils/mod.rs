pub trait OptOrExt<T> {
    fn if_none(self, new_value: T) -> T;
}

impl<T> OptOrExt<T> for Option<T> {
    /**
     * If the option is None, return the new value, otherwise return the value of the option
     */
    fn if_none(self, new_value: T) -> T {
        match self {
            Some(a) => a,
            None => new_value,
        }
    }
}
