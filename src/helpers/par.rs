use std::ops::{Range, RangeFrom, RangeInclusive};

use rayon::prelude::*;

pub trait ParFindChunkedExt {
    fn par_find_chunked<F>(self, chunk_size: usize, f: F) -> Option<usize>
    where
        F: Fn(usize) -> bool + Sync;
}

impl ParFindChunkedExt for Range<usize> {
    fn par_find_chunked<F>(self, chunk_size: usize, f: F) -> Option<usize>
    where
        F: Fn(usize) -> bool + Sync,
    {
        let mut idx = self.start;

        while idx < self.end {
            let end = std::cmp::min(idx + chunk_size, self.end);

            let chunk_result = (idx..end)
                .into_par_iter()
                .with_max_len(chunk_size / 8)
                .find_first(|&i| f(i));

            match chunk_result {
                Some(offset) => return Some(offset),
                None => idx += chunk_size,
            }
        }

        None
    }
}

impl ParFindChunkedExt for RangeInclusive<usize> {
    fn par_find_chunked<F>(self, chunk_size: usize, f: F) -> Option<usize>
    where
        F: Fn(usize) -> bool + Sync,
    {
        (*self.start()..self.end().checked_add(1).expect("overflow"))
            .par_find_chunked(chunk_size, f)
    }
}

impl ParFindChunkedExt for RangeFrom<usize> {
    fn par_find_chunked<F>(self, chunk_size: usize, f: F) -> Option<usize>
    where
        F: Fn(usize) -> bool + Sync,
    {
        (self.start..usize::MAX).par_find_chunked(chunk_size, f)
    }
}
