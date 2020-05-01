use nonempty::NonEmpty;

pub trait NonEmptyExt<T> {
    fn try_from<I: IntoIterator<Item = T>>(it: I) -> Option<NonEmpty<T>>;
}

impl<T> NonEmptyExt<T> for NonEmpty<T> {
    fn try_from<I: IntoIterator<Item = T>>(it: I) -> Option<Self> {
        let mut it = it.into_iter();
        Some(Self::from((it.nth(0)?, it.collect())))
    }
}
