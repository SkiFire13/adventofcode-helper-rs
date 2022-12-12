use std::ops::ControlFlow;

pub trait IteratorExt: Iterator {
    fn take_while_inclusive<F>(self, f: F) -> TakeWhileInclusive<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        TakeWhileInclusive {
            iter: Some(self),
            f,
        }
    }
}

impl<I: Iterator> IteratorExt for I {}

pub struct TakeWhileInclusive<I, F> {
    iter: Option<I>,
    f: F,
}

impl<I: Iterator, F: FnMut(&I::Item) -> bool> Iterator for TakeWhileInclusive<I, F> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.as_mut()?.next()?;
        if !(self.f)(&item) {
            self.iter = None;
        }
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Some(iter) = &self.iter else { return (0, Some(0)) };
        let (lo, hi) = iter.size_hint();
        (std::cmp::min(lo, 1), hi)
    }

    fn fold<B, G>(mut self, init: B, mut f: G) -> B
    where
        Self: Sized,
        G: FnMut(B, Self::Item) -> B,
    {
        let Some(mut iter) = self.iter else { return init };
        let acc = iter.try_fold(init, |acc, item| {
            let take = (self.f)(&item);
            let acc = f(acc, item);
            match take {
                true => ControlFlow::Continue(acc),
                false => ControlFlow::Break(acc),
            }
        });
        match acc {
            ControlFlow::Continue(acc) => acc,
            ControlFlow::Break(acc) => acc,
        }
    }
}
