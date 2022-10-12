pub trait ArrayExt<T, const N: usize> {
    fn map_idx<R, F: FnMut(usize, T) -> R>(self, f: F) -> [R; N];
}

impl<T, const N: usize> ArrayExt<T, N> for [T; N] {
    fn map_idx<R, F: FnMut(usize, T) -> R>(self, mut f: F) -> [R; N] {
        let mut idx = 0;
        self.map(|value| {
            let curr_idx = idx;
            idx += 1;
            f(curr_idx, value)
        })
    }
}

pub trait ArrayFromIterExt<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
}

impl<T, const N: usize> ArrayFromIterExt<T> for [T; N] {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let array = [(); N].map(|_| iter.next().expect("Iterator didn't yield enough items"));
        assert!(iter.next().is_none(), "Iterator yielded too many items");
        array
    }
}

impl<T, const N: usize, const M: usize> ArrayFromIterExt<T> for [[T; N]; M] {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let array = [(); M].map(|_| <[T; N]>::from_iter(iter.by_ref().take(N)));
        assert!(iter.next().is_none(), "Iterator yielded too many items");
        array
    }
}
