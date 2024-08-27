#[extend::ext(name = OptionExt)]
pub impl<T> Option<T> {
    fn try_map<U, E, F: FnOnce(T) -> Result<U, E>>(self, f: F) -> Result<Option<U>, E> {
        self.map(f).transpose()
    }
}
