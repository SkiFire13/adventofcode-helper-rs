pub trait ArrayExt<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
}

impl<T, const N: usize> ArrayExt<T> for [T; N] {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let array = [(); N].map(|_| iter.next().expect("Iterator didn't yield enough items"));
        assert!(iter.next().is_none(), "Iterator yielded too many items");
        array
    }
}

impl<T, const N: usize, const M: usize> ArrayExt<T> for [[T; N]; M] {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let array = [(); M].map(|_| <[T; N]>::from_iter(iter.by_ref().take(N)));
        assert!(iter.next().is_none(), "Iterator yielded too many items");
        array
    }
}
